use nix::unistd::Pid;

use crate::mhw::offsets;

use super::{
	pattern::{PatternGetter, PatternType},
	region::read_memory,
};

#[derive(Debug)]
struct SessionInfo {
	pub session_id: String,
	pub hostname: String,
	pub is_mission: bool,
	pub is_expedition: bool,
}

fn get_session_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<SessionInfo, Box<dyn std::error::Error>> {
	// TODO: only copy memory to a buffer with 1 syscall, then read from it, instead of using 4 syscalls

	let pattern = &patterns[PatternType::LobbyStatus as usize];

	let mut info = SessionInfo {
		session_id: String::new(),
		hostname: String::new(),
		is_mission: true,
		is_expedition: false,
	};

	if pattern.index.is_none() || pattern.offset.is_none() {
		return Ok(info);
	}

	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let pointer: usize =
		u32::from_ne_bytes(read_memory(pid, start, 4)?.try_into().unwrap()) as usize;

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_ID,
		offsets::ID_LENGTH,
	)?;
	info.session_id = String::from_utf8(mem)?;

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_HOST_NAME,
		offsets::PLAYER_NAME_LENGTH,
	)?;
	info.hostname = String::from_utf8(mem)?;

	// TODO: not working, find out why and fix this
	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let pointer: usize =
		u64::from_ne_bytes(read_memory(pid, start, 8)?.try_into().unwrap()) as usize;
	let mem = read_memory(pid, pointer + start + offsets::MISSION_STATUS_OFFSET, 1)?;
	info.is_mission = mem[0] != 0;

	// TODO: not working, find out why and fix this
	let mem = read_memory(pid, pointer + start + offsets::EXPEDITION_STATUS_OFFSET, 1)?;
	info.is_expedition = mem[0] != 0;

	Ok(info)
}

fn get_damage(pid: Pid, patterns: &[PatternGetter]) -> Result<(), Box<dyn std::error::Error>> {
	Err("not implemented".into())
}

fn get_monster_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<(), Box<dyn std::error::Error>> {
	let pattern = &patterns[PatternType::Monsters as usize];

	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let mem = read_memory(pid, start, 256)?;

	println!("{:02X?}", mem);

	Err("not implemented".into())
}

pub fn update_all(pid: Pid, patterns: &[PatternGetter]) -> Result<(), Box<dyn std::error::Error>> {
	let lobby_data = get_session_data(pid, patterns)?;
	println!("session info: {:#?}", lobby_data);

	if lobby_data.is_expedition || lobby_data.is_mission {
		get_damage(pid, patterns).unwrap()
	}

	get_monster_data(pid, patterns).unwrap();

	panic!("not implemented");

	Ok(())
}
