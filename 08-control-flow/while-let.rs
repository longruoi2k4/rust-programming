fn main() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Lớn hơn 9, thoát!");
            optional = None; // Đặt `optional` thành `None` để thoát vòng lặp
        } else {
            println!("`i` là `{:?}`. Thử lại.", i);
            optional = Some(i + 1); // Cập nhật giá trị của `optional`
        }
    }
}