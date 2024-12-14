use clap::Parser;

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
		help = "Don't access MH:W memory directly and dynamically, use a local copy via buffers - increase CPU usage (both u and s) at the advantage of potentially slightly less inconsistencies"
	)]
	pub no_direct_mem: bool,

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
		help = "Dumps memory to a file in the dir specified upon initialization"
	)]
	pub dump_mem: Option<String>,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
	let conf = Config::parse();

	if conf.no_direct_mem {
		todo!("no_direct_mem");
	}

	Ok(conf)
}
