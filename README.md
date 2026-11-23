# ros2_ws – Robotik Praktikum

ROS 2 Workspace fürs Robotik Praktikum. Läuft unter Ubuntu 22.04 mit ROS 2 Humble und Webots.

## Pakete

- `turtle_circle_pkg` – Turtle fährt in turtlesim eine Runde im Kreis und hält dann an
- `spot_vision` – eigene Nodes für den Spot in Webots:
  - `line_follower` – folgt einer roten Linie (PID-Regler, schickt `/cmd_vel`)
  - `aruco_detector` – erkennt ArUco-Marker (6x6, 30 cm) und gibt die Entfernung aus
  - `yolo_detector` – Objekterkennung mit YOLOv8n auf der Greiferkamera
- `webots_ros2_spot`, `webots_spot_msgs`, `webots_spot_teleop`, `webots_ros2` – Simulation und Teleop (übernommen, nicht von mir)

Das YOLO-Modell liegt unter `models/` (auch als OpenVINO-Export).

## Bauen

```bash
cd ~/ros2_ws
colcon build --symlink-install
source install/setup.bash
```

Für `yolo_detector` braucht man zusätzlich `pip install ultralytics`.

## Starten

Simulation:
```bash
ros2 launch webots_ros2_spot spot_launch.py
```

Dann in einem zweiten Terminal z.B.:
```bash
ros2 run spot_vision line_follower
ros2 run spot_vision aruco_detector
ros2 run spot_vision yolo_detector
```

Reglerwerte kann man beim Start ändern:
```bash
ros2 run spot_vision line_follower --ros-args -p kp:=1.2 -p kd:=0.2
```

Turtlesim:
```bash
ros2 run turtlesim turtlesim_node
ros2 run turtle_circle_pkg turtle_circle
```

Spot per Tastatur steuern:
```bash
ros2 run webots_spot_teleop keyboard
```
