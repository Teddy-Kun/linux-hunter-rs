use nix::unistd::Pid;

mod err;
mod memory;

pub fn get_new_browser() {
	let browser = memory::browser::Browser::new(Pid::from_raw(0), false, false, false);

	println!("{:?}", browser);
}
