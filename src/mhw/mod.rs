pub mod data;
pub mod offsets;

use crate::err::Error;
use nix::unistd::Pid;
use std::{
	fs::{self, read_dir},
	io::Read,
};

const MHW_EXE: &str = "\\MonsterHunterWorld.exe";

pub fn find_mhw_pid() -> Result<Pid, Box<dyn std::error::Error>> {
	// read "/proc"
	let proc = read_dir("/proc")?;

	// iterate over contents
	for entry in proc {
		let path = entry?.path();

		// only do smth if its a dir
		if path.is_dir() && path.file_name().is_some() {
			let name = match path.file_name().unwrap().to_str() {
				Some(name) => name,
				None => continue,
			};

			// check if dirs name is a pid
			if !name.chars().all(|c| c.is_ascii_digit()) {
				continue;
			}

			// try to open "/proc/pid/cmdline"
			let mut file = match fs::File::open(format!("{}/cmdline", path.to_string_lossy())) {
				Ok(file) => file,
				Err(_) => continue,
			};

			// try to read the contents of cmdline
			let mut contents = String::new();
			if file.read_to_string(&mut contents).is_err() {
				continue;
			}

			if contents.contains(MHW_EXE) {
				// TODO: implement the simple path logic
				// is that even necessary?
				return Ok(Pid::from_raw(name.parse::<i32>()?));
			}
		}
	}

	Err(Error::new("Can't find MH:W pid").into())
}
