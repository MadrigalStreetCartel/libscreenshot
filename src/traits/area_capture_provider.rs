use crate::{error::Result, shared::Area, ImageBuffer};

pub trait AreaCaptureProvider {
    fn capture_area(&self, area: Area) -> Result<ImageBuffer>;
}
