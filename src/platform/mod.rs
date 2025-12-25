// Platform detection module
// Provides OS-specific implementations using Strategy Pattern

use std::fmt;

/// Platform trait for OS-specific operations
pub trait Platform: fmt::Display {}

/// macOS platform implementation
#[cfg(target_os = "macos")]
pub struct MacOS;

#[cfg(target_os = "macos")]
impl Platform for MacOS {}

#[cfg(target_os = "macos")]
impl fmt::Display for MacOS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "macOS (Apple Silicon)")
    }
}

/// Windows platform implementation
#[cfg(target_os = "windows")]
pub struct Windows;

#[cfg(target_os = "windows")]
impl Platform for Windows {}

#[cfg(target_os = "windows")]
impl fmt::Display for Windows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Windows")
    }
}

/// Linux platform implementation
#[cfg(target_os = "linux")]
pub struct Linux;

#[cfg(target_os = "linux")]
impl Platform for Linux {}

#[cfg(target_os = "linux")]
impl fmt::Display for Linux {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Linux")
    }
}

/// Detect current platform at runtime
pub fn detect() -> Box<dyn Platform> {
    #[cfg(target_os = "macos")]
    return Box::new(MacOS);

    #[cfg(target_os = "windows")]
    return Box::new(Windows);

    #[cfg(target_os = "linux")]
    return Box::new(Linux);

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    compile_error!("Unsupported platform");
}
