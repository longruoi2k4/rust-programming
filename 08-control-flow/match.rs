fn main() {
    let number = 3;

    match number {
        1 => println!("Một"),
        2 => println!("Hai"),
        3 => println!("Ba"),
        _ => println!("Số khác"), // _ là wildcard, dùng để bao phủ tất cả các trường hợp còn lại
    }
}