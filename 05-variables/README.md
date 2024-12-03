# Variables và Mutability trong Rust

Trong bài viết này, chúng ta sẽ tìm hiểu về các biến và hằng trong Rust, tập trung vào các điểm khác biệt so với các ngôn ngữ khác.

## 1. Tính Bất Biến (Immutable Variables)
- Biến trong Rust mặc định là bất biến (immutable). Điều này có nghĩa là sau khi khởi tạo, bạn không thể thay đổi giá trị của biến.
- Ưu điểm: Giảm thiểu lỗi khi giá trị không thay đổi bất ngờ và giúp mã dễ đọc, dễ duy trì.

## 2. Biến Có Thể Thay Đổi (Mutable Variables)
- Biến có thể thay đổi được khai báo với từ khóa `mut`. 
- Khi sử dụng `mut`, bạn có thể thay đổi giá trị của biến trong suốt vòng đời của nó.

## 3. Shadowing
- Shadowing cho phép khai báo lại một biến với cùng tên nhưng giá trị và kiểu dữ liệu mới.
- Khác với `mut`, shadowing tạo ra một biến mới trong phạm vi nhất định, có thể thay đổi kiểu dữ liệu.

## 4. Hằng Số (Constants)
- Hằng số trong Rust được khai báo với từ khóa `const`, luôn bất biến và phải chỉ định kiểu dữ liệu rõ ràng.
- Hằng số không thể thay đổi giá trị trong suốt vòng đời của chương trình, và không thể sử dụng `mut` hay `shadowing`.

## Kết luận
Rust ưu tiên tính bất biến để bảo vệ tính an toàn và hiệu quả khi lập trình, nhưng vẫn cho phép linh hoạt với biến có thể thay đổi và shadowing khi cần thiết. Hằng số cung cấp cách quản lý giá trị cố định trong suốt chương trình.
