#! /bin/bash

rustup install nightly
rustup override set nightly
cargo build --release
