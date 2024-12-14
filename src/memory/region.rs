use std::{
	fs::File,
	io::{IoSliceMut, Write},
};

use nix::{
	sys::uio::{process_vm_readv, RemoteIoVec},
	unistd::Pid,
};

use crate::err::Error;

#[derive(Debug)]
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

	pub fn clear(&mut self) {
		self.data_sz = 0;
		self.dirty = true;
		self.data.clear();
	}

	fn dump_mem(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		let path = path.to_string() + "/" + self.debug_name.as_str() + ".txt";
		let contents = self
			.data
			.iter()
			.map(|byte| format!("{:02x} ", byte))
			.collect::<Vec<String>>()
			.join(" ");

		let mut file = File::create(path)?;

		file.write_all(contents.as_bytes())?;
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
