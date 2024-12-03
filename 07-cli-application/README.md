# Viết CLI Application đơn giản đầu tiên

Dựa vào những gì đã được học ở phần trước, mình sẽ cùng với các bạn viết một chương trình cơ bản đầu tiên tính toán chỉ số BMI của cơ thể người, chúng ta sẽ giao tiếp với chương trình thông qua console.

## 1. Viết hàm `calculate_bmi()`

Hàm này sẽ thực hiện công việc tính toán và trả về chỉ số BMI với input là chiều cao, cân nặng và output là chỉ số BMI. Vì chiều cao, cân nặng và chỉ số BMI đều là số thập phân nên mình dùng kiểu dữ liệu `f64`.

## 1.2. Viết hàm `get_input()`

Hàm này sẽ giúp chúng ta nhận dữ liệu đầu vào từ người dùng. Chúng ta sẽ in ra màn hình yêu cầu nhập, sau đó nhận giá trị nhập vào dưới dạng chuỗi, chuyển đổi chuỗi này thành kiểu `f64` và trả về.

## 1.3. Viết hàm `main` và chạy chương trình

Hàm `main` sẽ gọi các hàm đã định nghĩa ở trên để thực hiện các tác vụ sau:

- Nhập chiều cao và cân nặng từ người dùng.
- Tính toán chỉ số BMI dựa trên công thức.
- In ra kết quả BMI và phân loại dựa vào chỉ số BMI đó.

Chương trình sẽ phân loại chỉ số BMI theo các mức như sau:
- Dưới 18.5: Thiếu cân.
- Từ 18.5 đến 24.9: Cân nặng bình thường.
- Từ 25 đến 29.9: Thừa cân.
- Từ 30 trở lên: Béo phì.

Chạy chương trình và nhập các giá trị chiều cao và cân nặng để xem kết quả phân loại BMI.

-----

