#! /usr/bin/bash

echo
echo "---------------------"
echo "starts install script"
echo "---------------------"
echo
rustup component add rust-analyzer llvm-tools
rustup target add thumbv7em-none-eabihf
cargo install bacon cargo-binutils cargo-binstall
cargo binstall probe-rs-tools -y
echo
echo "---------------------"
echo "install script ends"
echo "---------------------"
echo
