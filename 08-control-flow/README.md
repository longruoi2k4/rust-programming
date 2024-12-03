# Luồng điều khiển trong Rust (Control Flow)

Sau những bài viết về các khái niệm cơ bản, hôm nay chúng ta sẽ tìm hiểu về cách kiểm soát luồng thực thi của mã nguồn trong Rust. Điều này giúp bạn xây dựng chương trình mạnh mẽ và rõ ràng thông qua các cấu trúc như biểu thức `if` và vòng lặp.

## 1. Biểu thức if trong Rust

### 1.1. Cấu trúc cơ bản
Biểu thức `if` trong Rust cho phép phân nhánh luồng thực thi dựa trên điều kiện. Trong Rust, điều kiện trong `if` luôn phải có kiểu **boolean**. Điều này khác với một số ngôn ngữ như C/C++, nơi bạn có thể sử dụng số nguyên. Điều kiện không cần được bao quanh bởi dấu ngoặc, và mỗi điều kiện sẽ theo sau một khối lệnh.

### 1.2. Sử dụng `if` trong câu lệnh `let`
Vì `if` là một biểu thức trong Rust, bạn có thể sử dụng nó để gán giá trị cho biến. Điều này giúp bạn gán giá trị một cách linh hoạt trong các tình huống khác nhau.

### 1.3. Xử lý nhiều điều kiện với `else if`
Rust cho phép sử dụng cấu trúc `if-else` để kiểm tra nhiều điều kiện khác nhau. Các nhánh trong `if-else` phải trả về cùng một kiểu dữ liệu. Rust sẽ kiểm tra các điều kiện lần lượt và thực thi nhánh đầu tiên thỏa mãn điều kiện.

## 2. Vòng lặp trong Rust

### 2.1. Vòng lặp `loop`
Rust cung cấp vòng lặp vô hạn với từ khóa `loop`. Bạn có thể sử dụng câu lệnh `break` để thoát khỏi vòng lặp khi cần thiết và câu lệnh `continue` để bỏ qua phần còn lại của vòng lặp hiện tại và tiếp tục với lần lặp tiếp theo.

### 2.2. Vòng lặp có điều kiện với `while`
Vòng lặp `while` trong Rust cho phép bạn lặp lại các thao tác dựa trên một điều kiện. Vòng lặp sẽ tiếp tục chạy khi điều kiện được đánh giá là đúng.

### 2.3. Vòng lặp `for` và `range`
Cấu trúc `for` trong Rust rất mạnh mẽ và có thể sử dụng với `range` để lặp qua một dãy giá trị. Đây là một cách rất tiện lợi để xử lý các tác vụ lặp mà không cần phải lo lắng về chỉ số đếm.

## 3. Kiểm tra case bằng `match`
Rust không có câu lệnh `switch` như trong C, nhưng cung cấp câu lệnh `match` để kiểm tra giá trị của một biểu thức và so khớp với các mẫu (patterns) khác nhau. Điều này cho phép bạn xử lý các tình huống phức tạp và linh hoạt hơn, đặc biệt khi làm việc với các kiểu dữ liệu phức tạp như `enum` và `tuple`.

## 4. Câu lệnh `if let`
`if let` là một cú pháp tiện lợi trong Rust để làm việc với các kiểu dữ liệu `enum` như `Option`. Thay vì phải dùng `match`, bạn có thể dễ dàng kiểm tra và xử lý giá trị trong `Option` một cách ngắn gọn và dễ hiểu.

## 5. Câu lệnh `let else`
`let else` cho phép bạn gán giá trị trong khi kiểm tra, và nếu không có giá trị phù hợp, nó sẽ thực hiện phân nhánh như `return`, `break`, hoặc `panic!`. Điều này giúp viết mã dễ dàng và trực quan hơn khi làm việc với các giá trị có thể không tồn tại.

## 6. Câu lệnh `while let`
`while let` là một cú pháp tương tự như `if let`, giúp bạn xử lý các giá trị phức tạp trong các vòng lặp. Cấu trúc này giúp mã của bạn trở nên rõ ràng và dễ hiểu hơn khi làm việc với các kiểu dữ liệu như `Option`.

-----
**Kết luận**:

Qua bài viết này, bạn đã hiểu các cấu trúc điều khiển trong Rust như `if`, vòng lặp `loop`, `while`, `for`, `match`, và các câu lệnh như `if let`, `let else`, và `while let`. Đây là những công cụ hữu ích giúp bạn xây dựng logic chương trình hiệu quả trong Rust.
