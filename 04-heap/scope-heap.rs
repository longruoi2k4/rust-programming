fn main() {
        let s = String::from("Hello, heap!"); // s có giá trị từ thời điểm này trở đi
        println!("{}", s);
    }   // phạm vi scope đã kết thúc và giá trị của s bị hủy trên Heap, s giải phóng trên Stack.