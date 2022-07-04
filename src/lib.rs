mod error;
pub mod platform;
pub mod prelude;
pub mod shared;
pub mod traits;

pub type ImageBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub use traits::*;

pub fn get_window_capture_provider() -> Option<Box<dyn WindowCaptureProvider>> {
    #[cfg(target_os = "linux")]
    return Some(Box::new(platform::linux::X11Provider::new()));
    #[cfg(target_os = "windows")]
    return Some(Box::new(platform::windows::GdiProvider::new()));
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    return None;
}

pub fn get_screen_capture_provider() -> Option<Box<dyn ScreenCaptureProvider>> {
    None
}

pub fn get_area_capture_provider() -> Option<Box<dyn AreaCaptureProvider>> {
    None
}

pub fn get_full_capture_provider() -> Option<Box<dyn FullCaptureProvider>> {
    #[cfg(target_os = "linux")]
    return Some(Box::new(platform::linux::X11Provider::new()));
    #[cfg(not(target_os = "linux"))]
    return None;
}
