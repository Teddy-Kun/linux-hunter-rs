mod monster;
mod player;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use linux_hunter_lib::mhw::data::{Crown, MonsterInfo, PlayerInfo, UiInfo};
use monster::Monster;
use player::Player;
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Direction, Layout, Rect},
	widgets::Widget,
	DefaultTerminal, Frame,
};

use crate::conf::Config;

#[derive(Debug)]
pub struct App<'a> {
	exit: bool,

	conf: &'a Config,
	data: UiInfo,
}

impl<'a> App<'a> {
	pub fn new(conf: &'a Config) -> Self {
		Self {
			conf,
			exit: false,
			data: UiInfo::default(),
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
		match key_event.code {
			KeyCode::Char('q') => self.exit(),
			_ => {}
		}
	}

	fn exit(&mut self) {
		self.exit = true;
	}
}

impl<'a> Widget for &'a App<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let mut constraints = Vec::new();

		for _ in 0..self.data.get_num_players() + self.data.get_num_monsters() {
			constraints.push(Constraint::Fill(1));
		}

		let layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(constraints)
			.split(area);

		let total_damage = self.data.get_total_damage();
		let mut index = 0;

		for player in &self.data.players {
			if let Some(player) = player {
				let name = match player.left_session {
					true => "<Left Session>",
					false => &player.name,
				};

				Player::new(name)
					.update_damage(player.damage, total_damage)
					.render(layout[index], buf);
				index += 1;
			}
		}

		if self.conf.show_monsters {
			for monster in &self.data.monsters {
				if let Some(monster) = monster {
					let crown = match self.conf.show_crowns {
						true => monster.crown,
						false => None,
					};

					Monster::new(&monster.name, monster.max_hp, crown)
						.update_hp(monster.hp)
						.render(layout[index], buf);
					index += 1;
				}
			}
		}
	}
}
