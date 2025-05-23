@echo off
echo Đang cài đặt pyCatAI-pet-rust...

REM Kiểm tra Rust đã được cài đặt chưa
where rustc >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Rust chưa được cài đặt. Vui lòng cài đặt Rust từ https://rustup.rs/
    echo Sau khi cài đặt, chạy lại script này.
    pause
    exit /b 1
)

echo Rust đã được cài đặt. Tiếp tục...

REM Cập nhật các dependencies
echo Cập nhật dependencies...
cargo update

REM Tạo thư mục sprites và assets nếu chưa tồn tại
if not exist "sprites" mkdir sprites
if not exist "assets" mkdir assets

REM Biên dịch ứng dụng
echo Biên dịch ứng dụng...
cargo build --release

echo Cài đặt hoàn tất! Để chạy ứng dụng, sử dụng run.bat hoặc 'cargo run --release'.
pause 