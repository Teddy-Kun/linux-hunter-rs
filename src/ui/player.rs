use ratatui::{
	layout::{Constraint, Direction, Layout},
	style::{Style, Stylize},
	widgets::{Block, Gauge, Paragraph, Widget},
};

pub struct Player<'a> {
	name: &'a str,
	total_damage: usize,
	damage_delt: usize,
}

impl<'a> Player<'a> {
	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			total_damage: 0,
			damage_delt: 0,
		}
	}

	pub fn update_damage(mut self, damage: usize, total_damage: usize) -> Self {
		self.damage_delt = damage;
		self.total_damage = total_damage;
		self
	}
}

impl Widget for &Player<'_> {
	fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![Constraint::Percentage(100), Constraint::Min(28)])
			.split(area);

		let sublayout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(vec![
				Constraint::Fill(1),
				Constraint::Min(3),
				Constraint::Fill(1),
			])
			.split(layout[1]);

		Gauge::default()
			.block(Block::bordered().title(self.name.to_string()))
			.gauge_style(Style::new().white().on_black())
			.ratio(self.damage_delt as f64 / self.total_damage as f64)
			.render(layout[0], buf);

		let damage_text = format!("Dmg: {} / {}", self.damage_delt, self.total_damage);
		Paragraph::new(damage_text)
			.right_aligned()
			.render(sublayout[1], buf);
	}
}
