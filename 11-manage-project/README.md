# Tóm tắt: Quản lý dự án Rust với Packages, Crates và Modules

## 1. **Packages và Crates**
- **Crates**: Là đơn vị biên dịch nhỏ nhất trong Rust, có thể là binary crate (chương trình thực thi) hoặc library crate (chức năng dùng chung).
- **Packages**: Là tập hợp các crates, có thể chứa nhiều binary crate hoặc một library crate.
- Cargo là công cụ quản lý packages, giúp tạo và quản lý dự án dễ dàng.

## 2. **Modules**
- Modules giúp tổ chức mã nguồn, kiểm soát quyền truy cập và tái sử dụng.
- **Visibility**: Mặc định, các mục trong module là private. Có thể làm chúng public với từ khóa `pub`.
- **Cách khai báo**: Module có thể được định nghĩa trong cùng file hoặc trong file riêng biệt.
- **Từ khóa `use`**: Giúp nhập các item từ module khác để giảm thiểu việc khai báo lại.
- **Từ khóa `self` và `super`**: Dùng để truy cập phạm vi trong cùng module hoặc module cha.
- **Module tree**: Có thể khai báo đường dẫn tuyệt đối (bắt đầu từ crate) hoặc tương đối (dùng `super` để "quay lại" module cha).

## Kết luận:
Rust cho phép tổ chức code hiệu quả bằng cách chia nhỏ thành crates và modules, giúp quản lý tốt việc tái sử dụng mã nguồn và quyền truy cập. Sử dụng các công cụ như `use`, `self`, và `super` giúp mã nguồn gọn gàng và dễ duy trì.
