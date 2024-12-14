use std::io::IoSliceMut;

use nix::{
	sys::uio::{process_vm_readv, RemoteIoVec},
	unistd::Pid,
};

use crate::err::Error;

#[derive(Debug)]
pub struct MemoryRegion {
	pub begin: usize,
	pub end: usize,
	pub debug_info: String,
	pub data: Vec<u8>,
	pub data_sz: usize,
	pub dirty: bool,
}

impl MemoryRegion {
	pub fn new(begin: usize, end: usize, debug_info: &str, alloc: bool) -> Self {
		let vec = match alloc {
			true => vec![0u8; end - begin],
			false => Vec::new(),
		};

		MemoryRegion {
			begin,
			end,
			debug_info: debug_info.to_string(),
			data: vec,
			data_sz: (end - begin),
			dirty: true,
		}
	}

	pub fn clear(&mut self) {
		self.data_sz = 0;
		self.dirty = true;
		self.data.clear();
	}

	pub fn fill_data(&mut self, pid: Pid) -> Result<(), Box<dyn std::error::Error>> {
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

		Ok(())
	}
}
