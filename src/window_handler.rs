// File này hiện không sử dụng. Để dự án gọn gàng, có thể xóa hoàn toàn file này
// Nhưng tôi giữ lại một số import để tránh lỗi biên dịch nếu có tham chiếu đến nó
// Xóa import không sử dụng 
// use log::error;

// Xóa bỏ struct và implementation không sử dụng
// pub struct Handler;
// 
// impl Handler {
//     pub fn get_foreground_window_position() -> Option<(i32, i32, i32)> {
//         unsafe {
//             let window = GetForegroundWindow();
//             
//             if window == HWND(0) {
//                 return None;
//             }
//             
//             let mut rect = windows::Win32::Foundation::RECT::default();
//             
//             if GetWindowRect(window, &mut rect).is_ok() {
//                 if rect.left >= 40 && rect.top >= 50 {
//                     return Some((rect.left, rect.top, rect.right));
//                 }
//             } else {
//                 error!("Failed to get window rectangle");
//             }
//             
//             None
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
//     
//     #[test]
//     fn test_get_foreground_window_position() {
//         let pos = Handler::get_foreground_window_position();
//         println!("Window position: {:?}", pos);
//     }
// } 