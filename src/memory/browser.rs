use crate::err::Error;
use crate::memory::region::MemoryRegion;

use nix::{
	libc::{iovec, process_vm_readv},
	unistd::Pid,
};
use sscanf::scanf;
use std::rc::Rc;
use std::{ffi::c_void, fs};

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

			let local: iovec = iovec {
				iov_base: region.data.as_ptr() as *mut c_void,
				iov_len: size as usize,
			};
			let remote: iovec = iovec {
				iov_base: region.begin as *mut c_void,
				iov_len: size as usize,
			};

			let read_size =
				unsafe { process_vm_readv(self.pid.as_raw(), &local, 1, &remote, 1, 0) };
			if read_size < 0 {
				eprintln!(
					"Region: {} Error with process_vm_readv ({})",
					region.debug_info, read_size
				);
			}

			if read_size as usize != size {
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

	fn refresh_region(&self, region: &mut MemoryRegion) {
		// usually this code is only going to be
		// invoked when lazy_alloc is set - and
		// of course data is 'dirty'
		if region.data.len() == 0 {
			region.data = Rc::new(Vec::with_capacity(region.end - region.begin));
			region.dirty = true;
		}

		if self.dirty_opt && !region.dirty {
			return;
		}

		let size = region.end - region.begin;
		let local: iovec = iovec {
			iov_base: region.data.as_ptr() as *mut c_void,
			iov_len: size as usize,
		};
		let remote: iovec = iovec {
			iov_base: region.begin as *mut c_void,
			iov_len: size as usize,
		};
		let read_size = unsafe { process_vm_readv(self.pid.as_raw(), &local, 1, &remote, 1, 0) };

		if read_size < 0 {
			region.data_sz = -1;
			eprintln!(
				"Region: {} Error with process_vm_readv ({})",
				region.debug_info, read_size
			);
			return;
		}
		if size != read_size as usize {
			region.data_sz = read_size;
		}

		region.dirty = false;
	}

	fn direct_mem_read(
		&self,
		addr: usize,
		size: usize,
	) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		let buf: Vec<u8> = Vec::with_capacity(size);

		let local: iovec = iovec {
			iov_base: buf.as_ptr() as *mut c_void,
			iov_len: size as usize,
		};
		let remote: iovec = iovec {
			iov_base: addr as *mut c_void,
			iov_len: size as usize,
		};

		let read_size = unsafe { process_vm_readv(self.pid.as_raw(), &local, 1, &remote, 1, 0) };

		if read_size < 0 {
			return Err(Error::new("Error reading MH:W memory").into());
		}
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
		todo!("store");
	}

	pub fn load(&self, dir_name: &str) {
		todo!("load");
	}

	pub fn find_patterns(
		&self,
		patterns: &[fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>],
		debug_all: bool,
	) -> Vec<Box<Vec<u8>>> {
		let mut result = Vec::new();

		println!("all_regions len: {}", self.all_regions.len());

		for pattern in patterns {
			for region in &self.all_regions {
				if debug_all {
					println!("Region: {}\n{:?}\n\n", region.debug_info, region.data.len());
				}

				let res = pattern(&region.data);
				println!("res {:?}", res);

				if let Ok(res) = res {
					result.push(Box::new(res));
				} else {
					println!("No match");
				}
			}
		}

		result
	}

	// Templates?
}
