#[cfg(feature = "macos")]
mod cg_provider;

#[cfg(feature = "macos")]
pub use self::cg_provider::CGProvider;

#[cfg(feature = "macos")]
pub mod macos_helper {
    use std::ffi::c_void;

    use cocoa::{appkit::NSEvent, base::id};
    use core_graphics::window::CGWindowID;

    pub unsafe fn ns_window_to_window_id(ptr: *const c_void) -> Option<CGWindowID> {
        let id = ptr as id;
        if !id.is_null() {
            Some(id.windowNumber() as u32)
        } else {
            None
        }
    }
}