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
		help = "Specifies what is the UI/stats refresh interval in ms (default 1000)"
	)]
	pub refresh: Option<u16>,

	#[arg(
		long,
		help = "Dumps memory to a file in the dir specified upon initialization. WARNING: very slow, memory hungry AND DELTES ALL CONTENTS OF THAT DIRECTORY, but useful for debugging"
	)]
	pub dump_mem: Option<String>,

	#[arg(long, help = "Loads a previously dumped memory dump")]
	pub load_dump: Option<String>,

	#[arg(long, help = "Shows how long it took to construct a frame in the tui")]
	pub show_frametime: bool,

	#[arg(
		short,
		long,
		help = "Manually set the log level (trace, debug, info, warn, error).",
		default_value = "info"
	)]
	pub log_level: Level,
}

impl Config {
	pub fn debug(&self) -> bool {
		self.log_level > Level::INFO
	}
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
	let conf = Config::parse();

	Ok(conf)
}
