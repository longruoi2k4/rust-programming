# Quản lí bộ nhớ trong Rust #1 - Stack
Ở bài viết về giới thiệu ngôn ngữ Rust, ta đã biết một trong những ưu điểm lớn nhất của Rust đó là khả năng quản lí bộ nhớ "độc đáo" giúp chúng ta có thể viết code an toàn và hiệu quả hơn. 
Mỗi ngôn ngữ lập trình lại có những cơ chế quản lí bộ nhớ khác nhau: Java, Python sử dụng **garbage collection** để tìm và giải phóng bộ nhớ lúc runtime; C và C++ thì quản lí thụ động, lập trình viên phải tự chỉ định **(allocate)** và giải phóng **(free)** bộ nhớ. Rust đi theo một hướng khác, memory được quản lí bởi một hệ thống quyền sở hữu **(ownership)** thông minh, cơ chế mượn dữ liệu **(borrowing)**. Ta sẽ thấy rõ được điều đó, thông qua cách Rust phân bổ, quản lí bộ nhớ trên **Stack** và **Heap**. Và ở bài viết này ta sẽ tìm hiểu về **Stack**.

Trong bài viết này, ta sẽ tìm hiểu:
1. Stack là gì ?
2. Stack và Ownership Rules
3. So sánh Stack trong Rust và C/C++

-----


 ## 1. Stack là gì ?
 
