use crate::{error::Result, shared::WindowId, ImageBuffer};

pub trait WindowCaptureProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer>;
    fn capture_focused_window(&self) -> Result<ImageBuffer>;
}
