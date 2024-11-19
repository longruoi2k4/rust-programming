# Quản lí bộ nhớ trong Rust #2 - Heap
Tiếp tục tìm hiểu về cách mà một chương trình Rust quản lí bộ nhớ, bài viết này mình sẽ trình bày những kiến thức cơ bản nhất về Heap. Như ta đã biết, khả năng quản lí bộ nhớ an toàn và mạnh mẽ của Rust phần nhiều nhờ cơ chế lưu trữ, phân bổ và dọn dẹp dữ liệu tự động trên Heap. Heap sẽ phức tạp hơn so với Stack vì ta sẽ đề cập nhiều tới khái niệm con trỏ **(pointer)**, một trong những kiến thức khá khó nhằn khi học lập trình. 
Trong bài viết này ta sẽ tìm hiểu:
1. Heap là gì ?
2. Heap và Ownership Rules
3. So sánh Heap trong Rust và C/C++
![](https://assets.devlinux.vn/uploads/editor-images/2024/11/16/7661cddb_cb7c_4f1e_96d2_e1feb0917d16_97a8f57261.webp)

-----
## 1. Heap là gì ?
Heap là vùng nhớ được dùng để lưu trữ các giá trị có kích thước không xác định trước, hoặc các phần tử có kích thước thay đổi. Khi cấp phát bộ nhớ trên heap, một vùng nhớ đủ lớn sẽ được tìm kiếm và dành riêng để sử dụng, sau đó hệ thống sẽ trả về một con trỏ trỏ tới địa chỉ vùng nhớ này. Để truy cập hoặc tương tác dữ liệu trên heap, chúng ta phải thông qua con trỏ đó.
```rust
fn main() {
    let s = String::from("Hello, heap!"); // s được lưu trên heap
} 
```
**Hình ảnh mô phỏng cho code trên**:
![](https://assets.devlinux.vn/uploads/editor-images/2024/11/16/1_23ee038073.png)Con trỏ là một biến đặc biệt, chứa địa chỉ của vùng nhớ, và bản thân nó có kích thước cố định nên thường được lưu trữ trên stack. Stack có cấu trúc tổ chức rõ ràng và thứ tự, còn heap thì ít có tổ chức hơn. Khi bạn yêu cầu một vùng nhớ trên heap, trình quản lý bộ nhớ sẽ tìm một khoảng trống đủ lớn, đánh dấu nó là "đang sử dụng", và trả về một con trỏ dẫn đến vị trí đó. Quá trình này được gọi là cấp phát bộ nhớ trên heap **(allocating on the heap)**.

Điều này khác với stack ở chỗ: khi lưu trữ dữ liệu trên stack, bạn chỉ đẩy giá trị vào mà không cần quá trình cấp phát phức tạp. Tuy nhiên, để lưu dữ liệu lớn hoặc không xác định kích thước, heap là lựa chọn tối ưu hơn. 

Ta có bảng so sánh giữa Heap và Stack:


| | Stack | Heap |
| -------- | -------- | -------- |
| Kiểu dữ liệu trong bộ nhớ     | Dữ liệu có kích thước cố định, biến cục bộ, con trỏ,...    | Dữ liệu có kích thước không xác định hoặc kích thước thay đổi; kiểu dữ liệu phức tạp, kích thước lớn,...   |
| Tốc độ| Dữ liệu được đẩy vào Stack cùng như truy cập trực tiếp vào dữ liệu trong Stack dễ dàng nên có tốc độ quản lí bộ nhớ nhanh hơn| Bộ cấp phát mất thời gian tìm kiếm vùng nhớ nên tốc độ nạp dữ liệu vào Heap chậm hơn; việc truy cập dữ liệu gián tiếp thông qua con trỏ cũng chậm hơn so với Stack.
|Cơ chế dọn dẹp bộ nhớ | Các biến trên stack thường là các biến cục bộ, được giải phóng tự động khi hàm kết thúc. | Dữ liệu trên heap tồn tại cho đến khi nó được giải phóng một cách thủ công hoặc thông qua cơ chế RAII của Rust (khi không còn ai sở hữu nó). |
|Phạm vi truy cập | Chỉ hàm chứa biến đó | Tất cả các hàm đều có thể truy cập được |

Việc cấp phát Heap tốn nhiều tài nguyên và thời gian nên ta hạn chế sử dụng nhất có thể. Nếu đồng thời cấp pháp hoặc giải phóng nhiều vùng nhớ Heap cùng một lúc sẽ gây ra hiện tượng phân mảnh.
## 2. Heap và Ownership Rules
Ở đây, mình sẽ nhắc lại ownership rules:
- Mỗi giá trị trong Rust đều có một biến gọi là owner của nó.
- Chỉ có một owner tại một thời điểm.
- Khi owner ra khỏi phạm vi biến là hàm chứa nó (scope), Rust sẽ tự động gọi hàm drop, giá trị sẽ bị hủy, dọn dẹp bộ nhớ của biến đó.
### 2.1. Heap hoạt động thế nào ?
Rust sử dụng hệ thống ownership để quản lý bộ nhớ trên heap. Điều này giúp Rust tránh được lỗi use-after-free và memory leak, điều mà nhiều ngôn ngữ khác như C/C++ gặp phải khi sử dụng heap.
```rust
    {
        let s = String::from("Hello, heap!"); // s có giá trị từ thời điểm này trở đi
		println!("{}", s);
    }   // phạm vi scope đã kết thúc và giá trị của s bị hủy trên Heap, s giải phóng trên Stack.
```
Trong Rust, thời điểm để giải phóng vùng nhớ của một biến lưu trên Heap mà ta đã sử dụng: khi biến đó ra khỏi phạm vi (scope). Khi điều này xảy ra, Rust sẽ tự động gọi một hàm đặc biệt tên là drop cho chúng ta. Đây là nơi mà Rust "dọn dẹp" vùng nhớ - drop sẽ chứa mã lệnh cần thiết để trả lại vùng nhớ đã cấp phát cho biến đó. Rust sẽ tự động thực hiện việc này mỗi khi gặp dấu đóng ngoặc nhọn (}), đánh dấu kết thúc phạm vi của biến.
### 2.2. Chuyển giao Ownership trong Heap
Để hiểu về chuyển giao Ownership trong Heap, mình lấy ví dụ mã chương trình đơn giản sau đây:
```rust
fn main() {
    let s1 = String::from("Hello, heap!");
    let s2 = s1; // ownership được chuyển từ s1 sang s2
    println!("{}", s1); // Lỗi: s1 không còn quyền sở hữu
}
```
Khi chạy dòng lệnh trên máy tính sẽ báo lỗi:![](https://assets.devlinux.vn/uploads/editor-images/2024/11/16/Anh_man_hinh_2024_11_16_luc_06_26_57_567d98a8f7.png)
Lỗi này gặp phải trong Rust là do quyền sở hữu (ownership) của biến s1. Khi chúng ta gán s1 cho s2, dữ liệu String được sao chép, có nghĩa là chúng ta sao chép con trỏ, độ dài và dung lượng trên stack. Chúng ta không sao chép dữ liệu trên heap mà con trỏ chỉ tới. Trong Rust, khi bạn gán một giá trị kiểu String (một kiểu dữ liệu có quyền sở hữu) cho một biến khác, quyền sở hữu sẽ được chuyển giao (move) thay vì sao chép (copy). Điều này giúp giảm thiểu bộ nhớ Heap và tăng hiệu năng cho chương trình.
![](https://assets.devlinux.vn/uploads/editor-images/2024/11/16/2_049db88ef6.png)Nhờ cơ chế ownership này,  sau dòng **let s2 = s1**, Rust coi s1 không còn giá trị nữa. Do đó, Rust không cần giải phóng bất cứ thứ gì khi khi s1 đi ra khỏi scope. Do đó, Rust tránh được lỗi giải phóng bộ nhớ Heap 2 lần (double free) tạo ra lỗ hổng bảo mật khi cả s1 và s2 đi ra khỏi scope.

Vậy trong những trường hợp bạn muốn copy dữ liệu thực sự thì sao? Rust cho chúng ta 2 giải pháp sau đây:

**Sử dụng clone()** : Nếu bạn muốn s1 vẫn tồn tại sau khi gán, bạn có thể sao chép giá trị của nó thay vì chuyển quyền sở hữu:
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Sao chép giá trị
println!("{}", s1);  // Bây giờ vẫn có thể sử dụng s1
```
**Sử dụng tham chiếu**: 
```rust
let s1 = String::from("hello");
let s2 = &s1; // s2 là một tham chiếu đến s1
println!("{}", s1); // s1 vẫn có thể được sử dụng
println!("{}", s2); // s2 là một tham chiếu
```
## 3. So sánh Heap trong Rust và C/C++
**Trong C/C++**:
```C
#include <stdio.h>
#include <stdlib.h> // Thư viện cho malloc và free
void  heap() {
       // Cấp phát bộ nhớ động cho một biến kiểu int trên heap
	    int *ptr = (int *)malloc(sizeof(int));  
      //  free(ptr); Không giải phóng giá trị con trỏ ptr bằng hàm free()
}

int main() {
		heap();
}

```
Ở chương trình C ví dụ trên, người lập trình viên không sử dụng hàm free() để giải phóng bộ nhớ Heap. Do vậy sau khi hàm main() chạy xong, bộ nhớ stack đã được giải phóng hoàn toàn, kể cả con trỏ ptr nhưng vùng nhớ động mà con trỏ ptr trỏ đến vẫn được lưu lại trên Heap và không được tái sử dụng lại. Điều này gây ra một lỗi bảo mật nghiêm trọng là memory leak.
Do quản lí bộ nhớ Heap trên C hoàn toàn thủ công nên gặp nhiều khó khăn khi viết những chương trình lớn, đòi hỏi độ tối ưu cao. Nếu chúng ta quên giải phóng Heap, chúng ta sẽ lãng phí bộ nhớ. Nếu chúng ta làm điều đó quá sớm, chúng ta sẽ có một biến không hợp lệ. Nếu chúng tôi làm điều đó hai lần, đó cũng là một lỗi. Chúng ta cần ghép chính xác một allocate với một free.

**Trong Rust**:
Khi sử dụng Rust, người lập trình viên sẽ không phải đau đầu với việc quản lí bộ nhớ trên Heap nữa vì Rust tự động quản lý bộ nhớ thông qua hệ thống ownership và borrow checker, giảm thiểu lỗi bộ nhớ. Cơ chế này của Rust không dùng đến garbage collection nên giúp nó vẫn duy trì được hiệu suất và tốc độ xử lí cao.


|  | C/C++ | Rust |
| -------- | -------- | -------- |
| Hiệu năng     | C/C++ có hiệu năng tốt vì việc quản lý bộ nhớ hoàn toàn do lập trình viên điều khiển. Tuy nhiên, việc quản lý thủ công này cũng dễ dẫn đến lỗi nghiêm trọng.     | Rust có hiệu năng gần tương đương với C nhờ tối ưu hóa tại compile time và hệ thống ownership giúp tránh nhiều lỗi runtime     |
| Cách sử dụng | Sử dụng các hàm như malloc(), calloc(), và realloc() để cấp phát bộ nhớ, và free() để giải phóng. | Tự động thông qua RAII. Sử dụng các kiểu như Box, Rc, Arc để quản lý bộ nhớ trên heap mà không cần gọi free() thủ công. |
| Mức độ an toàn | Do không có kiểm tra an toàn bộ nhớ tại runtime, chương trình C thường nhẹ và nhanh, nhưng có thể không an toàn, gây ra các lỗi như Memory leak, use-after-free,... | Các kiểu dữ liệu như Rc và Arc có thể gây ra một chút overhead do sử dụng bộ đếm tham chiếu. |

**Kết luận**:
Qua việc tìm hiểu về cách quản lí bộ nhớ trong Heap, ta thấy Rust có lợi thế về mặt an toàn bộ nhớ, giúp lập trình viên tránh được nhiều lỗi phổ biến mà C thường gặp. Tuy nhiên, nếu lập trình viên có kinh nghiệm quản lý bộ nhớ, C có thể cung cấp hiệu năng tối đa do không có overhead từ hệ thống kiểm tra an toàn.

