use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn draw(frame: &mut Frame) {
	let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
		.white()
		.on_blue();
	frame.render_widget(greeting, frame.area());
}
