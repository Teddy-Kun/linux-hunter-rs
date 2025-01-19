mod conf;
mod ui;

use conf::{get_config, Config};
use linux_hunter_lib::{
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
	io::{self, Write},
	thread::sleep,
	time::Duration,
};
use sysinfo::System;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;
use ui::App;

pub const PLAYER_NAME: usize = 0;
pub const CURRENT_PLAYER: usize = 1;
pub const PLAYER_DAMAGE: usize = 2;
pub const MONSTER: usize = 3;
pub const PLAYER_BUFF: usize = 4;
pub const EMETTA: usize = 5;
pub const PLAYER_NAME_LINUX: usize = 6;
pub const LOBBY_STATUS: usize = 7;

fn main_loop(conf: Config) -> Result<(), Box<dyn std::error::Error>> {
	let start = std::time::Instant::now();

	let mut mhw_pid = Pid::from_raw(0);
	if conf.mhw_pid.is_none() && conf.load_dump.is_none() {
		info!("Trying to detect MHW PID");
		let mut attempts = 0;
		loop {
			match find_mhw_pid() {
				Ok(pid) => {
					mhw_pid = pid;
					info!("Found pid: {}", mhw_pid);
					break;
				}

				Err(e) => {
					attempts += 1;
					if attempts > 50 {
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
					return Err("No MHW PID or dump path given".into());
				}
			}
		}
	}

	info!("finding main AoB entry points...");

	let mut regions = get_memory_regions(mhw_pid, &conf.load_dump)?;
	verify_regions(&regions)?;

	if conf.dump_mem.is_some() {
		let path = conf.dump_mem.clone().unwrap();

		remove_dir_all(&path)?;
		create_dir(path)?;
	}

	for region in &mut regions {
		if let Err(e) = region.fill_data(mhw_pid, conf.dump_mem.clone()) {
			warn!("Failed to fill region data: {}\n{}\n", e, region.debug_info)
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
			if region.data.is_some() {
				let get_pattern = &mut *get_pattern;
				if get_pattern.search(region).is_ok() {
					debug!(
						"found pattern '{:X?}' in region {:X}",
						get_pattern.pattern_type, i
					);

					break;
				}
			}
		}
	}

	if conf.debug() {
		debug!("took {}ms", start.elapsed().as_millis());

		let sys = System::new_all();
		let pid = sysinfo::get_current_pid().unwrap();
		if let Some(process) = sys.processes().get(&pid) {
			debug!("Memory usage: {}kb", process.memory() / 1024);
		}
	}

	info!("Done");

	if conf.debug() {
		for pg in &pattern_getters {
			debug!(
				"{:?}:\n Found: {}\n MemoryLocation: {:?}",
				pg.pattern_type,
				pg.mem_location.is_some(),
				pg.mem_location
			);
		}
	}

	if pattern_getters[PLAYER_NAME_LINUX].mem_location.is_none() {
		return Err("Can't find AoB for patterns::PlayerNameLinux".into());
	}

	if pattern_getters[PLAYER_DAMAGE].mem_location.is_none() {
		return Err("Can't find AoB for patterns::PlayerDamage".into());
	}

	if conf.show_monsters && pattern_getters[MONSTER].mem_location.is_none() {
		return Err("Can't find AoB for patterns::Monster".into());
	}

	// drop the ~3gb of memory regions, since we will use direct memory access to get the data
	drop(regions);

	let mut app = App::new(mhw_pid, &conf, pattern_getters);
	if conf.debug() {
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

fn main() {
	let conf = match get_config() {
		Ok(conf) => conf,
		Err(e) => {
			eprintln!("Failed to get config: {}", e);
			std::process::exit(1);
		}
	};

	let subscriber = FmtSubscriber::builder()
		.with_max_level(conf.log_level)
		.finish();
	if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
		eprintln!("Failed to set global default subscriber: {}", e);
		eprintln!("You wont get any logs!");
	}

	let mut exit_code = 0;

	match main_loop(conf) {
		Ok(_) => (),
		Err(e) => {
			error!("{}", e);
			exit_code = 1;
		}
	}

	let _ = io::stdout().flush();
	let _ = io::stderr().flush();
	std::process::exit(exit_code);
}
