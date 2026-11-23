import math

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from turtlesim.msg import Pose


class TurtleCircle(Node):

    def __init__(self):
        super().__init__('turtle_circle')

        self.linear_velocity = 2.0   
        self.angular_velocity = 1.0  

        self.cmd_vel_pub = self.create_publisher(Twist, '/turtle1/cmd_vel', 10)
        self.pose_sub = self.create_subscription(Pose, '/turtle1/pose', self.pose_callback, 10)
        self.timer = self.create_timer(0.1, self.timer_callback)

        self.last_theta = None   
        self.turned_angle = 0.0  
        self.lap_finished = False

        self.get_logger().info('Starting to drive one lap')

    def timer_callback(self):
        if self.lap_finished:
            return

        msg = Twist()
        msg.linear.x = self.linear_velocity
        msg.angular.z = self.angular_velocity
        self.cmd_vel_pub.publish(msg)

    def pose_callback(self, msg: Pose):
        if self.lap_finished:
            return

        if self.last_theta is None:
            self.last_theta = msg.theta
            self.get_logger().info(f'Start pose: x={msg.x:.2f} y={msg.y:.2f} theta={msg.theta:.2f}')
            return

        delta = msg.theta - self.last_theta

        if delta > math.pi:
            delta -= 2 * math.pi
        elif delta < -math.pi:
            delta += 2 * math.pi

        self.turned_angle += abs(delta)
        self.last_theta = msg.theta

        if self.turned_angle >= 2 * math.pi:
            self.stop_turtle(msg)

    def stop_turtle(self, pose: Pose):
        self.lap_finished = True
        self.timer.cancel()

        self.cmd_vel_pub.publish(Twist())

        self.get_logger().info(f'Lap finished at x={pose.x:.2f} y={pose.y:.2f} theta={pose.theta:.2f}')


def main(args=None):
    rclpy.init(args=args)
    node = TurtleCircle()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.try_shutdown()


if __name__ == '__main__':
    main()