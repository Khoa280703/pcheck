// Platform-specific CPU display and formatting

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

// Re-export platform-specific functions
#[cfg(target_os = "macos")]
pub use macos::{cores_per_row_verbose, cores_per_row_normal, format_core_display_verbose, format_core_display_normal};

#[cfg(target_os = "windows")]
pub use windows::{cores_per_row_verbose, cores_per_row_normal, format_core_display_verbose, format_core_display_normal};

#[cfg(target_os = "linux")]
pub use linux::{cores_per_row_verbose, cores_per_row_normal, format_core_display_verbose, format_core_display_normal};
