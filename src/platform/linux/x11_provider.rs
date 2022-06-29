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

    impl Rect {
        pub fn to_client_coordinates(&self) -> Rect {
            Rect { x: 0, y: 0, w: self.w, h: self.h }
        }
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

        pub unsafe fn open_default_display() -> Result<Self> {
            Self::open(None)
        }

        pub unsafe fn get_focused_window(&self) -> xlib::Window {
            let mut window = 0x0 as xlib::Window;
            let window_ptr = &mut window as *mut u64;
            let mut revert_to = 0;
            let revert_to_ptr = &mut revert_to as *mut i32;
            let _ = xlib::XGetInputFocus(**self, window_ptr, revert_to_ptr);
            window
        }

        pub unsafe fn get_client_rect(&self, window_id: xlib::Window) -> Rect {
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
            client_rect: Rect,
        ) -> Result<XImageHandle> {
            const ALL_PLANES: u64 = !0;
            match xlib::XGetImage(
                **self,
                window_id,
                client_rect.x,
                client_rect.y,
                client_rect.w,
                client_rect.h,
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
            #[inline]
            unsafe fn channel_offset(handle: &XImageHandle, mask: u64) -> Result<u32> {
                match ((***handle).byte_order, mask & 0xFFFFFFFF) {
                    (0, 0xFF) | (1, 0xFF000000) => Ok(0),
                    (0, 0xFF00) | (1, 0xFF0000) => Ok(1),
                    (0, 0xFF0000) | (1, 0xFF00) => Ok(2),
                    (0, 0xFF000000) | (1, 0xFF) => Ok(3),
                    _ => Err(Error::WindowCaptureFailed),
                }
            }

            #[inline]
            unsafe fn subpixel_at(handle: &XImageHandle, x: u32, y: u32, data: &[u8], stride: u32, channel_offset: u32) -> u8 {
                let index = y * (***handle).bytes_per_line as u32 + x * stride + channel_offset;
                data[index as usize]
            }

            unsafe {
                let stride = match ((**self).depth, (**self).bits_per_pixel) {
                    (24, 24) => Ok(3),
                    (24, 32) | (32, 32) => Ok(4),
                    _ => Err(Error::WindowCaptureFailed),
                }?;

                let (mask_r, mask_g, mask_b) = ((**self).red_mask, (**self).green_mask, (**self).blue_mask);
                let mask_a = !(mask_r | mask_g | mask_b);
                let offset_r = channel_offset(&self, mask_r)?;
                let offset_g = channel_offset(&self, mask_g)?;
                let offset_b = channel_offset(&self, mask_b)?;
                let offset_a = channel_offset(&self, mask_a)?;
                let size = ((**self).bytes_per_line * (**self).height) as usize;
                let data = std::slice::from_raw_parts((**self).data as *const u8, size);

                let image = ImageBuffer::from_fn((**self).width as u32, (**self).height as u32, |x, y| {
                    *image::Pixel::from_slice(&[
                        subpixel_at(&self, x, y, data, stride, offset_r),
                        subpixel_at(&self, x, y, data, stride, offset_g),
                        subpixel_at(&self, x, y, data, stride, offset_b),
                        if (**self).depth == 24 {
                            0xFF
                        } else {
                            subpixel_at(&self, x, y, data, stride, offset_a)
                        },
                    ])
                });

                Ok(image)
            }
        }
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
            let display = xutils::XDisplayHandle::open_default_display()?;
            let window_rect = display.get_client_rect(window_id);
            let client_rect = window_rect.to_client_coordinates();
            let ximage = display.get_image(window_id, client_rect)?;
            let image: ImageBuffer = ximage.try_into()?;
            Ok(image)
        }
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unsafe {
            let window = xutils::XDisplayHandle::open_default_display()?.get_focused_window();
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
