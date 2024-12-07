use nix::unistd::Pid;

pub mod err;
pub mod memory;
pub mod mhw;

pub fn get_new_browser() {
	let browser = memory::browser::Browser::new(Pid::from_raw(0), false, false, false);

	println!("{:?}", browser);
}
