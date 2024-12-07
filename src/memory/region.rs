use crate::err::Error;

#[derive(Debug)]
pub struct MemoryRegion {
	beg: u64,
	end: u64,
	debug_info: String,
	data: Vec<u8>,
	data_sz: isize,
	dirty: bool,
}

impl MemoryRegion {
	fn new(begin: u64, end: u64, debug_info: &str, alloc: bool) -> Result<MemoryRegion, Error> {
		todo!("new");
	}
}
