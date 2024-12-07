#[derive(Debug)]
pub struct MemoryRegion {
	pub begin: usize,
	pub end: usize,
	pub debug_info: String,
	pub data: Vec<u8>,
	pub data_sz: isize,
	pub dirty: bool,
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_info: &str, alloc: bool) -> Self {
		if alloc {
			return MemoryRegion {
				begin,
				end,
				debug_info: debug_info.to_string(),
				data: Vec::with_capacity((end - begin) as usize),
				data_sz: (end - begin) as isize,
				dirty: true,
			};
		}

		MemoryRegion {
			begin,
			end,
			debug_info: debug_info.to_string(),
			data: Vec::new(),
			data_sz: (end - begin) as isize,
			dirty: true,
		}
	}
}
