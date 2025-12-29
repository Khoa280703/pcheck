// AI Technician Module - Humanizing CLI
// Adds typewriter effect and artificial delays

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crate::lang::Text;
use crate::lang::Language;

#[derive(Clone)]
pub struct AiTechnician {
    pub enabled: bool,
    typewriter_delay_ms: u64,
}

impl AiTechnician {
    pub fn new(_lang: Language) -> Self {
        Self {
            enabled: true,  // Always on as per user decision
            typewriter_delay_ms: 10,  // Faster typewriter for better UX
        }
    }

    /// Typewriter effect - print text character by character
    pub fn type_print(&self, text: &str) {
        if !self.enabled {
            println!("{}", text);
            return;
        }

        for ch in text.chars() {
            print!("{}", ch);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(self.typewriter_delay_ms));
        }
        println!();
    }

    /// Artificial delay to simulate "reading" or "thinking"
    pub fn think(&self, duration_ms: u64) {
        if self.enabled {
            thread::sleep(Duration::from_millis(duration_ms));
        }
    }

    /// Print AI greeting
    pub fn greet(&self, text: &Text) {
        if !self.enabled {
            return;
        }

        println!();
        self.type_print(&format!("👨‍💻 AI: {}", text.ai_greet()));
        self.think(500);
        println!();
    }

    /// Print AI intro for hardware detection
    pub fn intro_detect(&self, text: &Text) {
        if !self.enabled {
            return;
        }

        self.type_print(&format!("🔍 AI: {}", text.ai_detecting()));
        self.think(300);
    }

    /// Print AI reaction to hardware specs
    pub fn react_specs(&self, text: &Text, is_good: bool) {
        if !self.enabled {
            return;
        }

        let reaction = if is_good {
            text.ai_specs_good()
        } else {
            text.ai_specs_ok()
        };
        self.type_print(&format!("   -> {}", reaction));
        self.think(500);
    }

    /// Real-time comment during test (no typewriter for speed)
    pub fn comment_realtime(&self, text: &str) {
        if !self.enabled {
            return;
        }

        println!("💬 AI: {}", text);
    }

    /// Post-test reaction based on result
    pub fn react_result(&self, text: &Text, is_pass: bool, has_warning: bool) {
        if !self.enabled {
            return;
        }

        let message = if is_pass {
            text.ai_pass()
        } else if has_warning {
            text.ai_warning()
        } else {
            text.ai_fail()
        };

        println!();
        self.type_print(&format!("💬 AI: {}", message));
        println!();
    }
}
