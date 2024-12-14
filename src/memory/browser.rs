// incomplete reimplementation of the memory browser of the og repo
// tried to port it to rust, but decided to do a different architecture
// its still here in case I need to look smth up

use crate::err::Error;
use crate::memory::region::MemoryRegion;

use nix::{
	sys::uio::{self, RemoteIoVec},
	unistd::Pid,
};
use sscanf::scanf;
use std::fs;
use std::{io::IoSliceMut, rc::Rc};

#[derive(Debug)]
pub struct Browser {
	// pbyte: u8,
	pid: Pid,

	dirty_opt: bool,
	lazy_alloc: bool,
	direct_mem: bool,

	all_regions: Vec<MemoryRegion>,
}

impl Browser {
	// internal functions
	fn snap_mem_regions(
		pid: Pid,
		regions: &mut Vec<MemoryRegion>,
		alloc_mem: bool,
	) -> Result<(), Box<dyn std::error::Error>> {
		regions.clear();

		let maps_path = String::from("/proc/") + pid.to_string().as_str() + "/maps";
		let maps = fs::read_to_string(&maps_path)?;

		for line in maps.lines() {
			match scanf!(
				line,
				"{usize:x}-{usize:x} {&str} {u64:x} {&str} {i64}                            {&str}"
			) {
				Err(_) => continue,
				Ok((begin, end, permissions, _offset, _device, inode, _path)) => {
					if inode == 0 || !permissions.starts_with("r") {
						continue;
					}

					let reg = MemoryRegion::new(begin, end, line, alloc_mem);
					regions.push(reg);
				}
			};
		}

		Ok(())
	}

	fn snap_pid(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		Browser::snap_mem_regions(self.pid, &mut self.all_regions, true)?;

		for region in &mut self.all_regions {
			let size = region.end - region.begin;

			let data = Rc::get_mut(&mut region.data);
			let local = IoSliceMut::new(data.unwrap());
			let remote = RemoteIoVec {
				base: region.begin,
				len: size as usize,
			};

			let read_size = uio::process_vm_readv(self.pid, &mut [local], &[remote])?;

			println!("DATA LEN: {}", region.data.len());

			if read_size != size {
				eprintln!(
					"Region: {} Read {} bytes instead of {}",
					region.debug_info, read_size, size
				);
				region.data_sz = read_size;
			}
			region.dirty = false;
		}

		Ok(())
	}

	fn update_regions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let mut new_regions: Vec<MemoryRegion> = Vec::with_capacity(self.all_regions.len() * 2);
		Browser::snap_mem_regions(self.pid, &mut new_regions, false)?;

		// Merge the current into the new (if possible) sorted
		let mut hint = 0;
		for region in &mut new_regions {
			for idx in hint..self.all_regions.len() {
				if self.all_regions[idx].begin == region.begin
					&& self.all_regions[idx].end == region.end
				{
					region.data = self.all_regions[idx].data.clone();

					hint += 1;
					break;
				}
			}

			if !self.lazy_alloc && region.data.len() == 0 {
				region.data = Rc::new(Vec::with_capacity(region.end - region.begin));
				region.dirty = true;
			}
		}

		Ok(())
	}

	fn verify_regions(&self) -> Result<(), Box<dyn std::error::Error>> {
		let mut prev_beg: usize = 0;
		let mut prev_end: usize = 0;
		let mut first = true;

		for region in &self.all_regions {
			if first {
				prev_beg = region.begin;
				prev_end = region.end;

				first = false;
				continue;
			}

			if region.begin < prev_beg {
				return Err(Error::new("Invalid region sequence - order").into());
			}
			if region.begin < prev_end {
				return Err(Error::new("Invalid region sequence - overlap").into());
			}

			prev_beg = region.begin;
			prev_end = region.end;
		}

		Ok(())
	}

	fn refresh_region(&self, region: &mut MemoryRegion) -> Result<(), Box<dyn std::error::Error>> {
		// usually this code is only going to be
		// invoked when lazy_alloc is set - and
		// of course data is 'dirty'
		if region.data.len() == 0 {
			region.data = Rc::new(Vec::with_capacity(region.end - region.begin));
			region.dirty = true;
		}

		if self.dirty_opt && !region.dirty {
			return Ok(());
		}

		let size = region.end - region.begin;

		let data = Rc::get_mut(&mut region.data);
		let local = IoSliceMut::new(data.unwrap());
		let remote = RemoteIoVec {
			base: region.begin,
			len: size as usize,
		};

		let read_size = uio::process_vm_readv(self.pid, &mut [local], &[remote])?;

		if size != read_size as usize {
			region.data_sz = read_size as usize;
		}

		region.dirty = false;

		Ok(())
	}

	fn direct_mem_read(
		&self,
		addr: usize,
		size: usize,
	) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		let mut buf: Vec<u8> = Vec::with_capacity(size);

		let local = IoSliceMut::new(&mut buf);
		let remote = RemoteIoVec {
			base: addr,
			len: size as usize,
		};

		let read_size = uio::process_vm_readv(self.pid, &mut [local], &[remote])?;

		if size != read_size as usize {
			return Err(Error::new("partial memory read").into());
		}

		Ok(buf)
	}

	// Public functions
	pub fn new(pid: Pid, dirty_opt: bool, lazy_alloc: bool, direct_mem: bool) -> Self {
		Self {
			pid,
			dirty_opt,
			lazy_alloc,
			direct_mem,
			all_regions: Vec::new(),
			// pbyte: 0,
		}
	}

	pub fn snap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		self.snap_pid()?;
		self.verify_regions()
	}

	pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		// if we're in direct memory mode
		// do not update
		if self.direct_mem {
			return Ok(());
		}

		// need to check memory layout
		// usually shouldn't change much
		// but it _does_ sometime
		self.update_regions()?;

		// don't execute the code
		// in case we haven't enabled
		// dirty_opt_
		if !self.dirty_opt {
			return Ok(());
		}

		for region in &mut self.all_regions {
			region.dirty = true;
		}

		Ok(())
	}

	pub fn clear(&mut self) {
		self.all_regions.clear();
	}

	pub fn store(&self, dir_name: &str) {
		todo!("store {}", dir_name);
	}

	pub fn load(&self, dir_name: &str) {
		todo!("load {}", dir_name);
	}

	pub fn find_patterns(
		&self,
		patterns: &[fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>],
		debug_all: bool,
	) -> Vec<Box<Vec<u8>>> {
		let mut result = Vec::new();

		println!("all_regions len: {}", self.all_regions.len());

		for pattern in patterns {
			let now = std::time::Instant::now();
			for region in &self.all_regions {
				if region.data.len() > 0 {
					println!("FOUND A LONG ONE")
				}

				if debug_all {
					println!("Region: {}\n{:?}\n\n", region.debug_info, region.data.len());
				}

				let res = pattern(&region.data);
				if let Ok(res) = res {
					result.push(Box::new(res));
				} else {
					println!("No match");
				}
			}

			println!("Took: {} ns", now.elapsed().as_nanos());
		}

		result
	}

	// Templates?
}
