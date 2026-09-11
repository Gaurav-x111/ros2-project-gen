#!/usr/bin/env bash
# Generate the single README GIF — records the full workflow and renders it.
#
# The GIF walks through: --version → list → doctor → init → colcon build
# on a multi-language workspace (minimal_ros2 template).
#
# Prerequisites:
#   pip install asciinema
#   cargo install --git https://github.com/asciinema/agg.git --locked
#   ROS 2 installed (jazzy/humble) at /opt/ros
#   ros2-project-gen installed (cargo install --path .)
#
# Usage:
#   ./demo/generate.sh
#
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

for tool in asciinema agg; do
  if ! command -v "$tool" &>/dev/null; then
    echo "Error: $tool not found."
    echo "  asciinema: pip install asciinema"
    echo "  agg:       cargo install --git https://github.com/asciinema/agg.git --locked"
    exit 1
  fi
done

cd "$SCRIPT_DIR"
WORKDIR="${WORKDIR:-$(mktemp -d)}"
trap 'rm -rf "$WORKDIR"' EXIT

echo "▶ Recording demo workflow ..."
COLUMNS=100 LINES=24 asciinema rec demo.cast --command "DEMO_DIR=$WORKDIR bash record.sh" --overwrite

echo "▶ Rendering demo.gif ..."
agg --theme dracula --font-size 18 --cols 100 \
    --idle-time-limit 2 --last-frame-duration 4 \
    demo.cast demo.gif

echo "  ✓ demo.gif ($(du -h demo.gif | cut -f1))"
file demo.gif