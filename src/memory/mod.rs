pub mod pattern;
pub mod region;
pub mod update;

use nix::unistd::Pid;
use region::MemoryRegion;
use sscanf::scanf;
use std::fs;

pub fn get_memory_regions(
	pid: Pid,
	debug: bool,
	dump_loc: &Option<String>,
) -> Result<Vec<MemoryRegion>, Box<dyn std::error::Error>> {
	// dont load the games memory if we are supposed to load from a dump
	// usefull for debugging
	if let Some(path) = dump_loc {
		return load_dump(path);
	}

	let maps_path = String::from("/proc/") + pid.to_string().as_str() + "/maps";
	let maps = fs::read_to_string(&maps_path)?;

	let mut regions: Vec<MemoryRegion> = Vec::new();

	if debug {
		println!("lines: {}", maps.lines().count());
	}

	for line in maps.lines() {
		match scanf!(
			line,
			"{usize:x}-{usize:x} {&str} {usize:x} {&str} {isize}{&str}"
		) {
			Err(_) => continue,
			Ok((begin, end, permissions, _offset, _device, inode, _path)) => {
				if inode != 0 || !permissions.starts_with("r") {
					continue;
				}

				let reg = MemoryRegion::new(begin, end, &format!("{:x}", begin), line);
				regions.push(reg);
			}
		};
	}

	if debug {
		println!("regions: {}", regions.len());
	}

	Ok(regions)
}

fn load_dump(path: &str) -> Result<Vec<MemoryRegion>, Box<dyn std::error::Error>> {
	let dir = fs::read_dir(path)?;

	let mut res = Vec::new();

	for entry in dir {
		let entry = entry?;

		let data = fs::read(entry.path())?;

		let reg = MemoryRegion::from_vec(
			data,
			entry.file_name().to_str().unwrap(),
			entry.path().to_str().unwrap(),
		);

		res.push(reg);
	}

	Ok(res)
}
