# Qemu (based) Virtual Machine Manager
**Note:** This currently only works and been tested on Debian based GNU/Linux operating systems and only supports x86_64 ISOs
## Description
The Qemu Virtual Machine Manager is meant to act as a frontend for users who want to use Qemu but need an easier format to work with. Although it's still a cli tool, it's syntax is much easier to get used to for people new to the Qemu scene.
## Install
### Dependencies
You will need to install a few dependencies before you can get QVMM to work, namely:
- qemu-utils
- qemu-system-x86
- qemu-system-gui
### Build from source
As of now, your only option is to build it from source code. You'll need to install the rust coding language to accomplish this.
```bash
git clone https://github.com/Giona2/QVMM.git
cd QVMM/
cargo build --release
```
The finished binary will be held in the target/release/ directory (```./QVMM/target/release/qvmm```)
## How It Works
As stated, it uses Qemu as a basis to operate on. QVMM only adds a bit of structure and automation to accomplish the same tasks.
When you create a new virtual machine (VM)...
```bash
qvmm new
```
it does a few things:
1. creates a new folder with the name of the VM in the application's data directory (```$HOME/.local/share/qvmm/```)
2. creates a config.yaml file in this folder
3. creates a disk directory with the qcow2 files. This allows for the timeline feature which I'll detail below
The file final file directory should look something like this:
$HOME/.local/share/qvmm/VM_Name
```
|- config.yaml
|- disks/
   |- 0_current.qcow2
```
The config.yaml file in future iterations will contain various information not defined by default in the VM's disk. At the moment it just tells the virtual machine how much RAM it's allowed use
### Timeline Feature
