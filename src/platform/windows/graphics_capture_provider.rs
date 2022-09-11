use crate::{error::*, shared::*, traits::*, ImageBuffer};

#[derive(Default)]
pub struct GraphicsCaptureProvider;

impl Provider for GraphicsCaptureProvider {
    fn new() -> Self {
        Self
    }
}

impl GraphicsCaptureProvider {
    pub fn new() -> Self {
        GraphicsCaptureProvider
    }
}

impl WindowCaptureProvider for GraphicsCaptureProvider {
    fn capture_window(&self, _window_id: WindowId) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_window_area(&self, _window_id: WindowId, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_focused_window_area(&self, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl ScreenCaptureProvider for GraphicsCaptureProvider {
    fn capture_screen(&self, _screen_id: ScreenId) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_current_screen(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl AreaCaptureProvider for GraphicsCaptureProvider {
    fn capture_area(&self, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl FullCaptureProvider for GraphicsCaptureProvider {
    fn capture_full(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}
