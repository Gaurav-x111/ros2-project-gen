#!/usr/bin/env bash
# Driver for `asciinema rec` — plays the demo commands with a pause between
# each echo and its execution, so the GIF reads as human typing.
# Note: no `set -e`/`set -u` here — ROS 2's setup.bash references unbound
# vars and partial installs return non-zero, which would abort the recording.
set -o pipefail

source /opt/ros/jazzy/setup.bash 2>/dev/null || true
source /opt/ros/humble/setup.bash 2>/dev/null || true

DEMO_DIR="${DEMO_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"
cd "$DEMO_DIR"
rm -rf my_robot

step() {
  printf '\x1b[1m$ %s\x1b[0m\n' "$*"
  sleep 0.5
  "$@"
  sleep 0.3
}

step ros2-project-gen --version
sleep 0.5
step ros2-project-gen list
sleep 0.5
step ros2-project-gen doctor
sleep 0.5
step ros2-project-gen init my_robot --template minimal_ros2
sleep 0.5

printf '\x1b[1m$ cd my_robot/ros2_ws\x1b[0m\n'
sleep 0.5
cd my_robot/ros2_ws
sleep 0.3

printf '\x1b[1m$ colcon build\x1b[0m\n'
sleep 0.5
colcon build
sleep 0.5