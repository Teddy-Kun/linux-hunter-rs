mod conf;

use conf::get_config;
use linux_rhunter_lib::{
	memory::pattern::{
		MemoryPattern, CURRENT_PLAYER_NAME, EMETTA, LOBBY_STATUS, MONSTER, PLAYER_BUFF,
		PLAYER_NAME, PLAYER_NAME_LINUX,
	},
	mhw::find_mhw_pid,
};
use nix::unistd::Pid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let conf = get_config()?;

	// scan memory
	let all_patterns: Vec<MemoryPattern> = vec![
		MemoryPattern::new(&PLAYER_NAME)?,
		MemoryPattern::new(&CURRENT_PLAYER_NAME)?,
		MemoryPattern::new(&MONSTER)?,
		MemoryPattern::new(&PLAYER_BUFF)?,
		MemoryPattern::new(&EMETTA)?,
		MemoryPattern::new(&PLAYER_NAME_LINUX)?,
		MemoryPattern::new(&LOBBY_STATUS)?,
	];

	let mhw_pid;
	if conf.mhw_pid.is_none() && conf.load.is_none() {
		mhw_pid = find_mhw_pid()?;
	} else {
		match conf.mhw_pid {
			Some(pid) => mhw_pid = Pid::from_raw(pid as i32),
			None => todo!("conf load"),
		}
	}

	println!("Found pid: {}", mhw_pid);

	// start with browser

	Ok(())
}
