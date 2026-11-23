from setuptools import find_packages, setup

package_name = 'spot_vision'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='mannm',
    maintainer_email='mannm@todo.todo',
    description='TODO: Package description',
    license='Apache-2.0',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'line_follower = spot_vision.line_follower:main',
            'aruco_detector = spot_vision.aruco_detector:main',
            'yolo_detector = spot_vision.yolo_detector:main',
        ],
    },
)
