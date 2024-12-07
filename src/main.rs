use conf::get_config;

mod conf;

fn main() {
    let _ = get_config();
    println!("Hello, world!");
}
