mod error;
pub mod platform;
pub mod shared;
mod traits;

pub type ImageBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub use traits::*;

pub fn get_best_window_capture_provider() -> impl WindowCaptureProvider {
    #[cfg(target_os = "linux")]
    {
        platform::linux::X11Provider::new()
    }
    #[cfg(target_os = "windows")]
    {
        platform::windows::GdiProvider::new()
    }
}

pub fn get_best_screen_capture_provider() -> impl ScreenCaptureProvider {
    #[cfg(target_os = "linux")]
    {
        platform::linux::X11Provider::new()
    }
    #[cfg(target_os = "windows")]
    {
        platform::windows::GdiProvider::new()
    }
}

pub fn get_best_area_capture_provider() -> impl AreaCaptureProvider {
    #[cfg(target_os = "linux")]
    {
        platform::linux::X11Provider::new()
    }
    #[cfg(target_os = "windows")]
    {
        platform::windows::GdiProvider::new()
    }
}

pub fn get_best_full_capture_provider() -> impl FullCaptureProvider {
    #[cfg(target_os = "linux")]
    {
        platform::linux::X11Provider::new()
    }
    #[cfg(target_os = "windows")]
    {
        platform::windows::GdiProvider::new()
    }
}
