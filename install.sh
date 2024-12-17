#!/bin/bash

# QVMM Dependencies 
sudo apt install qemu-utils
sudo apt install qemu-system-x86
sudo apt install qemu-system-gui

# Build the rust project and move the finished binary to ~/.local/bin/
cargo build --release
mv target/release/qvmm ~/.local/bin/

