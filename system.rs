pub mod terminal {
	/// Contains functions that are built to run on Windows, MacOS, and Linux
	/// Any modules that do not run on any one of these systems will be specified as such
	use std::process::Command;
	use std::io::{self, Write};
	use whoami::{self, Platform};


	pub fn clear() { match whoami::platform() {
		Platform::Linux => { Command::new("clear").status().unwrap(); }
		Platform::Windows => { Command::new("cls").status().unwrap(); }
		Platform::MacOS => { Command::new("clear").status().unwrap(); }
		_ => {}
	}}

	pub fn input_parse(prompt: &str) -> Vec<String> {
		let output = input(prompt);
		let output_parsed: Vec<String> = output.split(" ").into_iter().map(|x| x.trim().to_string()).collect();
		return output_parsed
	}

	pub fn input(prompt: &str) -> String {
		print!("{}", prompt);
		io::stdout().flush().expect("Failed to flush stdout");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input.clone().trim().to_string()
	}
}

pub mod current_user {
	use whoami;
	use crate::type_traits::StringExtra;

	pub fn dir() -> String {
		String::add_str(&["/home/", &whoami::username(), "/"])
	}

	pub fn config_dir() -> String {
		dir() + ".config/"
	}
}

pub mod shell_runner {
	/// A simplified solution to rust's std::process::Command struct
	/// Creates a temporary bash file in the current working directory and runs it to grab stdout, run functionality, or both, then deletes it
	/// You may also run multiple commands in a series, as the commands slice will join using \n and write to the bash file
	/// *Only works for Linux
	use std::fs;
	use std::process::Command;

	pub fn run(commands: &[&str]) {
		fs::write("./.bash.sh", commands.join("\n"))
			.expect("Failed to create .bash.sh file");
		Command::new("./.bash.sh")
			.status().expect("Failed to run .bash.sh file");
	}
	pub fn stdout(commands: &[&str]) -> String {
		fs::write("./.bash.sh", commands.join("\n"))
			.expect("Failed to create .bash.sh file");
		let raw_out = Command::new("./.bash.sh")
			.output().expect("Failed to run .bash.sh file").stdout;
		return String::from_utf8(raw_out).unwrap()
	}
}
