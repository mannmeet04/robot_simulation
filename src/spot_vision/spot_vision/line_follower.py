import time
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from geometry_msgs.msg import Twist
from cv_bridge import CvBridge
import cv2
import numpy as np


LOWER_RED_1 = np.array([0, 100, 70])
UPPER_RED_1 = np.array([10, 255, 255])
LOWER_RED_2 = np.array([170, 100, 70])
UPPER_RED_2 = np.array([179, 255, 255])


class LineFollower(Node):

    def __init__(self):
        super().__init__('line_follower')

        # Reglerwerte als Parameter, damit man sie beim Start ändern kann
        self.declare_parameter('kp', 1.0) # p x fehler 
        self.declare_parameter('ki', 0.0) 
        self.declare_parameter('kd', 0.1) # d x fehler, spot auf dem weg zu linie und nimmt anpassung bisschen zurück
        self.declare_parameter('max_angular', 0.8)
        self.declare_parameter('max_linear', 0.4)

        self.bridge = CvBridge() # ros bild -> opencv
        self.kernel = np.ones((5, 5), np.uint8) # für entfernung von kleinen roten flecken 
        self.cmd_pub = self.create_publisher(Twist, '/cmd_vel', 10)
        self.create_subscription(Image, '/Spot/kinect_color/image_color', self.image_callback, 10)
        self.reset_pid()

    def p(self, name):
        return self.get_parameter(name).value

    def reset_pid(self):
        self.integral = 0.0
        self.prev_error = 0.0
        self.prev_time = None

    def find_line_points(self, mask):
        """In mehreren Bildzeilen die Mitte der roten Pixel suchen."""
        h, _ = mask.shape 
        points = []
        for y in range(int(h * 0.5), int(h * 0.95), int(h * 0.05)):
            xs = np.where(mask[y] > 0)[0] # spaltennummer der roten pixel speichern 
            if len(xs) > 0:
                points.append((int(xs.mean()), y)) # mitte der linie berechnen 
        return points 

    def image_callback(self, msg):
        frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
        h, w, _ = frame.shape

        # rote pixel finden 
        hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
        mask = cv2.inRange(hsv, LOWER_RED_1, UPPER_RED_1) | cv2.inRange(hsv, LOWER_RED_2, UPPER_RED_2) # maske für rote pixel
        mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, self.kernel) # krümmelentfernung

        # punkte holen 
        points = self.find_line_points(mask)
        cmd = Twist()

        if len(points) < 2:
            # Linie verloren -> anhalten
            self.get_logger().info('Linie nicht gefunden - Stopp', throttle_duration_sec=1.0)
            self.reset_pid()
        else:
            # 3. Fehler: Abstand der Linie zur Bildmitte, skaliert auf -1 ... +1
            line_x = np.mean([pt[0] for pt in points]) # x-wert der linie nehmen
            error = (line_x - w / 2) / (w / 2) # fehler berechnen

            # 4. PID-Regler
            now = time.monotonic() # zeit merken
            if self.prev_time is None:
                derivative = 0.0
                dt = 0.0
            else:
                # wie schnell sich der fehler ändert 
                dt = max(now - self.prev_time, 1e-3) # ms sekunde daraufrechnen da der größere wert gebraucht wird
                derivative = (error - self.prev_error) / dt # änderung der fehler 
            self.integral = float(np.clip(self.integral + error * dt, -1.0, 1.0))  # summe der fehler zwischen -1 und 1 
            output = self.p('kp') * error + self.p('ki') * self.integral + self.p('kd') * derivative
            self.prev_error = error
            self.prev_time = now

            # positives angular.z = links, daher Minus
            max_ang = self.p('max_angular')
            cmd.angular.z = float(np.clip(-output, -max_ang, max_ang))
            # in Kurven (großer Fehler) langsamer fahren
            cmd.linear.x = float(self.p('max_linear') * (1.0 - 0.5 * min(abs(error), 1.0)))

            for pt in points:
                cv2.circle(frame, pt, 5, (0, 255, 0), -1) # grüne Punkte für die Linienmitte
            cv2.putText(frame, f'error={error:+.2f}', (10, 30),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.8, (255, 255, 255), 2)

        self.cmd_pub.publish(cmd)

        # Anzeige: Bildmitte als blaue Linie
        cv2.line(frame, (w // 2, 0), (w // 2, h), (255, 0, 0), 1)
        cv2.imshow('Line Follower', frame)
        cv2.imshow('Mask', mask)
        cv2.waitKey(1)

    def stop(self):
        self.cmd_pub.publish(Twist())


def main(args=None):
    rclpy.init(args=args)
    node = LineFollower()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        try:
            node.stop()  # Spot läuft sonst mit dem letzten Befehl weiter
        except Exception:
            pass
        cv2.destroyAllWindows()
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()