use std::str;

use nix::unistd::Pid;

use crate::{
	mhw::{
		data::{GameData, MonsterInfo, PlayerInfo, SessionInfo},
		offsets,
	},
	read_mem_to_type,
};

use super::{
	pattern::{PatternGetter, PatternType},
	region::read_memory,
};

fn get_session_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<SessionInfo, Box<dyn std::error::Error>> {
	// TODO: only copy memory to a buffer with 1 syscall, then read from it, instead of using 4 syscalls?

	let pattern = &patterns[PatternType::LobbyStatus as usize];

	let mut info = SessionInfo {
		session_id: Box::from(""),
		hostname: Box::from(""),
		is_mission: true,
		is_expedition: false,
	};

	if pattern.index.is_none() || pattern.offset.is_none() {
		return Ok(info);
	}

	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let pointer = read_mem_to_type!(pid, start, u32) as usize;

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_ID,
		offsets::ID_LENGTH,
	)?;
	// since the game uses UTF-8 this is safe
	info.session_id = unsafe { str::from_boxed_utf8_unchecked(mem) };

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_HOST_NAME,
		offsets::PLAYER_NAME_LENGTH,
	)?;
	// since the game uses UTF-8 this is safe
	info.hostname = unsafe { str::from_boxed_utf8_unchecked(mem) };

	// TODO: not working, find out why and fix this
	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let pointer = read_mem_to_type!(pid, start, u64) as usize;
	let mem = read_memory(pid, pointer + start + offsets::MISSION_STATUS_OFFSET, 1)?;
	info.is_mission = mem[0] != 0;

	// TODO: not working, find out why and fix this
	let mem = read_memory(pid, pointer + start + offsets::EXPEDITION_STATUS_OFFSET, 1)?;
	info.is_expedition = mem[0] != 0;

	Ok(info)
}

fn get_damage(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<Box<[PlayerInfo]>, Box<dyn std::error::Error>> {
	Err("not implemented".into())
}

fn get_monster_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<Box<[MonsterInfo]>, Box<dyn std::error::Error>> {
	let pattern = &patterns[PatternType::Monsters as usize];

	let start = pattern.index.unwrap() + pattern.offset.unwrap();
	let mem = read_memory(pid, start, 256)?;

	println!("{:02X?}", mem);

	Err("not implemented".into())
}

fn get_single_monster() -> Result<MonsterInfo, Box<dyn std::error::Error>> {
	Err("not implemented".into())
}

pub fn update_all(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<GameData, Box<dyn std::error::Error>> {
	let mut data = GameData::new(get_session_data(pid, patterns)?);
	println!("session info: {:#?}", data.session);

	if data.session.is_expedition || data.session.is_mission {
		match get_damage(pid, patterns) {
			Ok(damage) => data.players = damage,
			Err(e) => println!("failed to get player damage: {}", e),
		}

		match get_monster_data(pid, patterns) {
			Ok(monsters) => data.monsters = monsters,
			Err(e) => println!("failed to get monster data: {}", e),
		}
	}

	Ok(data)
}
