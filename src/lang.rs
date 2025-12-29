// Language module - Multi-language support for pchecker

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Vietnamese,
    English,
}

// Text translations
pub struct Text {
    pub lang: Language,
}

impl Text {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }

    pub fn header(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Công cụ kiểm tra phần cứng",
            Language::English => "Hardware Info Tool",
        }
    }

    pub fn system(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "HỆ ĐIỀU HÀNH",
            Language::English => "SYSTEM",
        }
    }

    pub fn cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "cpu",
            Language::English => "cpu",
        }
    }

    pub fn gpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "gpu",
            Language::English => "gpu",
        }
    }

    pub fn ram(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "ram",
            Language::English => "ram",
        }
    }

    pub fn cores_label(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "nhân",
            Language::English => "cores",
        }
    }

    pub fn disk(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Ứ CỨNG",
            Language::English => "DISK",
        }
    }

    pub fn ram_free(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "trống",
            Language::English => "free",
        }
    }

    pub fn done_in(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Hoàn thành trong",
            Language::English => "Done in",
        }
    }

    pub fn detecting(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang phát hiện phần cứng...",
            Language::English => "Detecting hardware...",
        }
    }

    pub fn no_gpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Không phát hiện GPU rời",
            Language::English => "No dedicated GPU detected",
        }
    }

    pub fn health_check(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "KIỂM TRA SỨC KHỎE PHẦN CỨNG",
            Language::English => "HARDWARE HEALTH CHECK",
        }
    }

    pub fn testing_cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang kiểm tra CPU",
            Language::English => "Testing CPU",
        }
    }

    pub fn testing_ram(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang kiểm tra RAM",
            Language::English => "Testing RAM",
        }
    }

    pub fn critical_issues(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "VẤN ĐỀ NGHIÊM TRỌNG:",
            Language::English => "CRITICAL ISSUES:",
        }
    }

    pub fn issues_detected(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "PHÁT HIỆN VẤN ĐỀ:",
            Language::English => "ISSUES DETECTED:",
        }
    }

    pub fn summary(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "TÓM TẮT:",
            Language::English => "SUMMARY:",
        }
    }

    pub fn hardware_good(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Phần cứng có vẻ ở trạng thái tốt",
            Language::English => "Hardware appears to be in good condition",
        }
    }

    pub fn hardware_some_issues(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Phần cứng có một số vấn đề",
            Language::English => "Hardware has some issues",
        }
    }

    pub fn hardware_not_recommended(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Không khuyến nghị sử dụng",
            Language::English => "Not recommended for use",
        }
    }

    pub fn cpu_health_check(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Kiểm tra CPU",
            Language::English => "CPU Health Check",
        }
    }

    pub fn operations(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "phép tính",
            Language::English => "operations",
        }
    }

    pub fn ops_per_sec(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "phép/giây",
            Language::English => "ops/sec",
        }
    }

    pub fn avg_op_time(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tb thời gian",
            Language::English => "avg time",
        }
    }

    pub fn variance(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "dao động",
            Language::English => "variance",
        }
    }

    pub fn ram_health_check(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Kiểm tra RAM",
            Language::English => "RAM Health Check",
        }
    }

    pub fn tested_gb(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "đã test",
            Language::English => "tested",
        }
    }

    pub fn write_speed(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tốc độ ghi",
            Language::English => "write speed",
        }
    }

    pub fn read_speed(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tốc độ đọc",
            Language::English => "read speed",
        }
    }

    pub fn errors_detected(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "lỗi phát hiện",
            Language::English => "errors detected",
        }
    }

    pub fn testing_disk(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang kiểm tra ổ cứng",
            Language::English => "Testing Disk",
        }
    }

    pub fn disk_health_check(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Kiểm tra ổ cứng",
            Language::English => "Disk Health Check",
        }
    }

    pub fn seek_time(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "thời gian seek",
            Language::English => "seek time",
        }
    }

    pub fn bad_sectors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "bad sector",
            Language::English => "bad sectors",
        }
    }

    // Disk field labels
    pub fn device(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "thiết bị",
            Language::English => "device",
        }
    }

    pub fn usage(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "đã dùng",
            Language::English => "usage",
        }
    }

    pub fn available(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "còn trống",
            Language::English => "available",
        }
    }

    pub fn performance_test(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "KIỂM TRA HIỆU NĂNG",
            Language::English => "PERFORMANCE TEST",
        }
    }

    pub fn smart_health(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "SỨC KHỎE SMART",
            Language::English => "SMART HEALTH",
        }
    }

    pub fn smart_status(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "trạng thái SMART",
            Language::English => "SMART status",
        }
    }

    pub fn temperature(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "nhiệt độ",
            Language::English => "temperature",
        }
    }

    pub fn frequency(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "xung nhịp",
            Language::English => "frequency",
        }
    }

    pub fn power_on_hours(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "giờ hoạt động",
            Language::English => "power on hours",
        }
    }

    pub fn power_cycles(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "chu kỳ bật",
            Language::English => "power cycles",
        }
    }

    pub fn model(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "mẫu",
            Language::English => "model",
        }
    }

    // GPU health check translations
    pub fn gpu_health_check(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Kiểm tra GPU",
            Language::English => "GPU Health Check",
        }
    }

    pub fn testing_gpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang kiểm tra GPU",
            Language::English => "Testing GPU",
        }
    }

    // Additional labels for result boxes
    pub fn disk_label(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "đĩa",
            Language::English => "disk",
        }
    }

    pub fn size(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "kích thước",
            Language::English => "size",
        }
    }

    pub fn fs(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "fs",
            Language::English => "fs",
        }
    }

    pub fn type_label(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "kiểu",
            Language::English => "type",
        }
    }

    pub fn ssd(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "SSD",
            Language::English => "SSD",
        }
    }

    pub fn hdd(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "HDD",
            Language::English => "HDD",
        }
    }

    pub fn unified_memory(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Unified (chia sẻ)",
            Language::English => "Unified (with CPU)",
        }
    }

    pub fn soc_see_cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "SoC (xem CPU)",
            Language::English => "SoC (see CPU)",
        }
    }

    pub fn not_available(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "N/A",
            Language::English => "N/A",
        }
    }

    #[allow(dead_code)]
    pub fn sensors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Cảm biến",
            Language::English => "Sensors",
        }
    }

    // SMART disk health labels
    pub fn health(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "sức khỏe",
            Language::English => "health",
        }
    }

    pub fn ssd_life(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tuổi thọ SSD",
            Language::English => "SSD life",
        }
    }

    pub fn serial(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "số serial",
            Language::English => "serial",
        }
    }

    pub fn firmware(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "firmware",
            Language::English => "firmware",
        }
    }

    pub fn realloc_sectors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "realloc sectors",
            Language::English => "realloc sectors",
        }
    }

    pub fn pending_sectors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "pending sectors",
            Language::English => "pending sectors",
        }
    }

    pub fn realloc_events(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "realloc events",
            Language::English => "realloc events",
        }
    }

    pub fn total_written(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tổng đã ghi",
            Language::English => "total written",
        }
    }

    pub fn total_read(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tổng đã đọc",
            Language::English => "total read",
        }
    }

    // GPU specific labels
    pub fn gpu_freq(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tần số",
            Language::English => "GPU freq",
        }
    }

    pub fn gpu_power(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "công suất",
            Language::English => "GPU power",
        }
    }

    pub fn gpu_usage(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "sử dụng",
            Language::English => "GPU usage",
        }
    }

    pub fn gpu_cores(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "nhân GPU",
            Language::English => "GPU cores",
        }
    }

    pub fn metal(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Metal",
            Language::English => "Metal",
        }
    }

    pub fn thermal_state(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "trạng thái nhiệt",
            Language::English => "Thermal state",
        }
    }

    pub fn smc_temp(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "nhiệt độ SMC",
            Language::English => "SMC temp",
        }
    }

    // GPU type values (not labels)
    pub fn gpu_type_integrated(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Tích hợp",
            Language::English => "Integrated",
        }
    }

    pub fn gpu_type_discrete(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Rời",
            Language::English => "Discrete",
        }
    }

    pub fn gpu_type_unknown(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Không rõ",
            Language::English => "Unknown",
        }
    }

    /// Translate GPU type string (Integrated/Discrete/Unknown)
    pub fn translate_gpu_type(&self, gpu_type: &str) -> String {
        match gpu_type {
            "Integrated" => self.gpu_type_integrated().to_string(),
            "Discrete" => self.gpu_type_discrete().to_string(),
            "Unknown" => self.gpu_type_unknown().to_string(),
            other => other.to_string(),
        }
    }

    // Torture test translations
    pub fn torture_test(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "KIỂM TRA ĐỐT LÒ (TORTURE TEST)",
            Language::English => "TORTURE TEST (System Stress)",
        }
    }

    pub fn torture_warning(&self) -> &str {
        match self.lang {
            Language::Vietnamese => {
                "Đây là bài test cường độ cao. Hệ thống sẽ bị đẩy tới giới hạn."
            }
            Language::English => {
                "This is a high-intensity test. Your system will be pushed to its limits."
            }
        }
    }

    pub fn torture_warning_psu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Máy có thể tắt đột ngột nếu nguồn (PSU) yếu",
            Language::English => "System may shut down if power supply (PSU) is weak",
        }
    }

    pub fn torture_warning_thermal(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Nhiệt độ sẽ tăng cao, quát sẽ chạy rất mạnh",
            Language::English => "Temperatures will rise, fans will run at maximum speed",
        }
    }

    pub fn torture_warning_fans(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Quát kêu to là BÌNH THƯỜNG trong bài test này",
            Language::English => "Loud fans are NORMAL during this test",
        }
    }

    #[allow(dead_code)]
    pub fn duration(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Thời lượng",
            Language::English => "Duration",
        }
    }

    pub fn seconds(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "giây",
            Language::English => "seconds",
        }
    }

    pub fn torture_cancel_info(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Nhấn Ctrl+C để hủy",
            Language::English => "Press Ctrl+C to cancel",
        }
    }

    pub fn torture_confirm(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Tiếp tục?",
            Language::English => "Continue?",
        }
    }

    pub fn torture_cancelled(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đã hủy bài test",
            Language::English => "Test cancelled",
        }
    }

    pub fn torture_starting(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang bắt đầu",
            Language::English => "Starting",
        }
    }

    pub fn torture_summary(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "TÓM TẮT BÀI TEST TỔNG",
            Language::English => "TORTURE TEST SUMMARY",
        }
    }

    pub fn torture_duration(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Thời gian chạy",
            Language::English => "Duration",
        }
    }

    pub fn torture_passed(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "HỆ THỐNG ĐÃ VƯỢT QUA BÀI TEST TỔNG! Máy ổn định.",
            Language::English => "SYSTEM SURVIVED THE TORTURE TEST! Hardware is stable.",
        }
    }

    pub fn torture_failed(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "HỆ THỐNG CÓ VẤN ĐỀ. Xem chi tiết bên trên.",
            Language::English => "SYSTEM HAS ISSUES. See details above.",
        }
    }

    #[allow(dead_code)]
    pub fn torture_dashboard_warning(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "QUÁT SẼ KÊU RẤT TO - ĐÂY LÀ BÌNH THƯỜNG",
            Language::English => "LOUD FANS ARE NORMAL DURING THIS TEST",
        }
    }

    // Dashboard labels for all components
    pub fn torture_cpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "CPU",
            Language::English => "CPU",
        }
    }

    pub fn torture_gpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "GPU",
            Language::English => "GPU",
        }
    }

    pub fn torture_ram(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "RAM",
            Language::English => "RAM",
        }
    }

    pub fn torture_disk(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Ổ cứng",
            Language::English => "Disk",
        }
    }

    pub fn torture_load(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "tải",
            Language::English => "load",
        }
    }

    pub fn torture_errors(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "lỗi",
            Language::English => "errors",
        }
    }

    pub fn torture_write(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "ghi",
            Language::English => "write",
        }
    }

    pub fn torture_read(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "đọc",
            Language::English => "read",
        }
    }

    pub fn torture_mb_s(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "MB/s",
            Language::English => "MB/s",
        }
    }

    pub fn torture_na(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "N/A",
            Language::English => "N/A",
        }
    }

    // Level selection prompt
    pub fn select_test_level(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Chọn mức độ kiểm tra",
            Language::English => "Select test level",
        }
    }

    pub fn level_quick(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Nhanh",
            Language::English => "Quick",
        }
    }

    pub fn level_normal(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Thường",
            Language::English => "Normal",
        }
    }

    pub fn level_deep(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Chuyên sâu",
            Language::English => "Deep",
        }
    }

    pub fn your_choice(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Lựa chọn của bạn",
            Language::English => "Your choice",
        }
    }

    pub fn invalid_choice(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Lựa chọn không hợp lệ. Vui lòng chọn 1, 2 hoặc 3.",
            Language::English => "Invalid choice. Please select 1, 2, or 3.",
        }
    }

    pub fn torture_final(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "BÀI TEST TỔNG (TORTURE TEST)",
            Language::English => "FINAL TEST (TORTURE TEST)",
        }
    }

    // AI Technician personality
    pub fn ai_greet(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Xin chào! Để tôi khám sức khỏe cho chiếc máy này nhé.",
            Language::English => "Hello! Let me check the health of this machine.",
        }
    }

    pub fn ai_detecting(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Đang ngó qua cấu hình phần cứng một chút...",
            Language::English => "Taking a quick look at the hardware configuration...",
        }
    }

    pub fn ai_specs_good(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Chà, máy ngon đấy! Cấu hình này dư sức làm việc nặng.",
            Language::English => "Wow, nice machine! This config can handle heavy workloads.",
        }
    }

    pub fn ai_specs_ok(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Cấu hình ổn định, đủ dùng cho công việc hàng ngày.",
            Language::English => "Decent configuration, good enough for daily tasks.",
        }
    }

    pub fn ai_stress_intro(&self, _component: &str) -> &str {
        match self.lang {
            Language::Vietnamese => "Được rồi, bây giờ tôi sẽ ép xung để xem tản nhiệt thế nào.",
            Language::English => "Alright, now I'll stress test to see how the cooling performs.",
        }
    }

    pub fn ai_complete(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Xong! Đã hoàn thành bài kiểm tra.",
            Language::English => "Done! Test completed.",
        }
    }

    pub fn ai_start(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Bắt đầu kiểm tra...",
            Language::English => "Starting check...",
        }
    }

    // AI post-test reactions
    pub fn ai_pass(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Hoàn thành tốt. Không phát hiện vấn đề.",
            Language::English => "Test passed. No issues detected.",
        }
    }

    pub fn ai_warning(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Hoàn thành nhưng có cảnh báo. Nên kiểm tra lại.",
            Language::English => "Test completed with warnings. Recommend review.",
        }
    }

    pub fn ai_fail(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Thất bại. Cần kiểm tra ngay.",
            Language::English => "Test failed. Immediate attention needed.",
        }
    }
}
