#!/bin/bash

# This script installs the necessary components to begin working on this operating system
# Assumes that rust and cargo are already installed using rustup

rustup component add rust-src
rustup component add llvm-tools-preview
cargo install cargo-xbuild
cargo install bootimage --version "^0.7.3"