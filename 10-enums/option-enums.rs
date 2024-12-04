fn process_number(number: Option<i32>) {
    match number {
        Some(value) => println!("Có giá trị: {}", value),
        None => println!("Không có giá trị!"),
    }
}

fn main() {
    let num = Some(42);
    let no_num: Option<i32> = None;

    process_number(num);    // Output: Có giá trị: 42
    process_number(no_num); // Output: Không có giá trị!
}
