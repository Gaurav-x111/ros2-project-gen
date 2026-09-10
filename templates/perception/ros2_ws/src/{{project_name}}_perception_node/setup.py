from setuptools import setup, find_packages

package_name = '{{project_name}}_perception_node'

setup(
    name=package_name,
    version='0.1.0',
    packages=find_packages(),
    package_dir={'': '.'},
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='user',
    maintainer_email='user@example.com',
    description='Perception node for {{project_name}}',
    license='MIT',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'perception_node = perception_node:main',
        ],
    },
)