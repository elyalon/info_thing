#!/bin/sh
project_dir="$(realpath "$(dirname $0)")"
cargo build --release && cp "$project_dir/target/release/info_thing" ~/.local/bin/
