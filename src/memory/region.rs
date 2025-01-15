use nix::{
	sys::uio::{process_vm_readv, RemoteIoVec},
	unistd::Pid,
};
use std::{
	fs::File,
	io::{IoSliceMut, Write},
};

use crate::err::Error;

#[derive(Debug, Clone)]
pub struct MemoryRegion {
	begin: usize,
	pub debug_name: String,
	pub debug_info: String,
	pub data: Vec<u8>,
	pub data_sz: usize,
	pub dirty: bool,
	pub from_vec: bool,
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_name: &str, debug_info: &str) -> Self {
		MemoryRegion {
			begin,
			debug_name: debug_name.to_string(),
			debug_info: debug_info.to_string(),
			data: vec![],
			data_sz: (end - begin),
			dirty: true,
			from_vec: false,
		}
	}

	pub fn from_vec(data: Vec<u8>, debug_name: &str, debug_info: &str) -> Self {
		let data_sz = data.len();

		MemoryRegion {
			begin: 0,
			debug_name: debug_name.to_string(),
			debug_info: debug_info.to_string(),
			data,
			data_sz,
			dirty: false,
			from_vec: true,
		}
	}

	pub fn get_begin(&self) -> usize {
		self.begin
	}

	pub fn clear(&mut self) {
		self.data_sz = 0;
		self.dirty = true;
		self.data.clear();
	}

	fn dump_mem(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let path = path.to_string() + "/" + self.debug_name.as_str() + ".bin";
		let mut file = File::create(path)?;

		file.write_all(&self.data)?;
		file.flush()?;

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

		match read_memory(pid, self.begin, self.data_sz) {
			Ok(data) => self.data = data,
			Err(e) => {
				self.dirty = true;
				return Err(e);
			}
		}

		if let Some(path) = dump_mem {
			if let Err(e) = self.dump_mem(&path) {
				eprintln!("Failed to dump memory: {}", e);
			}
		}

		Ok(())
	}
}

pub fn read_memory(
	pid: Pid,
	start: usize,
	length: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let mut buf = vec![0u8; length];

	let local = IoSliceMut::new(&mut buf);
	let remote = RemoteIoVec {
		base: start,
		len: length,
	};

	let read_size = process_vm_readv(pid, &mut [local], &[remote])?;

	if read_size != length {
		return Err(Error::new(&format!("Read {} bytes instead of {}", read_size, length)).into());
	}

	Ok(buf)
}

pub fn verify_regions(regions: &[MemoryRegion]) -> Result<(), Box<dyn std::error::Error>> {
	let mut prev_beg = regions[0].begin;
	for region in regions.iter().skip(1) {
		if region.begin < prev_beg {
			return Err(Error::new("Invalid region sequence - order").into());
		}

		prev_beg = region.begin;
	}

	Ok(())
}
