mod my_mod {
    pub fn ham_a() {
        println!("Gọi `my_mod::ham_a()`");
    }

    pub fn ham_b() {
        println!("Gọi `my_mod::ham_b()`");
    }

    pub mod nested {
        pub fn ham_nested() {
            println!("Gọi `my_mod::nested::ham_nested()`");
        }

        pub fn ham_nested_khac() {
            println!("Gọi `my_mod::nested::ham_nested_khac()`");
        }
    }
}

// Nhập một item cụ thể
use my_mod::ham_a;

// Nhập với bí danh
use my_mod::ham_b as ham_b_alias;

// Nhập item từ module lồng
use my_mod::nested::{ham_nested, ham_nested_khac};

fn main() {
    // Gọi hàm đã được nhập bằng cách 1
    ham_a();

    // Gọi hàm đã được nhập với bí danh
    ham_b_alias();

    // Gọi các hàm đã được nhập từ module lồng
    ham_nested();
    ham_nested_khac();
}
