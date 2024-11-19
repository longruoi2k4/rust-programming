fn main() {
    let s1 = String::from("Hello, heap!");

    let s2 = s1; // ownership được chuyển từ s1 sang s2
    println!("{}", s1); // Lỗi: s1 không còn quyền sở hữu
// Cách khắc phục
//  1. Sử dụng clone     
//    let s2 = s1.clone(); // Sao chép giá trị
//    println!("{}", s1);  // Bây giờ vẫn có thể sử dụng s1
//  2. Sử dụng reference
//    let s2 = &s1; // s2 là một tham chiếu đến s1
//    println!("{}", s1); // s1 vẫn có thể được sử dụng
//    println!("{}", s2); // s2 là một tham chiếu
}