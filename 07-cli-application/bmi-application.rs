use std::io;

// Hàm tính toán chỉ số BMI
fn calculate_bmi(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}

// Hàm nhập dữ liệu từ người dùng
fn get_input(prompt: &str) -> f64 {
		// In ra yêu cầu nhập
    println!("{}", prompt);
		
		// Nhập và lưu thành chuỗi
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Chuyển chuỗi nhập vào thành f64 và trả về
    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    // Nhập chiều cao và cân nặng từ người dùng
    let height = get_input("Enter your height in meters :");
    let weight = get_input("Enter your weight in kilograms :");

    // Tính chỉ số BMI
    let bmi = calculate_bmi(height, weight);

    // In ra kết quả BMI và phân loại
    println!("Your BMI is: {:.2}", bmi);
		// Phân loại
    if bmi < 18.5 {
        println!("You are underweight.");
    } else if bmi < 24.9 {
        println!("You have a normal weight.");
    } else if bmi < 29.9 {
        println!("You are overweight.");
    } else if bmi < 30.0 {
        println!("You are obese.");
    } else {
		println!("You are extremely obese.");
	}
}