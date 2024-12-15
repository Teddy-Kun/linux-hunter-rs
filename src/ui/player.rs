use ratatui::{
	layout::{Constraint, Direction, Layout},
	widgets::{Block, Gauge, Paragraph, Widget},
};

pub struct Player<'a> {
	name: &'a str,
	total_damage: u32,
	damage_delt: u32,
}

impl<'a> Player<'a> {
	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			total_damage: 0,
			damage_delt: 0,
		}
	}

	pub fn update_damage(mut self, damage: u32, total_damage: u32) -> Self {
		self.damage_delt = damage;
		self.total_damage = total_damage;
		self
	}
}

impl<'a> Widget for &Player<'a> {
	fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![Constraint::Percentage(100), Constraint::Min(28)])
			.split(area);

		Gauge::default()
			.block(Block::bordered().title(format!("{}", self.name)))
			.ratio(self.damage_delt as f64 / self.total_damage as f64)
			.render(layout[0], buf);

		let damage_text = format!("Dmg: {} / {}", self.damage_delt, self.total_damage);
		Paragraph::new(damage_text)
			.right_aligned()
			.render(layout[1], buf);
	}
}
