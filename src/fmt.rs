// Output formatting module

use std::time::Instant;

const SEPARATOR: &str = "============================================================";

// ANSI color codes for terminal output
pub const RESET: &str = "\x1b[0m";
pub const CYAN: &str = "\x1b[36m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const GREEN: &str = "\x1b[32m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const RED: &str = "\x1b[31m";
pub const DARK_GRAY: &str = "\x1b[90m";

/// Get color for temperature value
pub fn temp_color(temp: f32) -> &'static str {
    if temp < 60.0 {
        GREEN
    } else if temp < 75.0 {
        YELLOW
    } else if temp < 85.0 {
        ORANGE
    } else {
        RED
    }
}

/// Get temperature status text (requires Text for i18n)
#[allow(dead_code)]  // Reserved for future i18n features
pub fn temp_status_i18n(temp: f32, text: &crate::lang::Text) -> String {
    let status = if temp < 60.0 {
        text.temp_status_excellent()
    } else if temp < 75.0 {
        text.temp_status_stable()
    } else if temp < 85.0 {
        text.temp_status_warm()
    } else {
        text.temp_status_hot()
    };
    let icon = if temp < 75.0 { "✅" } else if temp < 85.0 { "⚠️" } else { "❌" };
    format!("{} {}", icon, status)
}

/// Legacy temp_status - kept for compatibility but deprecated
#[allow(dead_code)]
pub fn temp_status(temp: f32) -> &'static str {
    if temp < 60.0 {
        "✅ Rất tốt"
    } else if temp < 75.0 {
        "✅ Ổn định"
    } else if temp < 85.0 {
        "⚠️ Ấm"
    } else {
        "❌ Nóng"
    }
}

/// Get color for CPU usage % (consistent with temperature colors)
pub fn usage_color(usage: f32) -> &'static str {
    if usage > 90.0 {
        RED
    }
    // Overload - same as "Nóng"
    else if usage > 50.0 {
        GREEN
    }
    // Active - same as "Rất tốt"
    else {
        DARK_GRAY
    } // Idle
}

/// Format large number with suffix (Billion, Trillion) - i18n version
#[allow(dead_code)]  // Reserved for future i18n features
pub fn format_large_number_i18n(n: u64, text: &crate::lang::Text) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1} {}", n as f64 / 1_000_000_000.0, text.billion_suffix())
    } else if n >= 1_000_000 {
        format!("{:.1} {}", n as f64 / 1_000_000.0, text.million_suffix())
    } else {
        format_number(n)
    }
}

/// Legacy format_large_number - kept for compatibility but deprecated
#[allow(dead_code)]
pub fn format_large_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1} Tỷ", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1} Triệu", n as f64 / 1_000_000.0)
    } else {
        format_number(n)
    }
}

/// Format number with thousands separator
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*c);
    }
    result
}

/// Create progress bar string
pub fn progress_bar(percent: u8, width: usize) -> String {
    let filled = (percent as usize * width / 100).min(width);
    let empty = width - filled;
    format!(
        "{}{}{}{}{}",
        GREEN,
        "█".repeat(filled),
        DARK_GRAY,
        "░".repeat(empty),
        RESET
    )
}

pub fn print_header_with_text(version: &str, tagline: &str) {
    println!("{}", SEPARATOR);
    println!("🤖 PCHECKER {} - {}", version, tagline);
    println!("{}", SEPARATOR);
    println!();
}

pub fn print_section(icon: &str, label: &str, value: &str) {
    println!("{} {:<12}{}", icon, label, value);
}

pub fn print_footer_with_text(start_time: Instant, done_text: &str) {
    let elapsed = start_time.elapsed();
    let time_str = if elapsed.as_secs() == 0 {
        format!("{:.2}s", elapsed.as_secs_f64())
    } else {
        format!("{}s", elapsed.as_secs())
    };

    println!();
    println!("{}", SEPARATOR);
    println!("{} {}", done_text, time_str);
    println!("{}", SEPARATOR);
}
