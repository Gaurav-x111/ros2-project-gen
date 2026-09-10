cmake_minimum_required(VERSION 3.8)
project({{project_name}}_perception_node)

find_package(ament_cmake REQUIRED)

ament_python_install_package(perception_node)

ament_package()