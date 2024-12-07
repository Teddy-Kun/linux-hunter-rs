use crate::memory::{pattern::Pattern, region::MemoryRegion};
use nix::unistd::Pid;

#[derive(Debug)]
pub struct Browser<'a> {
	pbyte: u8,
	pid: Pid,

	dirty_opt: bool,
	lazy_alloc: bool,
	direct_mem: bool,

	all_regions: Vec<&'a MemoryRegion>,
}

impl<'a> Browser<'a> {
	// internal functions
	fn snap_mem_regions(&self, region: &MemoryRegion, alloc_mem: bool) {
		todo!("snap_mem_regions");
	}

	fn snap_pid(&self) {
		todo!("snap_pid");
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
