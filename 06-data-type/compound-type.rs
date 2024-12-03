fn main() {
    //Tuples
    let person: (&str, i32, f64) = ("Rust", 30, 1.68);
    println!("Tên: {}, Tuổi: {}, Chiều cao: {}", person.0, person.1, person.2);

    //Arrays
    //Mảng 1 chiều
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Mảng có 5 phần tử kiểu i32
    println!("Phần tử đầu tiên: {}", numbers[0]);
    println!("Phần tử cuối cùng: {}", numbers[4]);

    //Mảng 2 chiều
    let matrix = [
        [10, 20, 30], // Hàng đầu tiên
        [40, 50, 60], // Hàng thứ hai
    ];

    println!("Phần tử ở hàng 1, cột 2: {}", matrix[0][1]); // 20
    println!("Phần tử ở hàng 2, cột 3: {}", matrix[1][2]); // 60
}