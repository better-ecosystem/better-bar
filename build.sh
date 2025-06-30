#!/bin/bash

# Build and install to local/bin

# build
echo "<--------- Building better-bar --------->"
cargo build --release
echo "<--------- Build finished --------->"

# install
echo "<--------- Installing better-bar --------->"
cp target/release/better-bar ~/.local/bin/
echo "<--------- Installed better-bar --------->"

