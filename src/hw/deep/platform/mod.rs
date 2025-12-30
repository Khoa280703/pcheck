// Platform-specific deep hardware probes

pub mod macos;

// Re-export platform probe (unused but reserved for future direct access)
#[allow(unused_imports)]
#[cfg(target_os = "macos")]
pub use macos::MacOsDeepProbe;
