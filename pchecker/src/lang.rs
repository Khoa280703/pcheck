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
        "CPU"
    }

    pub fn gpu(&self) -> &str {
        "GPU"
    }

    pub fn ram(&self) -> &str {
        "RAM"
    }

    pub fn disk(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Ứ CỨNG",
            Language::English => "DISK",
        }
    }

    pub fn cores(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "nhân",
            Language::English => "cores",
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
}
