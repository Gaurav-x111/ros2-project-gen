from launch import LaunchDescription
from launch.actions import ExecuteProcess


def generate_launch_description():
    # The perception node is an ament_python package: its entry-point script
    # is installed into bin/, which `Node()` can't resolve (it looks in
    # lib/<pkg>/). Run it via ExecuteProcess instead.
    return LaunchDescription([
        ExecuteProcess(
            cmd=['perception_node'],
            output='screen',
        ),
    ])