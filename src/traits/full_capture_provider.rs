use crate::{error::Result, ImageBuffer};

pub trait FullCaptureProvider {
    fn capture_full(&self) -> Result<ImageBuffer>;
}
