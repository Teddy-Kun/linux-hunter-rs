mod conf;
mod ui;

use conf::get_config;
use linux_hunter_lib::{
	err::Error,
	memory::{
		get_memory_regions,
		pattern::{
			find_current_player_name, find_emetta, find_lobby_status, find_monster,
			find_player_buff, find_player_damage, find_player_name, find_player_name_linux,
			PatternGetter, PatternType,
		},
		region::verify_regions,
	},
	mhw::find_mhw_pid,
};
use nix::unistd::Pid;
use std::{
	fs::{create_dir, remove_dir_all},
	thread::sleep,
	time::Duration,
};
use sysinfo::System;
use ui::App;

pub const PLAYER_NAME: usize = 0;
pub const CURRENT_PLAYER: usize = 1;
pub const PLAYER_DAMAGE: usize = 2;
pub const MONSTER: usize = 3;
pub const PLAYER_BUFF: usize = 4;
pub const EMETTA: usize = 5;
pub const PLAYER_NAME_LINUX: usize = 6;
pub const LOBBY_STATUS: usize = 7;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start = std::time::Instant::now();

	let conf = get_config()?;

	let mut mhw_pid = Pid::from_raw(0);
	if conf.mhw_pid.is_none() && conf.load_dump.is_none() {
		println!("Trying to detect MHW PID");
		let mut attempts = 0;
		loop {
			match find_mhw_pid() {
				Ok(pid) => {
					mhw_pid = pid;
					println!("Found pid: {}", mhw_pid);
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
			None => {
				if conf.load_dump.is_none() {
					return Err(Error::new("No MHW PID or dump path given").into());
				}
			}
		}
	}

	println!("finding main AoB entry points...");

	let mut regions = get_memory_regions(mhw_pid, conf.debug, &conf.load_dump)?;
	verify_regions(&regions)?;

	if conf.dump_mem.is_some() {
		let path = conf.dump_mem.clone().unwrap();

		remove_dir_all(&path)?;
		create_dir(path)?;
	}

	for region in &mut regions {
		if let Err(e) = region.fill_data(mhw_pid, conf.dump_mem.clone()) {
			eprintln!("Failed to fill region data: {}\n{}\n", e, region.debug_info)
		}
	}

	let mut pattern_getters = [
		PatternGetter::new(PatternType::PlayerName, find_player_name),
		PatternGetter::new(PatternType::CurrentPlayerName, find_current_player_name),
		PatternGetter::new(PatternType::PlayerDamage, find_player_damage),
		PatternGetter::new(PatternType::Monsters, find_monster),
		PatternGetter::new(PatternType::PlayerBuff, find_player_buff),
		PatternGetter::new(PatternType::Emetta, find_emetta),
		PatternGetter::new(PatternType::PlayerNameLinux, find_player_name_linux),
		PatternGetter::new(PatternType::LobbyStatus, find_lobby_status),
	];

	for get_pattern in &mut pattern_getters {
		for (i, region) in regions.iter().enumerate() {
			let get_pattern = &mut *get_pattern;
			if get_pattern.search(&region.data).is_ok() {
				get_pattern.index = Some(i);

				if conf.debug {
					println!(
						"found pattern '{:X?}' in region {:X}",
						get_pattern.pattern_type, i
					);
				}

				break;
			}
		}
	}

	if conf.debug {
		println!("took {}ms", start.elapsed().as_millis());

		let sys = System::new_all();
		let pid = sysinfo::get_current_pid().unwrap();
		if let Some(process) = sys.processes().get(&pid) {
			println!("Memory usage: {}kb", process.memory() / 1024);
		}
	}

	println!("Done");

	if conf.debug {
		for pg in &pattern_getters {
			println!(
				"{:?}:\n Found: {}\n Offset: {:?}\n Index: {:?}\n",
				pg.pattern_type,
				pg.offset.is_some(),
				pg.offset,
				pg.index
			);
		}
	}

	if pattern_getters[PLAYER_NAME_LINUX].offset.is_none() {
		return Err(Error::new("Can't find AoB for patterns::PlayerNameLinux").into());
	}

	if pattern_getters[PLAYER_DAMAGE].offset.is_none() {
		return Err(Error::new("Can't find AoB for patterns::PlayerDamage").into());
	}

	if conf.show_monsters && pattern_getters[MONSTER].offset.is_none() {
		return Err(Error::new("Can't find AoB for patterns::Monster").into());
	}

	// drop the ~3gb of memory regions, since we will use direct memory access to get the data
	drop(regions);

	let mut app = App::new(mhw_pid, &conf, pattern_getters);
	if conf.debug {
		loop {
			app.main_update_loop();
			sleep(Duration::from_secs(1));
			println!("############################################");
			println!();
		}
	} else {
		let mut terminal = ratatui::init();
		app.run(&mut terminal)?;
		ratatui::restore();
	}

	Ok(())
}
