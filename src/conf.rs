use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short = 'm', long)]
    show_monsters: bool,

    #[arg(short = 'c', long)]
    show_crowns: bool,

    #[arg(short = 's', long)]
    save: String,

    #[arg(short = 'l', long)]
    load: String,

    #[arg(long)]
    no_direct_mem: bool,

    #[arg(short = 'f', long)]
    f_display: String,

    #[arg(long)]
    mhw_pid: u64,

    #[arg(long)]
    debug_ptrs: bool,

    #[arg(long)]
    debug_all: bool,

    #[arg(long)]
    mem_dirty_opt: bool,

    #[arg(long)]
    no_lazy_alloc: bool,

    #[arg(short = 'r', long)]
    refresh: u16,

    #[arg(long)]
    no_color: bool,

    #[arg(long)]
    compact_display: bool,
}

pub fn get_config() -> Config {
    Config::parse()
}
