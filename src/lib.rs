mod error;
pub mod platform;
pub mod prelude;
pub mod shared;
pub mod traits;

pub type ImageBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub use traits::*;

#[allow(unreachable_code)]
pub fn get_capture_provider() -> Option<impl Provider> {
    #[cfg(target_os = "linux")]
    return Some(platform::linux::X11Provider::new());
    #[cfg(target_os = "windows")]
    return Some(platform::windows::GdiProvider::new());
    #[cfg(target_os = "macos")]
    return Some(platform::macos::CGProvider::new());
    return None;
}

#[allow(unreachable_code)]
pub fn get_window_capture_provider() -> Option<impl WindowCaptureProvider> {
    #[cfg(target_os = "linux")]
    return Some(platform::linux::X11Provider::new());
    #[cfg(target_os = "windows")]
    return Some(platform::windows::GdiProvider::new());
    #[cfg(target_os = "macos")]
    return Some(platform::macos::CGProvider::new());
    return None;
}

#[allow(unreachable_code)]
pub fn get_screen_capture_provider() -> Option<Box<dyn ScreenCaptureProvider>> {
    None
}

#[allow(unreachable_code)]
pub fn get_area_capture_provider() -> Option<Box<dyn AreaCaptureProvider>> {
    #[cfg(target_os = "linux")]
    return Some(Box::new(platform::linux::X11Provider::new()));
    return None;
}

#[allow(unreachable_code)]
pub fn get_full_capture_provider() -> Option<Box<dyn FullCaptureProvider>> {
    #[cfg(target_os = "linux")]
    return Some(Box::new(platform::linux::X11Provider::new()));
    #[allow(unreachable_code)]
    return None;
}
