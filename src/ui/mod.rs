mod monster;
mod player;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use linux_hunter_lib::mhw::ui_data::Crown;
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

	max_hp: u32,
	hp: u32,
}

impl<'a> App<'a> {
	pub fn new(conf: &'a Config) -> Self {
		Self {
			conf,
			exit: false,
			max_hp: 0,
			hp: 0,
		}
	}

	/// runs the application's main loop until the user quits
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		self.max_hp = 20600;
		self.hp = 20600;

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
			KeyCode::Left => self.decrement_counter(),
			KeyCode::Right => self.increment_counter(),
			_ => {}
		}
	}

	fn exit(&mut self) {
		self.exit = true;
	}

	fn increment_counter(&mut self) {
		self.hp += 120;
		if self.hp >= self.max_hp {
			self.hp = self.max_hp;
		}
	}

	fn decrement_counter(&mut self) {
		if self.hp >= 120 {
			self.hp -= 120;
		} else {
			self.hp = 0;
		}
	}
}

impl<'a> Widget for &'a App<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(vec![
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
			])
			.split(area);

		Player::new("Player 1")
			.update_damage(2500, 10000)
			.render(layout[0], buf);
		Player::new("Player 2")
			.update_damage(2500, 10000)
			.render(layout[1], buf);
		Player::new("Player 3")
			.update_damage(2500, 10000)
			.render(layout[2], buf);
		Player::new("Player 4")
			.update_damage(2500, 10000)
			.render(layout[3], buf);

		let crown = match self.conf.show_crowns {
			true => Some(Crown::SmallGold),
			false => None,
		};

		Monster::new("Rathalos", self.max_hp, crown)
			.update_hp(self.hp)
			.render(layout[4], buf);

		Monster::new("Rathian", self.max_hp, crown)
			.update_hp(self.hp)
			.render(layout[5], buf);

		Monster::new("Yian Garuga", self.max_hp, crown)
			.update_hp(self.hp)
			.render(layout[6], buf);
	}
}
