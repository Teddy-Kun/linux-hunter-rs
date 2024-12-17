mod monster;
mod player;

use crate::conf::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use linux_hunter_lib::{
	memory::{region::MemoryRegion, update_regions},
	mhw::data::{Crown, FullData, MonsterInfo, PlayerInfo},
};
use monster::Monster;
use nix::unistd::Pid;
use player::Player;
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Direction, Layout, Rect},
	widgets::{Paragraph, Widget},
	DefaultTerminal, Frame,
};
use std::{
	collections::HashMap,
	io,
	time::{Duration, Instant},
};

#[derive(Debug)]
pub struct App<'a> {
	exit: bool,

	mhw_pid: Pid,
	conf: &'a Config,
	regions: HashMap<usize, MemoryRegion>,
	data: FullData,
	frametime: u128,
}

impl<'a> App<'a> {
	pub fn new(mhw_pid: Pid, conf: &'a Config, regions: HashMap<usize, MemoryRegion>) -> Self {
		Self {
			conf,
			mhw_pid,
			exit: false,
			data: FullData::default(),
			regions,
			frametime: 0,
		}
	}

	/// runs the application's main loop until the user quits
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		self.data.players = [
			Some(PlayerInfo {
				name: "Player 1".to_string(),
				damage: 2500,
				left_session: false,
			}),
			None,
			None,
			None,
		];

		self.data.monsters = [
			Some(MonsterInfo {
				name: "Rathalos".to_string(),
				crown: Some(Crown::Gold),
				hp: 12586,
				max_hp: 20600,
			}),
			None,
			None,
		];

		while !self.exit {
			let now = Instant::now();

			self.data.monsters[0].as_mut().unwrap().hp -= 100;

			// ignore the error for now
			let _ = update_regions(self.mhw_pid, &mut self.regions);

			if self.conf.show_frametime {
				self.frametime = now.elapsed().as_millis();
			}

			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	fn draw(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.area());
	}

	/// updates the application's state based on user input
	fn handle_events(&mut self) -> io::Result<()> {
		let has_event = event::poll(Duration::from_millis(100))?;
		if !has_event {
			return Ok(());
		}

		match event::read()? {
			// it's important to check that the event is a key press event as
			// crossterm also emits key release and repeat events on Windows.
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			}
			_ => {}
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		if let KeyCode::Char('q') = key_event.code {
			self.exit()
		}
	}

	fn exit(&mut self) {
		self.exit = true;
	}
}

impl<'a> Widget for &'a App<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let mut constraints = Vec::new();

		for _ in 0..8 {
			constraints.push(Constraint::Fill(1));
		}

		let layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(constraints)
			.split(area);

		let total_damage = self.data.get_total_damage();
		let mut index = 0;
		for player in self.data.players.iter().flatten() {
			let name = match player.left_session {
				true => "<Left Session>",
				false => &player.name,
			};

			Player::new(name)
				.update_damage(player.damage, total_damage)
				.render(layout[index], buf);
			index += 1;
		}

		index = 0;
		if self.conf.show_monsters {
			for monster in self.data.monsters.iter().flatten() {
				let crown = match self.conf.show_crowns {
					true => monster.crown,
					false => None,
				};

				Monster::new(&monster.name, monster.max_hp, crown)
					.update_hp(monster.hp)
					.render(layout[4 + index], buf);
				index += 1;
			}
		}

		if self.conf.show_frametime {
			Paragraph::new(format!("Frametime: {} ms", self.frametime)).render(layout[7], buf);
		}
	}
}
