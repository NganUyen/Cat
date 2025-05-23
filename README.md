# pyCatAI-pet-rust

Phiên bản Rust của pyCatAI-pet, một ứng dụng desktop pet sử dụng trí tuệ nhân tạo (AI) để tạo bình luận thú vị.

## Mô tả

pyCatAI-pet-rust là một ứng dụng desktop pet được viết bằng Rust. Ứng dụng hiển thị một con mèo hoạt hình trên màn hình của bạn, có khả năng tạo bình luận thông minh về những gì bạn đang làm trên màn hình bằng cách sử dụng Google Gemini API.

Con mèo sẽ di chuyển tự nhiên trên màn hình, thực hiện các hoạt động như đi bộ, nhảy, ngồi và ngủ. Thỉnh thoảng, nó sẽ tạo bình luận dựa trên những gì nó "nhìn thấy" trên màn hình của bạn.

## Tính năng

- Con mèo hoạt ảnh với nhiều trạng thái và chuyển động
- Di chuyển tự nhiên trên màn hình desktop
- Tạo bình luận thông minh sử dụng Google Gemini API
- Chuyển đổi văn bản thành giọng nói (TTS)
- Hiệu ứng chuyển màn hình
- Giao diện tùy chỉnh

## Cài đặt và Chạy

### Yêu cầu

- Rust và Cargo (phiên bản 1.60 trở lên)
- Windows (đã được thử nghiệm trên Windows 10/11)

### Cài đặt

1. Clone repository này:

```bash
git clone https://github.com/yourusername/pyCatAI-pet-rust.git
cd pyCatAI-pet-rust
```

2. Xây dựng ứng dụng:

```bash
cargo build --release
```

3. Chạy ứng dụng:

```bash
cargo run --release
```

## API Key

Để sử dụng đầy đủ tính năng của Google Gemini API, bạn cần cập nhật API key của mình trong file `src/comment_generator.rs`.

## Cấu trúc dự án

- `src/main.rs` - Điểm vào chính của ứng dụng
- `src/display.rs` - Quản lý hiển thị và cửa sổ
- `src/sprite_handler.rs` - Xử lý sprite và animation
- `src/window_handler.rs` - Tương tác với Windows API
- `src/comment_generator.rs` - Tạo bình luận sử dụng AI
- `sprites/` - Thư mục chứa các sprite của con mèo
- `assets/` - Các tài nguyên khác (logo, icon, ...)

## Giấy phép

Dự án này được cấp phép theo [tên giấy phép]. Xem file LICENSE để biết thêm chi tiết.

## Tác giả

- [Tên của bạn]

## Ghi nhận

Dự án này là phiên bản Rust của [pyCatAI-pet](https://github.com/yourusername/pyCatAI-pet) ban đầu được viết bằng Python.
