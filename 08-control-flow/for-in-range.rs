fn main() {
    let mut tong = 0;

    // Sử dụng vòng lặp `for in` để lặp qua các số từ 1 đến 100
    for i in 1..=100 {
        tong += i; // Cộng dồn giá trị của i vào tổng
    }

    println!("Tổng các số từ 1 đến 100 là: {}", tong);
}