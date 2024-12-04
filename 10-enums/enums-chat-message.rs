//Định nghĩa enums chat-message
enum ChatMessage {
    Text(String),
    File {filename: String, size : u64},
    SystemMessage,
}

// Hàm sử dụng enum
fn handle_message(message: ChatMessage) {
    match message {
        ChatMessage::Text(content) => {
            println!("Tin nhắn văn bản: \"{}\"", content);
        }
        ChatMessage::File { filename, size } => {
            println!("Tệp được gửi: \"{}\" ({} bytes)", filename, size);
        }
        ChatMessage::SystemMessage => {
            println!("Thông báo hệ thống.");
        }
    }
}

fn main() {
    let text_msg = ChatMessage::Text("Chào bạn!".to_owned());
    let file_msg = ChatMessage::File {
        filename: "document.pdf".to_owned(),
        size: 2048,
    };
    let system_msg = ChatMessage::SystemMessage;

    handle_message(text_msg);
    handle_message(file_msg);
    handle_message(system_msg);
}

