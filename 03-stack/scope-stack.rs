fn main() {  // 1. Stack frame hàm main push vào Stack
    let x = 2; // 2. Biến x đi vào scope, giá trị 2 có owner là x.
    example(2); // 3. Giá trị của x di chuyển vào hàm. 
} // 7. Owner x ra khỏi scope, giá trị x bị hủy. Stack frame hàm main pop khỏi stack
// 8. Chạy xong chương trình.										 
fn example(a: i32) { // 4. Stack example push vào stack. Biến a đi vào scope, giá trị 2 có owner là a.
    let b = 5; // 5. Biến b đi vào scope, giá trị 5 có owner là b.
}  // 6. Owner a và b ra khỏi scope, giá trị 5 và 2 bị hủy. Stack frame hàm example pop ra khỏi stack.