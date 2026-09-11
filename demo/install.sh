#!/usr/bin/env bash
# Driver for `asciinema rec` — shows installing from crates.io
# and confirming the version. First `cargo uninstall ros2-project-gen`
# (outside the recording) so the install shows the real "Installed
# package" flow. The dependency compile wall is cut at render time
# with `agg --select`.
set -o pipefail

step() {
  printf '\x1b[1m$ %s\x1b[0m\n' "$*"
  sleep 0.5
  "$@"
  sleep 0.3
}

step cargo install ros2-project-gen
sleep 0.6
step ros2-project-gen --version
sleep 0.8