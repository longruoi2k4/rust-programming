// Định nghĩa struct Rectangle với hai trường: width và height
struct Rectangle {
    width: u32,
    height: u32,
}

// Định nghĩa phương thức tính diện tích cho Rectangle trong khối impl
impl Rectangle {
    // Phương thức area trả về diện tích của hình chữ nhật
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Tạo một instance của Rectangle
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Gọi phương thức area để tính diện tích của hình chữ nhật
    println!("The area of the rectangle is {} square pixels.", rect.area());
}