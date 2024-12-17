use whoami;


pub fn get_data_dir() -> String {
    return format!("/home/{}/.local/share/qemu_vmm", whoami::username());
}
