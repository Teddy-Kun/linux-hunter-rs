mod conf;

use conf::get_config;

fn main() {
	let _ = get_config();

	// scan memory

	println!("Hello, world!");
}
