#!/usr/bin/env python3
"""Perception node for {{project_name}}"""

import rclpy
from rclpy.node import Node
from std_msgs.msg import String
from sensor_msgs.msg import Image, PointCloud2
from vision_msgs.msg import Detection2DArray


class PerceptionNode(Node):
    def __init__(self):
        super().__init__('perception_node')
        
        # Publishers
        self.detection_pub = self.create_publisher(
            Detection2DArray, 
            'detections', 
            10
        )
        
        # Subscribers
        self.image_sub = self.create_subscription(
            Image,
            '/camera/image_raw',
            self.image_callback,
            10
        )
        
        self.pointcloud_sub = self.create_subscription(
            PointCloud2,
            '/lidar/points',
            self.pointcloud_callback,
            10
        )
        
        self.get_logger().info('Perception node started')

    def image_callback(self, msg):
        self.get_logger().debug(f'Received image: {msg.width}x{msg.height}')

    def pointcloud_callback(self, msg):
        self.get_logger().debug(f'Received pointcloud: {msg.width} points')


def main(args=None):
    rclpy.init(args=args)
    
    node = PerceptionNode()
    
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()