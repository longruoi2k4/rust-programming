fn main() {
    let mut i = 1;  // Khởi tạo biến đếm
    let mut tong = 0;  // Khởi tạo biến tổng

    while i <= 100 {
        tong += i;  // Cộng dồn giá trị i vào tổng
        i += 1;  // Tăng i lên 1
    }

    println!("Tổng các số từ 1 đến 100 là: {}", tong);
}