Tất cả programs đều cần một nơi để lưu trữ những giá trị mà nó sử dụng trong quá trình xử lí dữ liệu đầu vào (input) cho ra dữ liệu đầu ra (output). Stack là những phần vùng nhớ có sẵn mà máy tính cấp phát để lưu trữ các giá trị (ở đây cụ thể là các biến) mà chương trình sử dụng trong runtime. Stack chứa các **Stack Frame**, mỗi Stack Frame chứa các biến cục bộ của một hàm trong chương trình. Stack lưu trữ các giá trị theo cơ chế **LIFO (last in first out)**, tức là lưu các giá trị theo thứ tự nhận được và giải phóng các giá trị theo thứ tự ngược lại. Thêm dữ liệu được gọi là **pushing onto the stack**, và xóa dữ liệu được gọi là **popping off the stack** ![](https://assets.devlinux.vn/uploads/editor-images/2024/11/15/1_0945d51f18.png)
Kích thước của một Stack Frame được xác định tại thời điểm biên dịch, do vậy chỉ những dữ liệu có kích thước cố định (fixed size) mới được lưu trữ trên Stack. Các dữ liệu có kích thước cố định có thể là:
- Các biến cục bộ
- Các chuỗi, mảng có số phần tử xác định khi khái báo
- Con trỏ đến dữ liệu Heap
- ...

Quyền truy cập của một Stack Frame chỉ thuộc về duy nhất hàm đã tạo ra nó. Điều này cho hàm trong Rust có khái niệm về vòng đời (life time), tức là hàm có thể được tạo ra và tự động giải phóng đi sau khi đã chạy xong hàm (life time hết). Điều này giúp chúng ta sẽ không phải lo về việc giải phóng bộ nhớ thụ động thông qua hàm free() trong như trong C.
## 2. Stack và Ownership Rules
Về ownership rules thì có một số điều cơ bản sau:
- Mỗi giá trị trong Rust đều có một biến gọi là owner của nó.
- Chỉ có một owner tại một thời điểm.
- Khi owner ra khỏi phạm vi biến là hàm chứa nó (scope), Rust sẽ tự động gọi hàm drop, giá trị sẽ bị hủy, dọn dẹp bộ nhớ của biến đó.

Trước khi thực thi chương trình, Rust có khả năng tính toán kích thước của Stack Frame trong quá trình biên dịch. Do đó, Rust tạo ra Stack Frame vừa đủ để chứa các dữ liệu có kích thước cố định trong hàm đó. 

Ta có đoạn code đơn giản dưới đây để mô phỏng cho việc Stack trong Rust hoạt động:
```rust
fn main() {  // 1. Stack frame hàm main push vào Stack
			let x = 2; // 2. Biến x đi vào scope, giá trị 2 có owner là x.
			example(2); // 3. Giá trị của x di chuyển vào hàm. 
} // 7. Owner x ra khỏi scope, giá trị x bị hủy. Stack frame hàm main pop khỏi stack
// 8. Chạy xong chương trình.										 
fn example(a: i32) { // 4. Stack example push vào stack. Biến a đi vào scope, giá trị 2 có owner là a.
			let b = 5; // 5. Biến b đi vào scope, giá trị 5 có owner là b.
}  // 6. Owner a và b ra khỏi scope, giá trị 5 và 2 bị hủy. Stack frame hàm example pop ra khỏi stack.
```
**Mô phỏng cách Stack Frame được lưu theo code trên:** 
![](https://assets.devlinux.vn/uploads/editor-images/2024/11/15/3_ac92fe5a39.png)
Điều quan trọng, Stack cũng có kích thước giới hạn được tính dựa trên kiến trúc máy tính, hệ điều hành, trình biên dịch,... Do đó, ta cần phải sử dụng cẩn thận để tránh hiện tượng crash do tràn bộ nhớ Stack **(Stack Overflow)**

Dưới đây là một ví dụ đơn giản cho việc không kiểm soát được hàm đệ quy gây ra lỗi Stack Overflow:
```rust
fn main() {
    // Bắt đầu đệ quy và không có điểm dưng
    main();
}
```
Sau khi biên dịch đoạn chương trình trên máy tính sẽ báo lỗi:
![](https://assets.devlinux.vn/uploads/editor-images/2024/11/15/Anh_man_hinh_2024_11_15_luc_15_54_06_40b5ab5a9e.png)
## 3. So sánh Stack trong Rust và C/C++
Khi làm việc với Rust hay C/C++, ta phải đặc biệt quan tâm tới cách quản lí bộ nhớ trên Stack và Heap. Khác với các ngôn ngữ bậc cao như Python, Java,.... quản lí bộ nhớ thông qua hệ thống động và garbage collector, Rust và C/C++ làm việc trực tiếp với Stack do đó có hiệu suất cao hơn. Sự khác biệt giữa Rust và C/C++ trong quản lí bộ nhớ được thể hiện nhiều nhất ở Heap, còn đối với Stack không có sự khác biệt quá nhiều.
Ta có bảng so sánh dưới đây:


|  | C/C++ | Rust |
| -------- | -------- | -------- |
|Cấp phát trên Stack     |Các biến cục bộ được cấp phát và giải phóng tự động khi hàm kết thúc.     | Tương tự, nhưng có hệ thống ownership và borrowing bảo vệ an toàn bộ nhớ.     |
| An toàn bộ nhớ |Không có bảo vệ, có thể dẫn đến lỗi như stack overflow hoặc use-after-free. | Bảo vệ chặt chẽ với hệ thống ownership và borrow checker. Kiểm tra lỗi bộ nhớ tại thời điểm biên dịch, giảm thiểu lỗi runtime. |
|Hiệu suất | Cung cấp hiệu suất tối ưu do không có overhead từ bảo vệ bộ nhớ. |Hiệu suất tương đương với C, nhưng có thêm bảo vệ an toàn bộ nhớ. |

**Kết luận**: 
Như vậy qua bài viết này, hi vọng các bạn đã hiểu rõ thêm về Stack. Stack trong Rust cung cấp một hệ thống an toàn bộ nhớ mạnh mẽ thông qua ownership và borrowing, đảm bảo rằng các lỗi liên quan đến bộ nhớ sẽ được phát hiện ngay trong quá trình biên dịch, mặc dù vẫn duy trì hiệu suất gần như C.
Nếu bạn muốn viết mã an toàn, tránh các lỗi như tràn bộ đệm hoặc lỗi sử dụng bộ nhớ không hợp lệ, Rust là sự lựa chọn tốt hơn trong các ứng dụng hệ thống.

