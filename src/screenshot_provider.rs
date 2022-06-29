use crate::error::Result;
use crate::shared::{Area, ScreenId, WindowId};
use crate::ImageBuffer;

pub trait WindowCaptureProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer>;
    fn capture_focused_window(&self) -> Result<ImageBuffer>;
}

pub trait ScreenCaptureProvider {
    fn capture_screen(&self, screen_id: ScreenId) -> Result<()>;
    fn capture_current_screen(&self) -> Result<()>;
}

pub trait AreaCaptureProvider {
    fn capture_area(&self, area: Area) -> Result<()>;
}

pub trait FullCaptureProvider {
    fn capture_full(&self) -> Result<()>;
}
