from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription([
        Node(
            package='{{project_name}}_bringup',
            executable='talker',
            name='talker',
            output='screen',
        ),
    ])
