use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
	#[arg(
		short = 'm',
		long,
		help = "Shows HP monsters data (requires slightly more CPU usage)"
	)]
	pub show_monsters: bool,

	#[arg(
		short = 'c',
		long,
		help = "Shows information about crowns (Gold Small, Silver Large and Gold Large)"
	)]
	pub show_crowns: bool,

	#[arg(
		long,
		help = "Specifies which pid to scan memory for (usually main MH:W). When not specified, linux-hunter-rs will try to find it automatically"
	)]
	pub mhw_pid: Option<i32>,

	#[arg(
		short = 'r',
		long,
		help = "Specifies what is the UI/stats refresh interval in ms. If unspecified it will try to refresh at 60 fps (16.66ms)"
	)]
	pub refresh: Option<f64>,

	#[arg(
		long,
		help = "Dumps memory to a file in the dir specified upon initialization. WARNING: very slow, memory hungry AND DELTES ALL CONTENTS OF THAT DIRECTORY, but useful for debugging"
	)]
	pub dump_mem: Option<Box<str>>,

	#[arg(long, help = "Loads a previously dumped memory dump")]
	pub load_dump: Option<Box<str>>,

	#[arg(long, help = "Shows how long it took to construct a frame in the tui")]
	pub show_frametime: bool,

	#[arg(
		short,
		long,
		help = "Manually set the log level (trace, debug, info, warn, error).",
		default_value = "info"
	)]
	pub log_level: Level,

	#[arg(
		long,
		help = "Sets the path to the log file. Defaults to ~/.cache/linux-hunter-rs.log"
	)]
	pub log_file: Option<Box<str>>,
}

impl Config {
	pub fn debug(&self) -> bool {
		self.log_level > Level::INFO
	}
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
	let mut conf = Config::parse();

	if conf.log_file.is_none() {
		match dirs::cache_dir() {
			None => eprintln!("Failed to get cache dir! Will not log to file."),
			Some(mut dir) => {
				dir.push("linux-hunter-rs.log");
				conf.log_file = Some(Box::from(dir.to_str().unwrap()))
			}
		}
	}

	Ok(conf)
}
