use std::fmt::Display;

use backtrace::Backtrace;

#[derive(Debug)]
pub struct Error {
	message: String,
	trace: Backtrace,
}

impl Error {
	pub fn new(message: &str) -> Self {
		Self {
			message: message.to_string(),
			trace: Backtrace::new(),
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:\n{:#?}", self.message, self.trace)
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}

	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}

	fn description(&self) -> &str {
		&self.message
	}
}
