use std::ffi::c_void;

use core_foundation::{
    base::{CFGetTypeID, CFRelease, CFTypeRef, ToVoid},
    number::{kCFNumberSInt32Type, CFNumberGetTypeID, CFNumberGetValue, CFNumberRef},
};

use core_graphics::{
    base::boolean_t,
    display::{
        CFArrayGetCount, CFArrayGetValueAtIndex, CFDictionaryGetValueIfPresent, CFDictionaryRef,
        CGDisplay, CGRect, CGRectNull, CGWindowListCopyWindowInfo,
    },
    image::CGImage,
    window::{
        kCGNullWindowID, kCGWindowImageBestResolution, kCGWindowImageBoundsIgnoreFraming,
        kCGWindowListExcludeDesktopElements, kCGWindowListOptionIncludingWindow,
        kCGWindowListOptionOnScreenBelowWindow, kCGWindowListOptionOnScreenOnly, kCGWindowNumber,
        kCGWindowOwnerPID, CGWindowID, CGWindowListCreateImage,
    },
};

use foreign_types::ForeignType;
use macos_bindings::{INSRunningApplication, INSWorkspace, NSWorkspace};

#[allow(dead_code)]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGRectMakeWithDictionaryRepresentation(
        dict: CFDictionaryRef,
        rect: *mut CGRect,
    ) -> boolean_t;
}

use crate::{error::*, shared::*, traits::*, ImageBuffer};

#[derive(Default)]
pub struct CGProvider;

impl Provider for CGProvider {
    fn new() -> Self {
        Self
    }
}

impl WindowCaptureProvider for CGProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer> {
        unsafe {
            // Capture window
            let cg_image = CGWindowListCreateImage(
                CGRectNull,
                kCGWindowListExcludeDesktopElements
                    | kCGWindowListOptionIncludingWindow
                    | kCGWindowListOptionOnScreenBelowWindow,
                window_id as CGWindowID,
                kCGWindowImageBoundsIgnoreFraming | kCGWindowImageBestResolution,
            );
            let cg_image = CGImage::from_ptr(cg_image);

            // Convert image data from BGRA layout to RGBA layout
            let mut buf = Vec::from(cg_image.data().bytes());
            buf.chunks_exact_mut(cg_image.bits_per_pixel() / 8)
                .for_each(|c| c.swap(0, 2));

            // Make image data layout contiguous
            let w = cg_image.width();
            let h = cg_image.height();
            let bpp = cg_image.bits_per_pixel() / 8;
            let bpr = cg_image.bytes_per_row();
            let mut new_buf = Vec::with_capacity(w * h * bpp);
            let mut i = 0;
            for _ in 0..cg_image.height() {
                new_buf.extend_from_slice(&buf[i..(i + w * bpp)]);
                i += bpr;
            }

            ImageBuffer::from_raw(cg_image.width() as u32, cg_image.height() as u32, new_buf)
                .ok_or(Error::WindowCaptureFailed)
        }
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unsafe {
            // Obtain focused application from shared workspace
            let shared_workspace = NSWorkspace::sharedWorkspace();
            let active_app = shared_workspace.frontmostApplication();
            let active_pid = active_app.processIdentifier();

            // Enumerate windows
            let flags = kCGWindowListExcludeDesktopElements | kCGWindowListOptionOnScreenOnly;
            let window_list_info = CGWindowListCopyWindowInfo(flags, kCGNullWindowID);
            let window_list_info_count = CFArrayGetCount(window_list_info);

            // Find window id using pid
            #[allow(non_snake_case)]
            let window_id = {
                let mut matching_dict = None;

                // Find window with matching process id
                for i in 0..(window_list_info_count - 1) {
                    let dict = CFArrayGetValueAtIndex(window_list_info, i) as CFDictionaryRef;
                    let mut raw_val: *const c_void = std::ptr::null();
                    let mut num = 0_i32;
                    let num_ptr = &mut num as *mut i32;
                    if CFDictionaryGetValueIfPresent(
                        dict,
                        kCGWindowOwnerPID.to_void(),
                        &mut raw_val,
                    ) == 1
                        && CFGetTypeID(raw_val) == CFNumberGetTypeID()
                        && CFNumberGetValue(
                            raw_val as CFNumberRef,
                            kCFNumberSInt32Type,
                            num_ptr.cast(),
                        )
                        && num == active_pid
                    {
                        matching_dict = Some(dict);
                    }
                }

                // Find window id
                match matching_dict {
                    Some(dict) => {
                        let mut raw_val: *const c_void = std::ptr::null();
                        let mut num = 0_u32;
                        let num_ptr = &mut num as *mut u32;
                        if CFDictionaryGetValueIfPresent(
                            dict,
                            kCGWindowNumber.to_void(),
                            &mut raw_val,
                        ) == 1
                            && CFGetTypeID(raw_val) == CFNumberGetTypeID()
                            && CFNumberGetValue(
                                raw_val as CFNumberRef,
                                kCFNumberSInt32Type,
                                num_ptr.cast(),
                            )
                        {
                            Some(num)
                        } else {
                            CFRelease(window_list_info as CFTypeRef);
                            None
                        }
                    }
                    None => None,
                }
            };

            match window_id {
                Some(id) => self.capture_window(id as u64),
                None => Err(Error::WindowCaptureFailed),
            }
        }
    }
}

impl ScreenCaptureProvider for CGProvider {
    fn capture_screen(&self, screen_id: ScreenId) -> Result<ImageBuffer> {
        let cg_display = CGDisplay::new(screen_id as u32);
        if let Some(cg_image) = cg_display.image() {
            let mut buf = Vec::from(cg_image.data().bytes());
            buf.chunks_exact_mut(4).for_each(|c| c.swap(0, 2));
            ImageBuffer::from_raw(cg_image.width() as u32, cg_image.height() as u32, buf)
                .ok_or(Error::WindowCaptureFailed)
        } else {
            Err(Error::WindowCaptureFailed)
        }
    }

    fn capture_current_screen(&self) -> Result<ImageBuffer> {
        let cg_display = CGDisplay::main();
        self.capture_screen(cg_display.id as u64)
    }
}

impl AreaCaptureProvider for CGProvider {
    fn capture_area(&self, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl FullCaptureProvider for CGProvider {
    fn capture_full(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}
