use crate::err::Error;

#[derive(Debug, Clone)]
pub struct MemoryRegion {
	pub beg: u64,
	pub end: u64,
	pub debug_info: String,
	pub data: Vec<u8>,
	pub data_sz: isize,
	pub dirty: bool,
}

impl MemoryRegion {
	pub fn new(begin: u64, end: u64, debug_info: &str, alloc: bool) -> Result<MemoryRegion, Error> {
		todo!("new");
	}
}
