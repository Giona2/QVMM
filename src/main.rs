use std::env;


mod commands;

const DATA_DIR: &str = "/home/jonah/.local/share/rust/qemu_vmm";

fn main() {
	let mut args = env::args().collect::<Vec<String>>(); args.push("".to_string());
	match args[1].as_str() {
		"new"			=> commands::new(),
		"list"			=> commands::list(),
		"run"			=> commands::run(None),
		"remove"		=> commands::remove(None),
		"rename"		=> commands::rename(None),
		"create-event"	=> commands::create_event(None),
		"revert-back"	=> commands::revert_back(None),
		"show-timeline"	=> commands::show_timeline(None),
		"help" 			=> commands::help(),
		"" 				=> commands::run_continuous(),
		_				=> commands::help(),
	}
}
