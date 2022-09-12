#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
trunk build --release --public-url /assets/
popd

pushd server
cargo run --release
popd
