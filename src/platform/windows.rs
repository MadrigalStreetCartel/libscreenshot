#[cfg(feature = "windows_gdi")]
mod gdi_provider;
#[cfg(feature = "windows_gdi")]
pub use self::gdi_provider::GdiProvider;

#[cfg(feature = "windows_graphics_capture")]
mod graphics_capture_provider;
#[cfg(feature = "windows_graphics_capture")]
pub use self::graphics_capture_provider::GraphicsCaptureProvider;
