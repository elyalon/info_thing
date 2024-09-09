#!/bin/sh
set -eux
cd "$(realpath "$(dirname $0)")"
cargo build --release && mkdir -p ~/.local/bin && cp "./target/release/info_thing" ~/.local/bin
