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
		region::verify_regions,
		scraper::get_memory_regions,
	},
	mhw::find_mhw_pid,
};

use nix::unistd::Pid;
use std::{
	sync::{atomic::AtomicBool, Arc, Mutex},
	thread::{self, sleep},
	time::Duration,
};
use sysinfo::System;
use ui::draw;

pub const PLAYER_NAME: usize = 0;
pub const CURRENT_PLAYER: usize = 1;
pub const PLAYER_DAMAGE: usize = 2;
pub const MONSTER: usize = 3;
pub const PLAYER_BUFF: usize = 4;
pub const EMETTA: usize = 5;
pub const PLAYER_NAME_LINUX: usize = 6;
pub const LOBBY_STATUS: usize = 7;

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
			None => todo!("conf load"),
		}
	}

	println!("finding main AoB entry points...");

	let mut regions = get_memory_regions(mhw_pid)?;
	verify_regions(&regions)?;

	for region in &mut regions {
		if let Err(e) = region.fill_data(mhw_pid, conf.dump_mem.clone()) {
			eprintln!("Failed to fill region data: {}\n{}\n", e, region.debug_info)
		}
	}

	println!("found {} regions", regions.len());

	let mut pattern_getters = [
		Arc::new(Mutex::new(PatternGetter::new(
			"PlayerName",
			find_player_name,
		))),
		Arc::new(Mutex::new(PatternGetter::new(
			"CurrentPlayerName",
			find_current_player_name,
		))),
		Arc::new(Mutex::new(PatternGetter::new(
			"PlayerDamage",
			find_player_damage,
		))),
		Arc::new(Mutex::new(PatternGetter::new("Monster", find_monster))),
		Arc::new(Mutex::new(PatternGetter::new(
			"PlayerBuff",
			find_player_buff,
		))),
		Arc::new(Mutex::new(PatternGetter::new("Emetta", find_emetta))),
		Arc::new(Mutex::new(PatternGetter::new(
			"PlayerNameLinux",
			find_player_name_linux,
		))),
		Arc::new(Mutex::new(PatternGetter::new(
			"LobbyStatus",
			find_lobby_status,
		))),
	];

	let regions_arc = Arc::new(regions);
	let mut threads = Vec::new();
	for get_pattern in &mut pattern_getters {
		let regions_clone = Arc::clone(&regions_arc);
		let get_pattern = Arc::clone(&get_pattern);
		let handle = thread::spawn(move || {
			for (i, region) in regions_clone.iter().enumerate() {
				let mut get_pattern = get_pattern.lock().unwrap();
				if let Ok(_) = get_pattern.search(&region.data) {
					get_pattern.index = i;
					println!("found pattern '{}' in region {}", get_pattern.debug_name, i);
					break;
				}
			}
		});
		threads.push(handle);
	}

	for handle in threads {
		handle.join().unwrap();
	}

	// TODO: remove
	println!("Patterns {:#?}", pattern_getters);

	let sys = System::new_all();

	let pid = sysinfo::get_current_pid().unwrap();
	if let Some(process) = sys.processes().get(&pid) {
		println!("Memory usage: {}kb", process.memory() / 1024);
	}

	println!("Done");

	if pattern_getters[PLAYER_NAME_LINUX]
		.lock()
		.unwrap()
		.result
		.is_none()
		|| pattern_getters[PLAYER_DAMAGE]
			.lock()
			.unwrap()
			.result
			.is_none()
	{
		return Err(Error::new("Can't find AoB for patterns::PlayerNameLinux").into());
	}

	if pattern_getters[PLAYER_DAMAGE]
		.lock()
		.unwrap()
		.result
		.is_none()
	{
		return Err(Error::new("Can't find AoB for patterns::PlayerDamage").into());
	}

	if conf.show_monsters && pattern_getters[MONSTER].lock().unwrap().result.is_none() {
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
