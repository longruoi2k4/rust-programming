enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // Bao gồm trạng thái của đồng xu
}

fn main() {
    let coin = Coin::Quarter(String::from("California"));

    if let Coin::Quarter(state) = coin {
        println!("Đây là đồng xu Quarter từ bang {}!", state);
    } else {
        println!("Đây không phải là đồng xu Quarter!");
    }
}
