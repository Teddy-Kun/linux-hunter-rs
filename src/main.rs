#![allow(dead_code)]

mod conf;
mod ui;

use conf::get_config;
use linux_hunter_lib::{
	err::Error,
	memory::{
		pattern::{
			find_current_player_name, find_emetta, find_lobby_status, find_monster,
			find_player_buff, find_player_name, find_player_name_linux,
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

const PLAYER_NAME_INDEX: usize = 0;
const CURRENT_PLAYER_NAME_INDEX: usize = 1;
const MONSTER_INDEX: usize = 2;
const PLAYER_BUFF_INDEX: usize = 3;
const EMETTA_INDEX: usize = 4;
const PLAYER_NAME_LINUX_INDEX: usize = 5;
const LOBBY_STATUS_INDEX: usize = 6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let conf = get_config()?;

	let mhw_pid;
	if conf.mhw_pid.is_none() && conf.load.is_none() {
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

	match conf.load {
		Some(_l) => todo!("load"),
		None => {
			if let Some(_s) = conf.save {
				todo!("save")
			}
		}
	}

	println!("finding main AoB entry points...");

	let mut regions = get_memory_regions(mhw_pid, true)?;
	for region in &mut regions {
		if let Err(e) = region.fill_data(mhw_pid) {
			eprintln!("Failed to fill region data: {}\n{}\n", e, region.debug_info)
		}
	}

	let pattern_getters: Vec<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>> = vec![
		find_player_name,
		find_current_player_name,
		find_monster,
		find_player_buff,
		find_emetta,
		find_player_name_linux,
		find_lobby_status,
	];

	if conf.debug_ptrs {
		todo!("debug_ptrs");
	}

	let mut findings: Vec<Vec<u8>> = Vec::with_capacity(pattern_getters.len());

	for get_pattern in &pattern_getters {
		let mut res = Vec::new();

		for region in &regions {
			if let Ok(r) = get_pattern(&region.data) {
				res = r;
				break;
			}
		}

		findings.push(res);
	}

	println!("findings {:#?}", findings);

	println!("Done");

	if conf.debug_all {
		return Ok(());
	}

	if findings[PLAYER_NAME_LINUX_INDEX].len() == 0
		|| findings[CURRENT_PLAYER_NAME_INDEX].len() == 0
	{
		return Err(Error::new("Can't find AoB for patterns::PlayerNameLinux and/or patterns::PlayerDamage - Try to run with 'sudo' and/or specify a pid").into());
	}

	if conf.show_monsters && findings[MONSTER_INDEX].len() == 0 {
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
