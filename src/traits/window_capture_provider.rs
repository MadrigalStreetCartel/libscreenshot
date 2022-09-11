use crate::{error::Result, shared::{WindowId, Area}, ImageBuffer};

pub trait WindowCaptureProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer>;
    fn capture_window_area(&self, window_id: WindowId, area: Area) -> Result<ImageBuffer>;
    fn capture_focused_window(&self) -> Result<ImageBuffer>;
    fn capture_focused_window_area(&self, area: Area) -> Result<ImageBuffer>;
}
