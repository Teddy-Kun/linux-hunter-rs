use std::fs;

use nix::unistd::Pid;
use region::MemoryRegion;
use sscanf::scanf;

pub mod pattern;
pub mod region;

pub fn get_memory_regions(
	pid: Pid,
	debug: bool,
) -> Result<Vec<MemoryRegion>, Box<dyn std::error::Error>> {
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
