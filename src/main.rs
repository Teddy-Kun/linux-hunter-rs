mod conf;
mod ui;

use conf::get_config;
use linux_hunter_lib::{
	err::Error,
	memory::{
		browser::Browser,
		pattern::{
			MemoryPattern, CURRENT_PLAYER_NAME, EMETTA, LOBBY_STATUS, MONSTER, PLAYER_BUFF,
			PLAYER_NAME, PLAYER_NAME_LINUX,
		},
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
			Some(pid) => mhw_pid = Pid::from_raw(pid as i32),
			None => todo!("conf load"),
		}
	}

	println!("Found pid: {}", mhw_pid);

	let mut browser = Browser::new(
		mhw_pid,
		conf.mem_dirty_opt,
		!conf.no_lazy_alloc,
		!conf.no_direct_mem,
	);

	println!("Browser: {:#?}", browser);

	match conf.load {
		Some(_l) => todo!("load"),
		None => {
			browser.snap()?;

			if let Some(_s) = conf.save {
				todo!("save")
			}
		}
	}

	println!("finding main AoB entry points...");

	let mut all_patterns: Vec<MemoryPattern> = vec![
		MemoryPattern::new(&PLAYER_NAME)?,
		MemoryPattern::new(&CURRENT_PLAYER_NAME)?,
		MemoryPattern::new(&MONSTER)?,
		MemoryPattern::new(&PLAYER_BUFF)?,
		MemoryPattern::new(&EMETTA)?,
		MemoryPattern::new(&PLAYER_NAME_LINUX)?,
		MemoryPattern::new(&LOBBY_STATUS)?,
	];
	browser.find_patterns(&mut all_patterns, conf.debug_all);

	if conf.debug_ptrs {
		for p in &all_patterns {
			if p.mem_location > 0 {}
		}

		todo!("debug_ptrs");
	}
	println!("Done");

	if conf.debug_all {
		return Ok(());
	}

	if all_patterns[5].mem_location < 0 || all_patterns[1].mem_location < 0 {
		return Err(Error::new("Can't find AoB for patterns::PlayerNameLinux and/or patterns::PlayerDamage - Try to run with 'sudo' and/or specify a pid").into());
	}

	if conf.show_monsters && all_patterns[2].mem_location < 0 {
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
	}

	Ok(())
}
