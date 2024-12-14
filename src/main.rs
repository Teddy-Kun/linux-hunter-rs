mod conf;
mod ui;

use conf::get_config;
use linux_hunter_lib::{
	err::Error,
	memory::{
		pattern::{
			find_current_player_name, find_emetta, find_lobby_status, find_monster,
			find_player_buff, find_player_damage, find_player_name, find_player_name_linux,
			PatternGetter,
		},
		scraper::get_memory_regions,
	},
	mhw::find_mhw_pid,
};

use nix::unistd::Pid;
use std::{
	sync::{atomic::AtomicBool, Arc},
	thread::sleep,
	time::Duration,
};
use ui::draw;

pub const PLAYER_NAME: usize = 0;
pub const CURRENT_PLAYER: usize = 1;
pub const PLAYER_DAMAGE: usize = 2;
pub const MONSTER: usize = 3;
pub const PLAYER_BUFF: usize = 4;
pub const EMETTA: usize = 5;
pub const PLAYER_NAME_LINUX: usize = 6;
pub const LOBBY_WSTATUS: usize = 7;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let conf = get_config()?;

	let mhw_pid;
	if conf.mhw_pid.is_none() {
		println!("Trying to detect MHW PID");
		let mut attempts = 0;
		loop {
			match find_mhw_pid() {
				Ok(pid) => {
					mhw_pid = pid;
					break;
				}

				Err(e) => {
					attempts += 1;
					if attempts > 50 {
						eprintln!("Couldn't find MHW PID");
						return Err(e);
					}
					sleep(Duration::from_millis(200));
				}
			}
		}
	} else {
		match conf.mhw_pid {
			Some(pid) => mhw_pid = Pid::from_raw(pid),
			None => todo!("conf load"),
		}
	}

	println!("Found pid: {}", mhw_pid);

	println!("finding main AoB entry points...");

	let mut regions = get_memory_regions(mhw_pid)?;
	for region in &mut regions {
		if let Err(e) = region.fill_data(mhw_pid, conf.dump_mem.clone()) {
			eprintln!("Failed to fill region data: {}\n{}\n", e, region.debug_info)
		}
	}

	let mut pattern_getters: Vec<PatternGetter> = vec![
		PatternGetter::new("PlayerName", find_player_name),
		PatternGetter::new("CurrentPlayerName", find_current_player_name),
		PatternGetter::new("PlayerDamage", find_player_damage),
		PatternGetter::new("Monster", find_monster),
		PatternGetter::new("PlayerBuff", find_player_buff),
		PatternGetter::new("Emetta", find_emetta),
		PatternGetter::new("PlayerNameLinux", find_player_name_linux),
		PatternGetter::new("LobbyStatus", find_lobby_status),
	];

	for get_pattern in &mut pattern_getters {
		for region in &regions {
			if let Ok(_) = get_pattern.search(&region.data) {
				break;
			}
		}
	}

	// TODO: remove
	println!("Patterns {:#?}", pattern_getters);

	println!("Done");

	if pattern_getters[PLAYER_NAME_LINUX].result.is_none()
		|| pattern_getters[PLAYER_DAMAGE].result.is_none()
	{
		return Err(Error::new("Can't find AoB for patterns::PlayerNameLinux and/or patterns::PlayerDamage - Try to run with 'sudo' and/or specify a pid").into());
	}

	if conf.show_monsters && pattern_getters[MONSTER].result.is_none() {
		return Err(Error::new("Can't find AoB for patterns::Monster").into());
	}

	let run = Arc::new(AtomicBool::new(true));
	let run_clone = Arc::clone(&run);

	ctrlc::set_handler(move || {
		run_clone.store(false, std::sync::atomic::Ordering::Relaxed);
	})?;

	let mut terminal = ratatui::init();
	// main loop
	while run.load(std::sync::atomic::Ordering::Relaxed) {
		terminal.draw(draw)?;

		sleep(Duration::from_millis(1000));
	}

	Ok(())
}
