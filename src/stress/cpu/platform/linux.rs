// Linux CPU display formatting

use crate::fmt::{RESET, CYAN, GREEN, DARK_GRAY};

/// Cores per row in verbose mode on Linux
pub fn cores_per_row_verbose() -> usize {
    3
}

/// Cores per row in normal mode on Linux
pub fn cores_per_row_normal() -> usize {
    4
}

/// Format a single core display in verbose mode
/// Linux: C00: [████░░░░░░] 95% @4.2GHz (with frequency)
pub fn format_core_display_verbose(
    i: usize,
    usage_int: u32,
    bar_filled: usize,
) -> String {
    let bar_str = format!(
        "{}{}{}{}",
        GREEN,
        "█".repeat(bar_filled),
        DARK_GRAY,
        "░".repeat(10 - bar_filled)
    );

    // Note: caller needs to add frequency separately
    format!(
        "{}C{:02}:{} [{}] {}%",
        CYAN, i, RESET, bar_str, usage_int
    )
}

/// Format a single core display in normal mode
/// Linux: C00:95% (usage only, frequency not shown in normal mode)
pub fn format_core_display_normal(
    i: usize,
    display_usage: f32,
) -> String {
    use crate::fmt::usage_color;
    let color = usage_color(display_usage);
    let usage_str = format!("{}%", display_usage as u32);
    format!("{}C{}:{}{}{} ", CYAN, i, color, usage_str, RESET)
}

/// Get per-core frequency for display (Linux may have per-core freq)
pub fn get_core_frequency_mhz(core_mhz: Option<u32>) -> f64 {
    core_mhz.unwrap_or(0) as f64 / 1000.0
}
