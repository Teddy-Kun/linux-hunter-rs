// Temporarily allow dead code so I keep my sanity
#![allow(dead_code, unused_variables)]

use std::{ffi::c_void, fs};

use sscanf::scanf;

use crate::memory::{pattern::Pattern, region::MemoryRegion};
use nix::{
	libc::{iovec, process_vm_readv},
	unistd::Pid,
};

#[derive(Debug)]
pub struct Browser {
	pbyte: u8,
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
		region: &mut Vec<MemoryRegion>,
		alloc_mem: bool,
	) -> Result<(), Box<dyn std::error::Error>> {
		*region = Vec::new();

		let maps_path = String::from("/proc/") + pid.to_string().as_str() + "/maps";
		let maps = fs::read_to_string(&maps_path)?;

		for line in maps.lines() {
			match scanf!(line, "{u64:x}-{u64:x} {&str} {u64:x} {&str} {i64}") {
				Err(_) => continue,
				Ok((begin, end, permissions, _offset, _device, inode)) => {
					if inode != 0 || !permissions.starts_with("r") {
						continue;
					}

					let reg = MemoryRegion::new(begin, end, line, alloc_mem)?;
					region.push(reg);
				}
			};
		}

		Ok(())
	}

	fn snap_pid(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		Browser::snap_mem_regions(self.pid, &mut self.all_regions, true)?;

		for region in &mut self.all_regions {
			let size = region.end - region.beg;

			let local: iovec = iovec {
				iov_base: region.data.as_mut_ptr() as *mut c_void,
				iov_len: size as usize,
			};
			let remote: iovec = iovec {
				iov_base: region.beg as *mut c_void,
				iov_len: size as usize,
			};

			let e = unsafe { process_vm_readv(self.pid.as_raw(), &local, 1, &remote, 1, 0) };
			if e < 0 {
				eprintln!(
					"Region: {} Error with process_vm_readv ({})",
					region.debug_info, e
				);
			}

			if e as u64 != size {
				eprintln!(
					"Region: {} Read {} bytes instead of {}",
					region.debug_info, e, size
				);
				region.data_sz = e;
			}
			region.dirty = false;
		}

		Ok(())
	}

	fn update_region(&self) {
		todo!("update_region");
	}

	fn find_once(
		&self,
		pattern: &Pattern,
		buf: &mut [u8],
		sz: usize,
		hint: u8,
		debug_all: bool,
	) -> isize {
		todo!("find_once");
	}

	fn verify_regions(&self) {
		todo!("verify_regions");
	}

	fn refresh_region(&self, region: &MemoryRegion) {
		todo!("refresh_region");
	}

	fn find_first(&self, pattern: &Pattern, debug_all: bool, start_addr: usize) -> isize {
		todo!("find_once");
	}

	fn direct_mem_read(&self) -> bool {
		todo!("direct_mem_read");
	}

	// Public functions
	pub fn new(pid: Pid, dirty_opt: bool, lazy_alloc: bool, direct_mem: bool) -> Self {
		todo!("new");
	}

	pub fn snap(&self) {
		todo!("snap");
	}

	pub fn update(&self) {
		todo!("update");
	}

	pub fn clear(&self) {
		todo!("clear");
	}

	pub fn store(&self, dir_name: &str) {
		todo!("store");
	}

	pub fn load(&self, dir_name: &str) {
		todo!("load");
	}

	pub fn find_patterns(&self, begin: &[Pattern], end: &[Pattern], debug_all: bool) {
		todo!("find_patterns");
	}

	// Templates?
}
