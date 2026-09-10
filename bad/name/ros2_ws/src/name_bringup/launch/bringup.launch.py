from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='name_bringup',
            executable='bringup_node',
            name='bringup_node',
            output='screen',
        ),
    ])