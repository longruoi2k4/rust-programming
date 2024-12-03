# Struct trong Rust

## 1. Định nghĩa và Khởi tạo Struct
### 1.1. Struct là gì?
Struct trong Rust là một kiểu dữ liệu tùy chỉnh, cho phép nhóm các giá trị có mối liên hệ với nhau thành một đơn vị. Các giá trị này được gọi là "trường" (fields) trong struct, và struct tương tự như object trong lập trình hướng đối tượng.

### 1.2. Khai báo Struct
Struct được khai báo với từ khóa `struct`, sau đó là tên của struct và các trường với kiểu dữ liệu tương ứng.

### 1.3. Khởi tạo Struct
Để tạo một instance của struct, bạn sử dụng cú pháp tương tự như khai báo biến, với các giá trị cho từng trường.



## 2. Phương thức (Method) trong Struct
### 2.1. Định nghĩa phương thức
Phương thức được định nghĩa trong block `impl` của struct, luôn nhận tham số `self`. Bạn có thể gọi phương thức thông qua instance của struct.

### 2.2. Hàm liên kết (Associated Functions)
Hàm liên kết là các hàm trong block `impl` không cần tham số `self`, thường được dùng để khởi tạo instance của struct.

### 2.3. Multiple impl Blocks
Một struct có thể có nhiều block `impl`, giúp tổ chức các phương thức một cách rõ ràng và dễ quản lý.

## Kết luận
Struct trong Rust giúp nhóm các thuộc tính có liên quan lại với nhau, làm cho mã nguồn dễ đọc và bảo trì hơn. Đây là công cụ mạnh mẽ trong lập trình Rust, cùng với các tính năng như phương thức và hàm liên kết.
