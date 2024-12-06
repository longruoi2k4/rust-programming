mod my_mod {
    // Hàm riêng tư, chỉ có thể truy cập bên trong module `my_mod`.
    fn ham_rieng_tu() {
        println!("Gọi `my_mod::ham_rieng_tu()`");
    }

    // Hàm công khai, có thể truy cập từ bên ngoài module `my_mod`.
    pub fn ham_cong_khai() {
        println!("Gọi `my_mod::ham_cong_khai()`");
    }

    // Hàm công khai gián tiếp gọi hàm riêng tư.
    pub fn goi_ham_rieng_tu() {
        print!("Gọi `my_mod::goi_ham_rieng_tu()`, điều này\n> ");
        ham_rieng_tu();
    }

    // Module lồng, với một hàm công khai.
    pub mod long {
        pub fn ham() {
            println!("Gọi `my_mod::long::ham()`");
        }
    }
}

fn main() {
    // Gọi hàm công khai trong module `my_mod`.
    my_mod::ham_cong_khai();

    // Gọi hàm công khai gián tiếp gọi hàm riêng tư.
    my_mod::goi_ham_rieng_tu();

    // Gọi hàm công khai trong module lồng.
    my_mod::long::ham();

    // Lỗi! Không thể gọi hàm riêng tư từ bên ngoài module.
    // my_mod::ham_rieng_tu();
}
