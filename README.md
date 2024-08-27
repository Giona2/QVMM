# Qemu (based) Virtual Machine Manager
## Description
The Qemu Virtual Machine Manager is meant to act as a frontend for users who want to use Qemu but need an easier format to work with. Although it's still a cli tool, it's syntax is much easier to get used to for people new to the Qemu scene.
## Install
As of now, your only option is to build it from source code. You'll need to install the rust coding language to accomplish this.
```bash
git clone https://Giona2/QVMM.git
cd QVMM/
cargo build --release
```
The finished binary will be held in the target/release/ directory (``` ./QVMM/target/release/qvmm ```)
