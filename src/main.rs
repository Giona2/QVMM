use std::env;
use std::fs;
use whoami;


mod commands;
mod utils;

const DATA_DIR: &str = "/home/jonah/.local/share/qemu_vmm";
fn get_date_dir() -> String {
    return format!("/home/{}/.local/share/qemu_vmm", whoami::username());
}

fn main() {
    if !fs::exists(get_date_dir()).unwrap() {
        fs::create_dir(get_date_dir())
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
