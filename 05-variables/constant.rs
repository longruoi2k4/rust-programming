// Các biến toàn cục được khai báo bên ngoài tất cả các phạm vi khác.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Truy cập hằng số trong một hàm nào đó
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("Ngôn ngữ là {}", LANGUAGE);
    println!("Ngưỡng là {}", THRESHOLD);
    println!("{} là {}", n, if is_big(n) { "lớn" } else { "nhỏ" });
}
