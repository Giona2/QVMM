use std::process::Command;
use std::io::{self, Write};
use whoami::{self, Platform};

pub fn input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().expect("Failed to flush stdout");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	input.clone().trim().to_string()
}
