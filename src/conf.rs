use clap::Parser;
use linux_hunter_lib::err::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
	#[arg(short = 'm', long)]
	pub show_monsters: bool,

	#[arg(short = 'c', long)]
	pub show_crowns: bool,

	#[arg(short = 's', long)]
	pub save: Option<String>,

	#[arg(short = 'l', long)]
	pub load: Option<String>,

	#[arg(long)]
	pub no_direct_mem: bool,

	#[arg(short = 'f', long)]
	pub f_display: Option<String>,

	#[arg(long)]
	pub mhw_pid: Option<i32>,

	#[arg(long)]
	pub debug_ptrs: bool,

	#[arg(long)]
	pub debug_all: bool,

	#[arg(long)]
	pub mem_dirty_opt: bool,

	#[arg(long)]
	pub no_lazy_alloc: bool,

	#[arg(short = 'r', long)]
	pub refresh: Option<u16>,

	#[arg(long)]
	pub no_color: bool,

	#[arg(long)]
	pub compact_display: bool,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
	let conf = Config::parse();

	if conf.save.is_some() && conf.load.is_some() {
		return Err(Error::new("Can't specify both 'load' and 'save' options").into());
	}

	Ok(conf)
}
