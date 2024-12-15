use linux_hunter_lib::mhw::ui_data::Crown;
use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Style, Stylize},
	widgets::{Block, Gauge, Paragraph, Widget},
};

pub struct Monster<'a> {
	pub name: &'a str,
	pub hp: u32,
	pub max_hp: u32,
	pub crown: Option<Crown>,
}

impl<'a> Monster<'a> {
	pub fn new(name: &'a str, max_hp: u32, crown: Option<Crown>) -> Self {
		Self {
			name,
			max_hp,
			hp: max_hp,
			crown,
		}
	}

	#[must_use = "method moves the value of self and returns the modified value"]
	pub fn update_hp(mut self, hp: u32) -> Self {
		if hp > self.max_hp {
			self.hp = self.max_hp;
		} else {
			self.hp = hp;
		}

		self
	}
}

impl<'a> Widget for &Monster<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(100),
				Constraint::Min(17),
				Constraint::Min(11),
			])
			.split(area);

		// center the right side height wise
		let sublayout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(vec![
				Constraint::Fill(1),
				Constraint::Min(3),
				Constraint::Fill(1),
			]);
		let sublayout_right = sublayout.clone();

		let sublayout_center = sublayout.split(layout[1]);
		let sublayout_right = sublayout_right.split(layout[2]);

		Gauge::default()
			.block(Block::bordered().title(format!("{}", self.name)))
			.gauge_style(Style::new().white().on_black().italic())
			.ratio(self.hp as f64 / self.max_hp as f64)
			.render(layout[0], buf);

		let hp_text = format!("HP: {}/{}", self.hp, self.max_hp);
		Paragraph::new(hp_text)
			.centered()
			.render(sublayout_center[1], buf);

		let crown_text = match &self.crown {
			Some(crown) => format!("{}", crown),
			None => String::from(""),
		};
		Paragraph::new(crown_text)
			.right_aligned()
			.render(sublayout_right[1], buf);
	}
}
