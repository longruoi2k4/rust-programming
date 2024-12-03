fn main() {
    let x = 5;
    let x = x + 1;
   {
        let x = x * 2;
        println!("Giá trị của x trong phạm vi bên trong là: {x}");
    }
     println!("Giá trị của x là: {x}");
}