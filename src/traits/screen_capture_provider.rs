use crate::{error::Result, shared::ScreenId, ImageBuffer};

pub trait ScreenCaptureProvider {
    fn capture_screen(&self, screen_id: ScreenId) -> Result<ImageBuffer>;
    fn capture_current_screen(&self) -> Result<ImageBuffer>;
}
