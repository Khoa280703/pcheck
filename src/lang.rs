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
}
