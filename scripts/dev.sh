#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; CARGO_TARGET_DIR=../target-trunk trunk serve' & \
 bash -c 'cd server; cargo watch -- cargo run')
