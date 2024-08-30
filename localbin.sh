#!/bin/sh
set -eux

cargo build --release && cp --force target/release/info_thing ~/.local/bin
