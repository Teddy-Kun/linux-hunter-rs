use nix::unistd::Pid;

use crate::mhw::offsets;

use super::{
	pattern::{PatternGetter, PatternType},
	region::read_memory,
};

#[derive(Debug)]
struct SessionInfo {
	session_id: String,
	hostname: String,
	is_mission: bool,
	is_expedition: bool,
}

fn get_session_data(
	pid: Pid,
	pattern: &PatternGetter,
) -> Result<SessionInfo, Box<dyn std::error::Error>> {
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

fn get_monster_data(pid: Pid, pattern: &PatternGetter) -> Result<(), Box<dyn std::error::Error>> {
	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let mem = read_memory(pid, start, 256)?;

	println!("{:02X?}", mem);

	Err("not implemented".into())
}

pub fn update_all(pid: Pid, patterns: &[PatternGetter]) -> Result<(), Box<dyn std::error::Error>> {
	let lobby_pattern = &patterns[PatternType::LobbyStatus as usize];
	let lobby_data = get_session_data(pid, lobby_pattern)?;
	println!("session info: {:#?}", lobby_data);

	panic!("not implemented");

	Ok(())
}
