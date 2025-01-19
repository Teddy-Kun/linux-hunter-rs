use super::{
	pattern::{PatternGetter, PatternType},
	region::read_memory,
};
use crate::{
	mhw::{
		data::{GameData, MonsterInfo, PlayerInfo, SessionInfo},
		offsets,
	},
	read_mem_to_type,
};
use nix::unistd::Pid;
use std::str;
use tracing::{debug, error, trace};

fn get_session_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<SessionInfo, Box<dyn std::error::Error>> {
	// TODO: maybe only copy memory to a buffer with 1 syscall, then read from it, instead of using 4 syscalls?

	let pattern = &patterns[PatternType::LobbyStatus as usize];

	let mut info = SessionInfo::default();

	// if the mem_location is none, just return the default SessionInfo, so we can still attempt to check for players and monsters
	if pattern.mem_location.is_none() {
		return Ok(info);
	}

	let start = pattern.mem_location.unwrap().get_addr();
	trace!("start: {}", start);

	let pointer = read_mem_to_type!(pid, start, usize);
	trace!("pointer: {}", pointer);

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_ID,
		offsets::ID_LENGTH,
	)?; // Fails here ATM
	 // since the game uses UTF-8 this should be safe
	info.session_id = unsafe { str::from_boxed_utf8_unchecked(mem) };
	trace!("Got session id");

	let mem = read_memory(
		pid,
		pointer + start + offsets::SESSION_HOST_NAME,
		offsets::PLAYER_NAME_LENGTH,
	)?;
	// since the game uses UTF-8 this should be safe
	info.hostname = unsafe { str::from_boxed_utf8_unchecked(mem) };
	trace!("Got host name");

	// TODO: not working, find out why and fix this
	let start = pattern.mem_location.unwrap().get_addr();
	let pointer = read_mem_to_type!(pid, start, u64) as usize;
	let mem = read_memory(pid, pointer + start + offsets::MISSION_STATUS_OFFSET, 1)?;
	info.is_mission = mem[0] != 0;
	trace!("Got mission status");

	// TODO: not working, find out why and fix this
	let mem = read_memory(pid, pointer + start + offsets::EXPEDITION_STATUS_OFFSET, 1)?;
	info.is_expedition = mem[0] != 0;
	trace!("Got expedition status");

	Ok(info)
}

fn get_damage(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<Box<[PlayerInfo]>, Box<dyn std::error::Error>> {
	trace!("pid: {}, patterns: {:#?}", pid, patterns);
	Err("not implemented".into())
}

fn get_monster_data(
	pid: Pid,
	patterns: &[PatternGetter],
) -> Result<Box<[MonsterInfo]>, Box<dyn std::error::Error>> {
	let pattern = &patterns[PatternType::Monsters as usize];

	let start = pattern.mem_location.unwrap().get_addr();
	let mem = read_memory(pid, start, 256)?;

	debug!("{:02X?}", mem);

	get_single_monster()?;

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
	debug!("session info: {:#?}", data.session);

	if data.session.is_expedition || data.session.is_mission {
		match get_damage(pid, patterns) {
			Ok(damage) => data.players = damage,
			Err(e) => error!("failed to get player damage: {}", e),
		}

		match get_monster_data(pid, patterns) {
			Ok(monsters) => data.monsters = monsters,
			Err(e) => error!("failed to get monster data: {}", e),
		}
	}

	Ok(data)
}
