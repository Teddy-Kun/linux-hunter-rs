use std::fmt::Display;

#[cfg(feature = "backtrace")]
use backtrace::Backtrace;

#[derive(Debug)]
pub struct Error {
	message: String,
	#[cfg(feature = "backtrace")]
	trace: Backtrace,
}

impl Error {
	#[cfg(feature = "backtrace")]
	pub fn new(message: &str) -> Self {
		Self {
			message: message.to_string(),
			trace: Backtrace::new(),
		}
	}

	#[cfg(not(feature = "backtrace"))]
	pub fn new(message: &str) -> Self {
		Self {
			message: message.to_string(),
		}
	}
}

impl Display for Error {
	#[cfg(feature = "backtrace")]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:\n{:#?}", self.message, self.trace)
	}

	#[cfg(not(feature = "backtrace"))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:", self.message)
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
