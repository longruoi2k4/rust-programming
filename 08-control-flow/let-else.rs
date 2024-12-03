fn main() {
    let some_number: Option<i32> = Some(42);
    
    // Sử dụng `let-else` để kiểm tra và phân nhánh
    let Some(number) = some_number else {
        println!("Không có giá trị, kết thúc!");
        return; // Kết thúc hàm nếu không có giá trị
    };
    
    // Tiếp tục nếu có giá trị
    println!("Số tìm thấy: {}", number);
}