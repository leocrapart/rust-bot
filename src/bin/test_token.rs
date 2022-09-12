use std::env;
use dotenv::dotenv;

fn main() {
	dotenv().ok();
	let token = env::var("DISCORD_TOKEN").expect("token");
	println!("{token}")
}