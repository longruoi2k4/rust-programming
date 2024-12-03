// Định nghĩa struct Person
struct Person {
    name: String,
    age: u32,
}

// Hàm in thông tin cho Person
fn print_info(person: &Person) {
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}

fn main() {
    // Khởi tạo đối tượng Person
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Gọi hàm print_info để in thông tin
    print_info(&person);
}