use nix::{
	sys::uio::{process_vm_readv, RemoteIoVec},
	unistd::Pid,
};
use std::{
	fs::File,
	io::{IoSliceMut, Write},
};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct MemoryRegion {
	begin: usize,
	end: usize,
	pub debug_name: String,
	pub debug_info: String,
	pub data: Option<Box<[u8]>>,
	pub from_vec: bool,
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_name: &str, debug_info: &str) -> Self {
		MemoryRegion {
			begin,
			end,
			debug_name: debug_name.to_string(),
			debug_info: debug_info.to_string(),
			data: None,
			from_vec: false,
		}
	}

	pub fn from_vec(data: Vec<u8>, debug_name: &str, debug_info: &str) -> Self {
		MemoryRegion {
			begin: 0,
			end: data.len(),
			debug_name: debug_name.to_string(),
			debug_info: debug_info.to_string(),
			data: Some(data.into_boxed_slice()),
			from_vec: true,
		}
	}

	pub fn get_begin(&self) -> usize {
		self.begin
	}

	fn dump_mem(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		if let Some(data) = &self.data {
			let path = path.to_string() + "/" + self.debug_name.as_str() + ".bin";
			let mut file = File::create(path)?;

			file.write_all(data)?;
			file.flush()?;
		}

		Ok(())
	}

	pub fn fill_data(
		&mut self,
		pid: Pid,
		dump_mem: Option<String>,
	) -> Result<(), Box<dyn std::error::Error>> {
		if self.from_vec {
			return Ok(());
		}

		match read_memory(pid, self.begin, self.end - self.begin) {
			Ok(data) => self.data = Some(data),
			Err(e) => {
				return Err(e);
			}
		}

		if let Some(path) = dump_mem {
			if let Err(e) = self.dump_mem(&path) {
				warn!("Failed to dump memory: {}", e);
			}
		}

		Ok(())
	}
}

pub fn read_memory(
	pid: Pid,
	start: usize,
	length: usize,
) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
	let mut buf = vec![0u8; length];

	let local = IoSliceMut::new(&mut buf);
	let remote = RemoteIoVec {
		base: start,
		len: length,
	};

	let read_size = process_vm_readv(pid, &mut [local], &[remote])?;

	if read_size != length {
		return Err(format!("Read {} bytes instead of {}", read_size, length).into());
	}

	Ok(buf.into_boxed_slice())
}

#[macro_export]
macro_rules! read_mem_to_type {
	($pid:expr, $start:expr, $t:ty) => {{
		let ptr_loc: Box<[u8]> = read_memory($pid, $start, 4)?;

		use tracing::debug;
		debug!("{:02X?}", ptr_loc);

		let ptr_ptr: *mut [u8; 4] = Box::into_raw(ptr_loc) as *mut [u8; 4];
		// Create a slice from the pointer and interpret it as type T
		// this is always safe, since we always get data of the right size
		let sliced_slice: $t = unsafe { std::ptr::read(ptr_ptr as *const $t) };
		sliced_slice
	}};
}

pub fn verify_regions(regions: &[MemoryRegion]) -> Result<(), Box<dyn std::error::Error>> {
	let mut prev_beg = regions[0].begin;
	for region in regions.iter().skip(1) {
		if region.begin < prev_beg {
			return Err("Invalid region sequence - order".into());
		}

		prev_beg = region.begin;
	}

	Ok(())
}
