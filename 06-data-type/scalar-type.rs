fn main() {
    // Kiểu số nguyên
    let x: i32 = 42; // Số nguyên có dấu, 32 bit
    let y: u8 = 255; // Số nguyên không dấu, 8 bit
    println!("x = {}, y = {}", x, y);

    // Kiểu boolean
    let is_greater: bool = 10 > 5; // Kết quả là true
    if is_greater {
        println!("10 lớn hơn 5!");
    } else {
        println!("10 không lớn hơn 5!");
    }

    //Kiểu characters
    let c1: char = 'A'; // Khai báo kiểu `char` cụ thể
    let c2 = '😊';       // Suy luận kiểu `char` tự động
    println!("Ký tự thứ nhất: {}", c1);
    println!("Ký tự thứ hai: {}", c2);
}