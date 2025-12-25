// Language module - Multi-language support for pchecker

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Vietnamese,
    English,
}

impl Language {
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "vi" | "vietnamese" => Some(Language::Vietnamese),
            "en" | "english" => Some(Language::English),
            _ => None,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Language::Vietnamese => "vi",
            Language::English => "en",
        }
    }
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
            Language::Vietnamese => "CPU",
            Language::English => "CPU",
        }
    }

    pub fn gpu(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "GPU",
            Language::English => "GPU",
        }
    }

    pub fn ram(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "RAM",
            Language::English => "RAM",
        }
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

    pub fn select_prompt(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Chọn ngôn ngữ:\n\n  [1] Tiếng Việt\n  [2] English\n\nLựa chọn của bạn [1-2]: ",
            Language::English => "Select language:\n\n  [1] Tiếng Việt\n  [2] English\n\nYour choice [1-2]: ",
        }
    }

    pub fn invalid_choice(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "Lựa chọn không hợp lệ. Vui lòng chọn 1 hoặc 2.",
            Language::English => "Invalid choice. Please select 1 or 2.",
        }
    }

    pub fn platform_name(&self) -> &str {
        match self.lang {
            Language::Vietnamese => "macOS (Apple Silicon)",
            Language::English => "macOS (Apple Silicon)",
        }
    }
}
