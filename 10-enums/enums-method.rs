// Định nghĩa một enum với các trạng thái khác nhau
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Implement phương thức cho enum
impl TrafficLight {
    // Phương thức trả về thời gian chờ ứng với từng trạng thái đèn giao thông
    fn waiting_time(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,    // 30 giây cho đèn đỏ
            TrafficLight::Yellow => 5, // 5 giây cho đèn vàng
            TrafficLight::Green => 20, // 20 giây cho đèn xanh
        }
    }

    // Phương thức kiểm tra đèn có phải là đèn xanh hay không
    fn is_green(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;

    println!("Đèn đỏ: chờ {} giây.", red_light.waiting_time());
    println!("Đèn xanh: chờ {} giây.", green_light.waiting_time());
    
    if green_light.is_green() {
        println!("Đèn xanh, bạn có thể đi!");
    } else {
        println!("Dừng lại!");
    }
}
