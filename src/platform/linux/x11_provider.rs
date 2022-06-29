use crate::{error::*, shared::*, traits::*, ImageBuffer};

use x11::xlib;

mod xutils {
    use std::ops::Deref;
    use x11::xlib;

    use crate::{error::*, ImageBuffer};

    pub struct XDisplayHandle(*mut xlib::Display);

    pub struct XImageHandle(*mut xlib::XImage);

    #[allow(dead_code)]
    pub struct Rect {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
    }

    impl Deref for XDisplayHandle {
        type Target = *mut xlib::Display;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Deref for XImageHandle {
        type Target = *mut xlib::XImage;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Drop for XDisplayHandle {
        fn drop(&mut self) {
            unsafe {
                xlib::XCloseDisplay(**self);
            }
        }
    }

    impl Drop for XImageHandle {
        fn drop(&mut self) {
            unsafe {
                xlib::XDestroyImage(**self);
            }
        }
    }

    impl XDisplayHandle {
        pub unsafe fn open(name: Option<std::ffi::CString>) -> Result<Self> {
            let name = match name {
                None => std::ptr::null(),
                Some(cstr) => cstr.as_ptr(),
            };
            match xlib::XOpenDisplay(name) {
                d if d.is_null() => Err(Error::WindowCaptureFailed),
                d => Ok(XDisplayHandle(d)),
            }
        }

        pub unsafe fn get_window_rect(&self, window_id: xlib::Window) -> Rect {
            let mut attrs = std::mem::MaybeUninit::uninit();

            xlib::XGetWindowAttributes(**self, window_id, attrs.as_mut_ptr());

            let attrs = attrs.assume_init();
            let mut root = 0;
            let mut parent = 0;
            let mut children: *mut xlib::Window = std::ptr::null_mut();
            let mut nchildren = 0;

            xlib::XQueryTree(
                **self,
                window_id,
                &mut root,
                &mut parent,
                &mut children,
                &mut nchildren,
            );

            if !children.is_null() {
                xlib::XFree(children as *mut std::os::raw::c_void);
            }

            let mut x = attrs.x;
            let mut y = attrs.y;
            let w = attrs.width as u32;
            let h = attrs.height as u32;

            if parent != 0 {
                let mut child = 0;
                xlib::XTranslateCoordinates(
                    **self, parent, root, attrs.x, attrs.y, &mut x, &mut y, &mut child,
                );
            }

            Rect { x, y, w, h }
        }

        pub unsafe fn get_image(
            &self,
            window_id: xlib::Window,
            rect: Rect,
        ) -> Result<XImageHandle> {
            const ALL_PLANES: u64 = !0;
            match xlib::XGetImage(
                **self,
                window_id,
                0,
                0,
                rect.w,
                rect.h,
                ALL_PLANES,
                xlib::ZPixmap,
            ) {
                d if d.is_null() => Err(Error::WindowCaptureFailed),
                d => Ok(XImageHandle(d)),
            }
        }
    }

    impl TryInto<ImageBuffer> for XImageHandle {
        type Error = Error;

        fn try_into(self) -> std::result::Result<ImageBuffer, Self::Error> {
            unsafe {
                macro_rules! get {
                    ($($a:ident),+) => ($(let $a = (**self).$a;)+);
                }

                get!(
                    width,
                    height,
                    byte_order,
                    depth,
                    bytes_per_line,
                    bits_per_pixel,
                    red_mask,
                    green_mask,
                    blue_mask
                );

                let stride = match (depth, bits_per_pixel) {
                    (24, 24) => Ok(3),
                    (24, 32) | (32, 32) => Ok(4),
                    _ => Err(Error::WindowCaptureFailed),
                }?;

                macro_rules! channel_offset {
                    ($mask:expr) => {
                        match (byte_order, $mask & 0xFFFFFFFF) {
                            (0, 0xFF) | (1, 0xFF000000) => Ok(0),
                            (0, 0xFF00) | (1, 0xFF0000) => Ok(1),
                            (0, 0xFF0000) | (1, 0xFF00) => Ok(2),
                            (0, 0xFF000000) | (1, 0xFF) => Ok(3),
                            _ => Err(Error::WindowCaptureFailed),
                        }
                    };
                }
                let red_offset = channel_offset!(red_mask)?;
                let green_offset = channel_offset!(green_mask)?;
                let blue_offset = channel_offset!(blue_mask)?;
                let alpha_offset = channel_offset!(!(red_mask | green_mask | blue_mask))?;
                let size = (bytes_per_line * height) as usize;
                let data = std::slice::from_raw_parts((**self).data as *const u8, size);

                let image = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
                    macro_rules! subpixel {
                        ($channel_offset:ident) => {
                            data[(y * bytes_per_line as u32 + x * stride as u32 + $channel_offset)
                                as usize]
                        };
                    }
                    *image::Pixel::from_slice(&[
                        subpixel!(red_offset),
                        subpixel!(green_offset),
                        subpixel!(blue_offset),
                        if depth == 24 {
                            0xFF
                        } else {
                            subpixel!(alpha_offset)
                        },
                    ])
                });

                Ok(image)
            }
        }
    }
}

struct X11Helper;

impl X11Helper {
    unsafe fn get_focused_window(handle: &xutils::XDisplayHandle) -> xlib::Window {
        let mut w = 0x0 as xlib::Window;
        let raw_w_ptr = &mut w as *mut u64;
        let mut revert_to = 0;
        let raw_revert_to_ptr = &mut revert_to as *mut i32;
        let _active_window = xlib::XGetInputFocus(**handle, raw_w_ptr, raw_revert_to_ptr);
        w
    }

    unsafe fn open_default_display() -> Result<xutils::XDisplayHandle> {
        xutils::XDisplayHandle::open(None)
    }
}

pub struct X11Provider;

impl Provider for X11Provider {
    fn new() -> Self {
        Self
    }
}

impl WindowCaptureProvider for X11Provider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer> {
        unsafe {
            let display = X11Helper::open_default_display()?;
            let window_rect = display.get_window_rect(window_id);
            let ximage = display.get_image(window_id, window_rect)?;
            let image: ImageBuffer = ximage.try_into()?;
            Ok(image)
        }
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unsafe {
            let window = {
                let display = X11Helper::open_default_display()?;
                X11Helper::get_focused_window(&display)
            };
            self.capture_window(window)
        }
    }
}

impl ScreenCaptureProvider for X11Provider {
    fn capture_screen(&self, _screen_id: ScreenId) -> Result<ImageBuffer> {
        unimplemented!()
    }

    fn capture_current_screen(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl AreaCaptureProvider for X11Provider {
    fn capture_area(&self, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl FullCaptureProvider for X11Provider {
    fn capture_full(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}
