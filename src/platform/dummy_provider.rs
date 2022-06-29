use crate::{
    error::Result,
    shared::{Area, ScreenId, WindowId},
    traits::*
};

pub struct DummyProvider;

impl DummyProvider {
    pub fn new() -> Self {
        DummyProvider
    }
}

impl WindowCaptureProvider for DummyProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl ScreenCaptureProvider for DummyProvider {
    fn capture_screen(&self, screen_id: ScreenId) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_current_screen(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl AreaCaptureProvider for DummyProvider {
    fn capture_area(&self, area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl FullCaptureProvider for DummyProvider {
    fn capture_full(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}
