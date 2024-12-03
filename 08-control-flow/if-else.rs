fn main() {
    let number = 7;

    // Sử dụng if-else để kiểm tra giá trị của biến
    if number < 0 {
        println!("{} là số âm.", number);
    } else if number > 0 {
        println!("{} là số dương.", number);
    } else {
        println!("{} là số không.", number);
    }

    // Sử dụng if-else như một biểu thức để gán giá trị
    let new_number =
        if number < 10 && number > -10 {
            println!("Và đây là một số nhỏ, nhân lên gấp mười lần.");
            number * 10 // Trả về một số nguyên
        } else {
            println!("Và đây là một số lớn, chia đôi số này.");
            number / 2 // Trả về cùng kiểu dữ liệu (i32)
        };

    println!("Số ban đầu: {}, Kết quả: {}", number, new_number);
}