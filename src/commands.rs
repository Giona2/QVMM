use std::{ffi::OsString, fs};
use std::io::Error;
use std::process::Command;
use utils::system::terminal;
use yaml_rust::YamlLoader;

use crate::DATA_DIR;


pub fn run_continuous() { let selected_vm: &str = &terminal::input("Selected Virtual Machine Name > "); loop {
	let option: &str = &terminal::input(&(selected_vm.to_string() + " > ")); match option {
		"run"			=> run(Some(selected_vm)),
		"remove"		=> {remove(Some(selected_vm)); break;}
		"create-event"	=> create_event(Some(selected_vm)),
		"revert-back"	=> revert_back(Some(selected_vm)),
		"show-timeline"	=> show_timeline(Some(selected_vm)),
		"exit"			=> break,
		"clear"			=> continuous_mode::clear_terminal(),
		"help" | _		=> continuous_mode::help(selected_vm)
	}
}}

mod continuous_mode {
    use std::process::Command;

	pub fn help(selected_vm: &str) {
		println!();
		println!("{selected_vm} > run           | run {selected_vm}");
		println!("{selected_vm} > remove        | remove {selected_vm} and exit continuous mode");
		println!("{selected_vm} > create-event  | creates a new event in {selected_vm}'s timeline");
		println!("{selected_vm} > revert-back   | reverts back to the last event");
		println!("{selected_vm} > show-timeline | displays {selected_vm}'s timeline");
		println!("{selected_vm} > clear         | clear the terminal");
		println!("{selected_vm} > exit          | exit continuous mode");
		println!();
	}

	pub fn clear_terminal() {
		Command::new("clear").status().unwrap();
	}
}

pub fn new() {
	let name: &str			 = &terminal::input("Name           > ");
	let storage_amount: &str = &terminal::input("Storage Amount > ");
	let ram_amount: &str	 = &terminal::input("RAM Amount     > ");

	fs::create_dir(DATA_DIR.to_string() + "/" + name)
		.expect("Failed to create directory for the VM");
	fs::create_dir(DATA_DIR.to_string() + "/" + name + "/disk")
		.expect("Failed to create disk directory");
	Command::new("qemu-img")
		.args(["create", "-f", "qcow2", &(DATA_DIR.to_string() + "/" + name + "/disk/0_current.qcow2"), storage_amount])
		.status().unwrap();
	fs::write(DATA_DIR.to_string() + "/" + name + "/config.yaml", format!("ram_amount: \"{ram_amount}\""))
		.expect("Failed to create config.yaml file");
}

pub fn list() {
	for vm_name in fs::read_dir(DATA_DIR.to_string()).unwrap() {
		println!("- {}", vm_name.unwrap().file_name().into_string().unwrap());
	}
}

pub fn run(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};
	let selected_disk: &str = &terminal::input("Boot from CD Rom? [Y/n]       > ");

	match selected_disk.to_lowercase().as_str() {
		"y" => {
			let cdrom_dir: &str = &terminal::input("    CD Rom Directory          > ");
			let yaml_config = &YamlLoader::load_from_str(&fs::read_to_string(DATA_DIR.to_string() + "/" + selected_vm + "/" + "config.yaml").unwrap()).unwrap()[0];
			Command::new("qemu-system-x86_64").args([ "-enable-kvm",
					"-hda", &(DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2"),
					"-cdrom", cdrom_dir,
					"-m", yaml_config["ram_amount"].as_str().unwrap()
				])
				.status().unwrap();
		}
		_   => {
			let yaml_config = &YamlLoader::load_from_str(&fs::read_to_string(DATA_DIR.to_string() + "/" + selected_vm + "/" + "config.yaml").unwrap()).unwrap()[0];
			Command::new("qemu-system-x86_64").args(["-enable-kvm", 
					"-hda", &(DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2"),
					"-m", yaml_config["ram_amount"].as_str().unwrap()
				])
				.status().unwrap();
		}
	}
}

pub fn remove(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};
	fs::remove_dir_all(DATA_DIR.to_string() + "/" + selected_vm)
		.expect("Failed to remove directory");
}

pub fn rename(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};
	let new_name: &str		= &terminal::input("Updated Name                  > ");
	fs::rename(DATA_DIR.to_string() + "/" + selected_vm, DATA_DIR.to_string() + "/" + new_name)
		.expect("Failed to rename the project");
}

pub fn create_event(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};
	let event_name: &str  = &terminal::input("Event Name                    > ");

	let timestamps: Vec<OsString> = fs::read_dir(DATA_DIR.to_string() + "/" + selected_vm + "/disk").unwrap()
		.collect::<Vec<Result<fs::DirEntry, Error>>>().into_iter()
		.map(|x| x.unwrap().file_name()).collect();
	let event_file_name: &str = &format!("{}_{}.qcow2", timestamps.len(), event_name);

	fs::rename(DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2", DATA_DIR.to_string() + "/" + selected_vm + "/disk/" + event_file_name)
		.expect("Failed to rename current state");

	Command::new("qemu-img").args(["create",
			"-f", "qcow2",
			"-b", &(DATA_DIR.to_string() + "/" + selected_vm + "/disk/" + event_file_name),
			"-F", "qcow2", &(DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2")
		])
		.status().unwrap();
}

pub fn revert_back(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};

	let timestamps: Vec<OsString> = fs::read_dir(DATA_DIR.to_string() + "/" + selected_vm + "/disk").unwrap()
		.collect::<Vec<Result<fs::DirEntry, Error>>>().into_iter()
		.map(|x| x.unwrap().file_name()).collect();
	let mut most_current_event: &str = ""; for timestamp in timestamps.iter() {
		if timestamp.to_str().unwrap().split("_").collect::<Vec<&str>>()[0].parse::<usize>().unwrap() == timestamps.len() - 1 {
			most_current_event = timestamp.to_str().unwrap() }}

	if timestamps.len() != 1 {
		fs::remove_file(DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2")
			.expect("Failed to remove current state");
		fs::rename(DATA_DIR.to_string() + "/" + selected_vm + "/disk/" + most_current_event, DATA_DIR.to_string() + "/" + selected_vm + "/disk/0_current.qcow2")
			.expect("Failed to rename the most current event");
	}
}

pub fn show_timeline(vm_name: Option<&str>) {
	let selected_vm: &str 	= match vm_name {
		Some(name) => name,
		None => &terminal::input("Selected Virtual Machine Name > "),};

	for event_file in fs::read_dir(DATA_DIR.to_string() + "/" + selected_vm + "/disk/").unwrap() {
		let event_file_name = event_file.unwrap().file_name();
		let (event_num, event_name) = event_file_name
			.to_str().unwrap()
			.split_once("_").unwrap();
		println!("{event_num}: {event_name}")
	}
}

pub fn help() {
	println!();
	println!("qemu-vmm               | run in continuous mode");
	println!("qemu-vmm new           | create a new VM");
	println!("qemu-vmm list          | list all existing VMs");
	println!("qemu-vmm run           | run an existing VM");
	println!("qemu-vmm remove        | remove an existing VM");
	println!("qemu-vmm rename        | rename an existing VM");
	println!("qemu-vmm create-event  | creates a new event in an existing VM's timeline");
	println!("qemu-vmm revert-back   | reverts back to the last event");
	println!("qemu-vmm show-timeline | displays the selected VM's timeline");
	println!();
}
