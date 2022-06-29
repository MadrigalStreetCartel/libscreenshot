#[cfg(all(feature = "windows", target_os = "windows"))]
pub mod windows;

#[cfg(all(feature = "linux", target_os = "linux"))]
pub mod linux;
