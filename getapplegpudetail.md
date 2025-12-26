Đây là các lệnh bạn có thể copy và paste thẳng vào iTerm để lấy thông tin GPU trên macOS (nhất là Apple Silicon):

1. Lấy Tên GPU và Số Core (Quan trọng nhất)

Lệnh này không cần quyền admin, đây là lệnh mà code Rust của bạn nên gọi ngầm:

Bash
system_profiler SPDisplaysDataType
Kết quả sẽ hiện: "Chipset Model", "Total Number of Cores".

2. Lấy VRAM (Unified Memory)

Trên Apple Silicon, VRAM là RAM hệ thống. Dùng lệnh này để xem tổng RAM:

Bash
system_profiler SPHardwareDataType | grep "Memory:"
Kết quả: "Memory: 18 GB" (hoặc 16/32/64 GB).

3. Lấy thông tin Metal (Hỗ trợ đồ họa)

Để xem GPU hỗ trợ Metal version mấy (Family):

Bash
system_profiler SPDisplaysDataType | grep "Metal"
4. Lấy Nhiệt độ & Điện năng (Bắt buộc sudo)

Nếu bạn muốn test xem máy cho phép đọc cảm biến không (để debug cho tính năng Monitor):

Bash
sudo powermetrics --samplers gpu_power,thermal -n 1
Nhập mật khẩu máy khi được hỏi.

Tìm dòng: GPU die temperature (nếu có) hoặc Thermal pressure.

Để lấy được con số độ C (ví dụ 45°C, 50°C), chúng ta bắt buộc phải quay lại phương án dùng thư viện smc của Rust (đọc trực tiếp cảm biến phần cứng).

Bạn hãy tạo một file test nhỏ này (src/bin/scan_smc.rs hoặc chạy tạm trong main.rs) để "điểm danh" xem con M3 của bạn đang giấu nhiệt độ ở Key nào:

Bước 1: Thêm dependency

Trong Cargo.toml:

Ini, TOML
[dependencies]
smc = "0.4"
Bước 2: Code "Brute Force" tìm Key nhiệt độ

Code này sẽ quét qua tất cả các Key thường dùng của Apple để xem Key nào trả về số hợp lý (> 20 độ và < 100 độ).

Rust
use smc::{SMC, Kind};

fn main() {
    println!("🔍 ĐANG QUÉT CẢM BIẾN NHIỆT TRÊN MAC M3...");
    
    let smc = match SMC::new() {
        Ok(s) => s,
        Err(e) => {
            println!("❌ Không thể kết nối chip SMC: {:?}", e);
            return;
        }
    };

    // Danh sách các Key nghi vấn cho GPU/SOC trên M-Series
    let candidate_keys = vec![
        // --- GPU Keys (Thường bắt đầu bằng Tg) ---
        "Tg05", "Tg0f", "Tg0D", "Tg00", 
        "Tg10", "Tg11", "Tg01", "Tg02",
        
        // --- SOC Keys (Thường dùng chung nếu GPU nằm trong SOC) ---
        "SocD", // M3 thường dùng cái này cho nhiệt độ chung
        "Tp0D", // E-Cluster / P-Cluster Die
        "Tp05", "Tp01",
        
        // --- Keys lạ khác ---
        "TW0b", // Airflow
    ];

    let mut found = false;

    for key in candidate_keys {
        // Thử đọc key kiểu float (f32)
        match smc.read_key::<f32>(key.into()) {
            Ok(temp) => {
                if temp > 10.0 && temp < 120.0 {
                    println!("✅ TÌM THẤY: [{}] = {:.1}°C", key, temp);
                    found = true;
                }
            },
            Err(_) => {
                // Key không tồn tại, bỏ qua
            }
        }
    }

    if !found {
        println!("⚠️ Không tìm thấy key nào quen thuộc. Apple đã đổi mã trên M3!");
    }
}
Cách xử lý cho Tool của bạn

Trường hợp 1: Nếu Code trên tìm ra Key (Ví dụ Tg0f hoặc SocD) Bạn dùng key đó để hiển thị nhiệt độ chính xác trong tool.

Code: smc.read_key::<f32>("Tg0f".into()).unwrap_or(0.0)

Trường hợp 2: Nếu không tìm ra (Apple khóa nốt SMC) Bạn sẽ dùng thông tin "Thermal pressure" từ powermetrics mà bạn vừa lấy được. Đây là cách Apple muốn dev sử dụng.

Hiển thị trong Tool:

Nếu Nominal -> Hiển thị: "Nhiệt độ: ✅ Mát mẻ (Nominal)"

Nếu Moderate -> Hiển thị: "Nhiệt độ: ⚠️ Hơi ấm (Moderate)"

Nếu Heavy -> Hiển thị: "Nhiệt độ: ❌ Quá nhiệt (Heavy)"

Cách này tuy không có số nhưng lại rất chính xác về mặt "Sức khỏe hệ thống".