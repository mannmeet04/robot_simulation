import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image, CameraInfo
from cv_bridge import CvBridge
import cv2
import numpy as np

MARKER_SIZE = 0.3  # Kantenlänge des Markers in Metern


class ArucoDetector(Node):

    def __init__(self):
        super().__init__('aruco_detector')
        self.bridge = CvBridge() # ros und opencv brücke
        self.camera_matrix = None 
        self.dist_coeffs = None # Verzerrungskoeffizienten

        dictionary = cv2.aruco.getPredefinedDictionary(cv2.aruco.DICT_6X6_50) 
        self.detector = cv2.aruco.ArucoDetector(dictionary, cv2.aruco.DetectorParameters()) 

        
        h = MARKER_SIZE / 2 # die mitte vom rand ist 0,15 m
        self.object_points = np.array([
            [-h,  h, 0],
            [ h,  h, 0],
            [ h, -h, 0],
            [-h, -h, 0],
        ], dtype=np.float32)

        self.create_subscription(CameraInfo, '/Spot/kinect_color/camera_info', self.info_callback, 10)
        self.create_subscription(Image, '/Spot/kinect_color/image_color', self.image_callback, 10)

    def info_callback(self, msg):
        self.camera_matrix = np.array(msg.k, dtype=np.float64).reshape(3, 3) # kamera infos in 3x3 matrix
        if len(msg.d) > 0:
            self.dist_coeffs = np.array(msg.d, dtype=np.float64)
        else:
            self.dist_coeffs = np.zeros(5) # wenn msg leer dann 5 nullen 

    def image_callback(self, msg):
        if self.camera_matrix is None:
            self.get_logger().info('Warte auf camera_info ...', throttle_duration_sec=2.0)
            return

        frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
        corners, ids, _ = self.detector.detectMarkers(frame) # ecken ids suche 

        if ids is not None:
            cv2.aruco.drawDetectedMarkers(frame, corners, ids) # bei schilder dann nr und grünen rahmen machen 
            for marker_corners, marker_id in zip(corners, ids.flatten()):
                ok, rvec, tvec = cv2.solvePnP( # wie quadratisch das schild ist , wie weit das schild ist
                    self.object_points, marker_corners.reshape(4, 2), # ecken 
                    self.camera_matrix, self.dist_coeffs, # zoom und verzerrung
                    flags=cv2.SOLVEPNP_IPPE_SQUARE) # rechenverfahren
                if ok:
                    x, y, z = tvec.flatten() # pos von schild
                    distance = np.linalg.norm(tvec) # entfernung berechnet
                    self.get_logger().info(
                        f'Marker ID {marker_id}: Entfernung {distance:.2f} m '
                        f'(x={x:.2f}, y={y:.2f}, z={z:.2f})')
                    cv2.drawFrameAxes(frame, self.camera_matrix, self.dist_coeffs,
                                      rvec, tvec, MARKER_SIZE / 2) # 3 pfeile im bild

        cv2.imshow('ArUco Detector', frame) # Bild anzeigen
        cv2.waitKey(1)


def main(args=None):
    rclpy.init(args=args)
    node = ArucoDetector()
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