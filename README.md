# Qemu (based) Virtual Machine Manager
**Note:** This currently only works and been tested on Debian based GNU/Linux operating systems and only supports x86_64 ISOs

## Description
The Qemu Virtual Machine Manager is meant to act as a frontend for users who want to use Qemu but need an easier format to work with. Although it's still a cli tool, it's syntax is much easier to get used to for people new to the Qemu scene.

## Install
### Dependencies
You will need to install a few dependencies before you can get QVMM to work, namely:
**Note**: If you run the install script, these dependencies will be installed automatically
- qemu-utils
- qemu-system-x86
- qemu-system-gui
### Build from source
**Note**: This install script only works for Debian based systems with apt installed as the primary package manager
As of now, your only option is to build it from source code. You'll need to install the rust compiler to accomplish this. This install will also require some dependencies itself. Namely...
- git
- bash
Clone the repository from github...
```bash
git clone https://github.com/Giona2/QVMM.git
```
and run the install script...
```bash
cd QVMM
bash ./install.sh
```
The finished binary will be held in the ~/.local/bin folder

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
The final file directory should look something like this:
~/.local/share/qvmm/VM_Name
```
|- config.yaml
|- disks/
   |- 0_current.qcow2
```
The config.yaml file in future iterations will contain various information not defined by default in the VM's disk. At the moment it just tells the virtual machine how much RAM it's allowed use
### Timeline Feature
To emulate the snapshot feature some other VMs use, I attempted to implement my own way of doing something similar.
You may notice when you run ```qvmm help``` you'll see a few options, namely...
```bash
qvmm create-event
qvmm revert-back
qvmm show-timeline
```
These are the commands that run the timeline feature. The timeline feature can be represented as a timeline (obviously) where you can mark the current state of your virtual machine as an event in the timeline with the ```qvmm create-event``` command. If you want to move back to the last event on the timeline, run ```qvmm revert-back```.
Note that you can only go back to the most recent event on the timeline.
If you want to see the timeline, run ```qvmm show-timeline```. It will display all the events you created in chronological order.
**Warning**: After you run the revert-back command, you will not be able to undo this action.
