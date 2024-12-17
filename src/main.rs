use std::env;
use std::fs;

mod commands;
mod utils;
pub mod data;
use data::get_data_dir;


fn main() {
    if !fs::exists(get_data_dir()).unwrap() {
        fs::create_dir(get_data_dir())
            .expect("Failed to make the app data folder");
    }

	let mut args = env::args().collect::<Vec<String>>(); args.push("".to_string());
	match args[1].as_str() {
		"new"           => commands::new(),
		"list"          => commands::list(),
		"run"           => commands::run(None),
		"remove"        => commands::remove(None),
		"rename"        => commands::rename(None),
		"create-event"  => commands::create_event(None),
		"revert-back"   => commands::revert_back(None),
		"show-timeline" => commands::show_timeline(None),
		"help"          => commands::help(),
		""              => commands::run_continuous(),
		_               => commands::help(),
	}
}
