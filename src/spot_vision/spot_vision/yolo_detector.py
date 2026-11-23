import os
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from cv_bridge import CvBridge
import cv2
from ultralytics import YOLO


class YoloDetector(Node):

    def __init__(self):
        super().__init__('yolo_detector')


        self.declare_parameter('image_topic', '/SpotArm/gripper_camera/image_color')
        self.declare_parameter('confidence', 0.5) # mindeste sicherheit bei der erkennung 
        self.declare_parameter('model_path',
                               os.path.expanduser('~/ros2_ws/models/yolov8n_openvino_model'))

        # Modell laden
        model_path = self.get_parameter('model_path').value
        self.get_logger().info(f'Lade Modell: {model_path}')
        self.model = YOLO(model_path, task='detect')

        self.bridge = CvBridge()
        topic = self.get_parameter('image_topic').value
        self.create_subscription(Image, topic, self.image_callback, 10)
        self.get_logger().info(f'Höre auf {topic}')

    def image_callback(self, msg):
        frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')

        conf = self.get_parameter('confidence').value

        # YOLO ausführen
        results = self.model(frame, conf=conf, verbose=False)
        result = results[0]

        # jedes erkannte Objekt ausgeben
        for box in result.boxes:
            name = self.model.names[int(box.cls[0])]
            score = float(box.conf[0])
            x1, y1, x2, y2 = [int(v) for v in box.xyxy[0].tolist()]
            self.get_logger().info(
                f'{name}: {score:.2f} bei ({x1}, {y1}) - ({x2}, {y2})')

        # Bild mit Rechtecken anzeigen
        cv2.imshow('YOLO Detector', result.plot())
        cv2.waitKey(1)


def main(args=None):
    rclpy.init(args=args)
    node = YoloDetector()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        cv2.destroyAllWindows()
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()