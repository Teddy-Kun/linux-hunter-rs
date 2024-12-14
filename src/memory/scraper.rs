use std::fs;

use nix::unistd::Pid;
use sscanf::scanf;

use super::region::MemoryRegion;

pub fn get_memory_regions(pid: Pid) -> Result<Vec<MemoryRegion>, Box<dyn std::error::Error>> {
	let maps_path = String::from("/proc/") + pid.to_string().as_str() + "/maps";
	let maps = fs::read_to_string(&maps_path)?;

	let mut regions: Vec<MemoryRegion> = Vec::new();

	for line in maps.lines() {
		match scanf!(
			line,
			"{usize:x}-{usize:x} {&str} {u64:x} {&str} {i64}                            {&str}"
		) {
			Err(_) => continue,
			Ok((begin, end, permissions, _offset, _device, inode, _path)) => {
				if inode == 0 || !permissions.starts_with("r") {
					continue;
				}

				let reg = MemoryRegion::new(begin, end, itoa::Buffer::new().format(begin), line);
				regions.push(reg);
			}
		};
	}

	Ok(regions)
}
