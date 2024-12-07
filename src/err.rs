use std::fmt::Display;

use backtrace::Backtrace;

#[derive(Debug)]
pub struct Error<'a> {
	message: &'a str,
	trace: Backtrace,
}

impl<'a> Error<'a> {
	pub fn new(message: &'a str) -> Self {
		Self {
			message,
			trace: Backtrace::new(),
		}
	}
}

impl<'a> Display for Error<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl<'a> std::error::Error for Error<'a> {
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
