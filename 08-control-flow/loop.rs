fn main() {
    let mut cnt = 0u32;

    // Vòng lặp vô hạn
    loop {
        cnt += 1;

        if cnt == 3 {
            println!("three");

            // Bỏ qua phần còn lại của vòng lặp này
            continue;
        }

        println!("{}", cnt);

        if cnt == 5 {
            println!("Done!");

            // Thoát khỏi vòng lặp này
            break;
        }
    }
}