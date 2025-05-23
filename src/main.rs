mod display;
mod sprite_handler;
mod window_handler;
mod comment_generator;

use anyhow::Result;
use display::Display;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Khởi tạo logging
    env_logger::init();
    info!("Starting pyCatAI-pet Rust version");
    
    // Khởi chạy ứng dụng
    match Display::new().run() {
        Ok(_) => {
            info!("Application closed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application error: {}", e);
            // Ghi lỗi vào file error log
            use std::fs::OpenOptions;
            use std::io::Write;
            use chrono::Local;
            
            let now = Local::now();
            let error_message = format!("{}: {}\n", now.format("%Y-%m-%d %H:%M:%S"), e);
            
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("errorlog.txt")?;
                
            file.write_all(error_message.as_bytes())?;
            
            Err(e.into())
        }
    }
} 