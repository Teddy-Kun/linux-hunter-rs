use super::{
	pattern::{PatternGetter, PatternType},
	region::read_memory,
};
use crate::{
	memory::region::load_rel_addr,
	mhw::{
		data::{GameData, MonsterInfo, PlayerInfo, SessionInfo},
		offsets,
	},
	read_mem_to_type,
};
use nix::unistd::Pid;
use std::str;
use tracing::{debug, error, trace};

fn get_session_data(pid: Pid, patterns: &[PatternGetter]) -> anyhow::Result<SessionInfo> {
	// TODO: maybe only copy memory to a buffer with 1 syscall, then read from it, instead of using 4 syscalls?

	let pattern = &patterns[PatternType::LobbyStatus as usize];

	let mut info = SessionInfo::default();

	// if the mem_location is none, just return the default SessionInfo, so we can still attempt to check for players and monsters
	if pattern.mem_location.is_none() {
		return Ok(info);
	}

	trace!("pattern: {:#?}", pattern);

	let start = pattern.mem_location.unwrap().address;
	// trace!("start: {}", start);

	let pointer = load_rel_addr(pid, start)?;

	// let pointer = read_mem_to_type!(pid, start, u32) as usize;
	trace!("pointer: {}", pointer);

	// let rel_pointer = load_rel_addr(pid, pointer)?;

	let debug_ptr = 211829448;
	let cur_pointer = (pointer + offsets::SESSION_ID) as u32;
	trace!("cur_pointer: {}", cur_pointer);

	// TODO: fix EFAULT: Bad address
	// was pointer + offsets::SESSION_ID
	let mem = read_memory(pid, debug_ptr, offsets::ID_LENGTH)?; // Fails here ATM
															 // since the game uses UTF-8 this should be safe
	info.session_id = unsafe { str::from_boxed_utf8_unchecked(mem) };
	trace!("Got session id '{}'", info.session_id);

	let mem = read_memory(
		pid,
		pointer as usize + start + offsets::SESSION_HOST_NAME,
		offsets::PLAYER_NAME_LENGTH,
	)?;
	// since the game uses UTF-8 this should be safe
	info.hostname = unsafe { str::from_boxed_utf8_unchecked(mem) };
	trace!("Got host name");

	// TODO: not working, find out why and fix this
	let start = pattern.mem_location.unwrap().address;
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

fn get_damage(pid: Pid, patterns: &[PatternGetter]) -> anyhow::Result<Box<[PlayerInfo]>> {
	trace!("pid: {}, patterns: {:#?}", pid, patterns);
	Err(anyhow::anyhow!("not implemented"))
}

fn get_monster_data(pid: Pid, patterns: &[PatternGetter]) -> anyhow::Result<Box<[MonsterInfo]>> {
	let pattern = &patterns[PatternType::Monsters as usize];

	let start = pattern.mem_location.unwrap().address;
	let mem = read_memory(pid, start, 256)?;

	debug!("{:02X?}", mem);

	get_single_monster()?;

	Err(anyhow::anyhow!("not implemented"))
}

fn get_single_monster() -> anyhow::Result<MonsterInfo> {
	Err(anyhow::anyhow!("not implemented"))
}

pub fn update_all(
	pid: Pid,
	patterns: &[PatternGetter],
	get_monsters: bool,
) -> anyhow::Result<GameData> {
	let mut data = GameData::new(get_session_data(pid, patterns)?);
	debug!("session info: {:#?}", data.session);

	if data.session.is_expedition || data.session.is_mission {
		match get_damage(pid, patterns) {
			Ok(damage) => data.players = damage,
			Err(e) => error!("failed to get player damage: {}", e),
		}

		if get_monsters {
			match get_monster_data(pid, patterns) {
				Ok(monsters) => data.monsters = monsters,
				Err(e) => error!("failed to get monster data: {}", e),
			}
		}
	}

	Ok(data)
}
