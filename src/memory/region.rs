use std::rc::Rc;

#[derive(Debug)]
pub struct MemoryRegion {
	pub begin: usize,
	pub end: usize,
	pub debug_info: String,
	pub data: Rc<Vec<u8>>,
	pub data_sz: isize,
	pub dirty: bool,
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_info: &str, alloc: bool) -> Self {
		let vec = match alloc {
			true => Vec::with_capacity((end - begin) as usize),
			false => Vec::new(),
		};

		MemoryRegion {
			begin,
			end,
			debug_info: debug_info.to_string(),
			data: Rc::new(vec),
			data_sz: (end - begin) as isize,
			dirty: true,
		}
	}
}
