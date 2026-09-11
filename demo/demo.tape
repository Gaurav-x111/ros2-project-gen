# demo.tape — record with: vhs demo.tape
# Requires vhs (https://github.com/charmbracelet/vhs) + a working Chromium.
#
# Note: this .tape file is the canonical, human-readable source for the demo.
# The committed `demo.gif` is generated on a machine where VHS can render;
# on this environment VHS cannot drive the snap Chromium, so the GIF is
# produced instead via the asciinema driver (see demo/record.sh).

Output demo.gif

Set FontSize 18
Set Width 1200
Set Height 700
Set Theme "Dracula"
Set Padding 20
Set TypingSpeed 60ms
Set Shell "bash"

# Let the terminal settle before recording starts
Sleep 500ms

# Show the tool exists
Type "ros2-project-gen --version"
Sleep 500ms
Enter
Sleep 1s

# List available templates
Type "ros2-project-gen list"
Sleep 500ms
Enter
Sleep 3s

# Check environment before building
Type "ros2-project-gen doctor"
Sleep 500ms
Enter
Sleep 3s

# The main event: generate a full multi-language workspace
Type "ros2-project-gen init my_robot --template minimal_ros2"
Sleep 500ms
Enter
Sleep 2s

Type "cd my_robot/ros2_ws && colcon build"
Sleep 500ms
Enter
Sleep 5s

# Hold the final frame so viewers can read it
Sleep 3s