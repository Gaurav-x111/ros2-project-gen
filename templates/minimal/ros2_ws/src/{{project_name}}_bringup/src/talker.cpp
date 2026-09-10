#include <chrono>
#include <memory>
#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"

class MinimalNode : public rclcpp::Node
{
public:
    MinimalNode()
    : Node("minimal_node"), count_(0)
    {
        publisher_ = this->create_publisher<std_msgs::msg::String>("chatter", 10);
        timer_ = this->create_wall_timer(
            std::chrono::milliseconds(500),
            std::bind(&MinimalNode::timer_callback, this));
        RCLCPP_INFO(this->get_logger(), "Minimal node started");
    }

private:
    void timer_callback()
    {
        auto msg = std_msgs::msg::String();
        msg.data = "Hello World! " + std::to_string(count_++);
        RCLCPP_INFO(this->get_logger(), "Publishing: '%s'", msg.data.c_str());
        publisher_->publish(msg);
    }

    rclcpp::TimerBase::SharedPtr timer_;
    rclcpp::Publisher<std_msgs::msg::String>::SharedPtr publisher_;
    size_t count_;
};

int main(int argc, char * argv[])
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<MinimalNode>());
    rclcpp::shutdown();
    return 0;
}
