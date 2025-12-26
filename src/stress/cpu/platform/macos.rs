// macOS CPU display formatting

use crate::fmt::{RESET, CYAN, GREEN, DARK_GRAY};

/// Cores per row in verbose mode on macOS
pub fn cores_per_row_verbose() -> usize {
    4
}

/// Cores per row in normal mode on macOS
pub fn cores_per_row_normal() -> usize {
    6
}

/// Format a single core display in verbose mode
/// macOS: C00: [████░░░░░░] 95% (no frequency)
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

    format!(
        "{}C{:02}:{} [{}] {}%",
        CYAN, i, RESET, bar_str, usage_int
    )
}

/// Format a single core display in normal mode
/// macOS: C00:95% (usage only)
pub fn format_core_display_normal(
    i: usize,
    display_usage: f32,
) -> String {
    use crate::fmt::usage_color;
    let color = usage_color(display_usage);
    let usage_str = format!("{}%", display_usage as u32);
    format!("{}C{}:{}{}{} ", CYAN, i, color, usage_str, RESET)
}
