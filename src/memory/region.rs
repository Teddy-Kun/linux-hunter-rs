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
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_name: &str, debug_info: &str) -> Self {
		MemoryRegion {
			begin,
			debug_name: debug_name.to_string(),
			debug_info: debug_info.to_string(),
			data: vec![0u8; end - begin],
			data_sz: (end - begin),
			dirty: true,
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
		let local = IoSliceMut::new(&mut self.data);
		let remote = RemoteIoVec {
			base: self.begin,
			len: self.data_sz,
		};

		let read_size = process_vm_readv(pid, &mut [local], &[remote])?;

		if read_size != self.data_sz {
			self.dirty = true;

			return Err(Error::new(&format!(
				"Read {} bytes instead of {}",
				read_size, self.data_sz
			))
			.into());
		}

		self.dirty = false;

		if let Some(path) = dump_mem {
			if let Err(e) = self.dump_mem(&path) {
				eprintln!("Failed to dump memory: {}", e);
			}
		}

		Ok(())
	}
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
