# Data Types - Kiểu dữ liệu trong Rust

Tương tự như các ngôn ngữ lập trình khác, mọi giá trị trong Rust cũng đều có một kiểu dữ liệu xác định để dựa vào đó máy tính biết phải lưu trữ và làm việc với dữ liệu đó như thế nào.

Rust có hai loại kiểu dữ liệu chính: vô hướng (scalar) và kết hợp (compound).

## 1. Kiểu dữ liệu vô hướng (Scalar Types)
Kiểu dữ liệu vô hướng trong Rust đại diện cho một giá trị duy nhất. Rust có 4 kiểu dữ liệu vô hướng chính:
- Số nguyên (integers)
- Số thực dấu phẩy động (floating-point numbers)
- Kiểu logic (Booleans)
- Ký tự (characters)

### 1.1. Kiểu số nguyên (Integers)
Rust hỗ trợ cả số nguyên có dấu (signed) và không dấu (unsigned). Cả hai kiểu này có các độ dài bit khác nhau:

| Dung lượng | Số nguyên có dấu (signed) | Số nguyên không dấu (unsigned) |
| ---------- | ------------------------- | ----------------------------- |
| 8-bit      | i8                        | u8                            |
| 16-bit     | i16                       | u16                           |
| 32-bit     | i32                       | u32                           |
| 64-bit     | i64                       | u64                           |
| 128-bit    | i128                      | u128                          |
| arch       | isize                     | usize                         |

### 1.2. Số thực dấu phẩy động (floating-point numbers)
Rust có hai kiểu số thực dấu phẩy động:
- f32: Độ chính xác đơn (32 bits)
- f64: Độ chính xác gấp đôi (64 bits). Kiểu mặc định trong Rust là f64.

### 1.3. Kiểu logic (Booleans)
Kiểu Boolean trong Rust chứa hai giá trị: `true` và `false`. Kích thước của kiểu Boolean là 1 byte.

### 1.4. Ký tự (characters)
Ký tự trong Rust có kích thước 4 bytes và có thể đại diện cho bất kỳ ký tự Unicode nào.

## 2. Kiểu dữ liệu kết hợp (Compound Types)
Các kiểu dữ liệu kết hợp cho phép nhóm nhiều giá trị lại với nhau. Rust có hai kiểu dữ liệu kết hợp:
- Tuples
- Arrays

### 2.1. Tuples
Tuple là một cấu trúc dữ liệu nhóm các giá trị có thể có kiểu dữ liệu khác nhau. Các giá trị trong tuple có thể được truy cập bằng cách sử dụng chỉ mục (index). Tuple có độ dài cố định và không thể thay đổi sau khi được khởi tạo.

### 2.2. Arrays
Mảng trong Rust là một tập hợp các phần tử có cùng kiểu dữ liệu và độ dài cố định. Mảng được khai báo bằng dấu ngoặc vuông.

