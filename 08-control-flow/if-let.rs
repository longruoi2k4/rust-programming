fn main() {
    let some_number: Option<i32> = Some(42);  // Option chứa giá trị
    let no_number: Option<i32> = None;       // Option không chứa giá trị

    // Sử dụng `if let` để kiểm tra và xử lý nếu có giá trị trong `Option`
    if let Some(number) = some_number {
        println!("Đã tìm thấy số: {}", number);
    } else {
        println!("Không có số");
    }

    // Sử dụng `if let` để kiểm tra và xử lý trường hợp không có giá trị trong `Option`
    if let Some(number) = no_number {
        println!("Đã tìm thấy số: {}", number);
    } else {
        println!("Không có số");
    }
}