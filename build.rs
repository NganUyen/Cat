use std::io;
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    // làm ơn học git rồi dùng, làm theo yt thì có mà ăn cứt
    // Đối với Windows, có thể sử dụng winres để tạo biểu tượng và metadata
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("FileDescription", "pyCatAI-pet Rust Version");
        res.set("ProductName", "pyCatAI-pet");
        res.set("LegalCopyright", "Copyright © 2023");
        if let Err(e) = res.compile() {
            eprintln!("Failed to set Windows resources: {}", e);
        }
    }
    
    // Đường dẫn tới thư mục sprites
    let sprites_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("sprites");
    
    // Đảm bảo thư mục sprites tồn tại
    if !sprites_dir.exists() {
        println!("cargo:warning=Sprites directory does not exist. Creating...");
        std::fs::create_dir_all(&sprites_dir)?;
    }
    
    // Thêm rerun-if-changed để Cargo rebuild khi sprites thay đổi
    println!("cargo:rerun-if-changed=sprites");
    println!("cargo:rerun-if-changed=assets");
    
    Ok(())
} 