mod a {
    pub fn ham_a() {
        println!("Gọi hàm `a::ham_a()`");
    }

    pub mod b {
        pub fn ham_b() {
            println!("Gọi hàm `a::b::ham_b()`");
        }

        pub fn goi_ham() {
            println!("Gọi hàm `a::b::goi_ham()`");
        }
    }

    pub fn ham_c() {
        println!("Gọi hàm `a::ham_c()`");
        self::ham_a(); // Gọi `ham_a` trong cùng module `a` bằng `self`
        self::b::ham_b(); // Gọi `ham_b` trong module con `b` bằng `self`
    }
}

fn main() {
    // Gọi hàm từ module `a`
    a::ham_a(); // Gọi trực tiếp hàm `ham_a` từ module `a`

    // Gọi hàm từ module `b` thông qua `super`
    a::b::ham_b(); // Gọi hàm `ham_b` từ module con `b` của `a`

    // Gọi hàm từ module `a` thông qua `self`
    a::ham_c(); // Gọi hàm `ham_c` trong module `a`, trong đó sử dụng `self` và `super`
}
