use std::mem::size_of;

use windows::Win32::{
    Foundation::{GetLastError, ERROR_INVALID_PARAMETER, HWND, RECT},
    Graphics::Gdi::{
        CreateCompatibleBitmap, CreateCompatibleDC, CreatedHDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        HBITMAP, HDC, HGDIOBJ,
    },
    Storage::Xps::{PrintWindow, PRINT_WINDOW_FLAGS, PW_CLIENTONLY},
    UI::WindowsAndMessaging::{GetClientRect, GetForegroundWindow, PW_RENDERFULLCONTENT},
};

use crate::{error::*, shared::*, traits::*, ImageBuffer};

pub struct GdiHelper;

impl GdiHelper {
    unsafe fn get_dc(hwnd: HWND) -> Result<HDC> {
        let hdc = GetDC(hwnd);
        if hdc.is_invalid() {
            eprintln!("GetDC failed: {:?}", GetLastError());
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(hdc)
        }
    }

    unsafe fn get_window_rect(hwnd: HWND, hdc: HDC) -> Result<RECT> {
        let mut rect = RECT::default();
        if !GetClientRect(hwnd, &mut rect).as_bool() {
            eprintln!("GetClientRect failed: {:?}", GetLastError());
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(rect)
        }
    }

    unsafe fn create_compatible_dc(hdc: HDC) -> Result<CreatedHDC> {
        let hdc = CreateCompatibleDC(hdc);
        if hdc.is_invalid() {
            eprintln!("CreateCompatibleDC failed: {:?}", GetLastError());
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(hdc)
        }
    }

    unsafe fn create_compatible_bitmap(
        hdc: HDC,
        chdc: CreatedHDC,
        w: i32,
        h: i32,
    ) -> Result<HBITMAP> {
        let hbmp = CreateCompatibleBitmap(hdc, w, h);
        if hbmp.is_invalid() {
            eprintln!("CreateCompatibleBitmap failed: {:?}", GetLastError());
            DeleteDC(chdc);
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(hbmp)
        }
    }

    unsafe fn select_object(hdc: HDC, chdc: CreatedHDC, hbmp: HBITMAP) -> Result<HGDIOBJ> {
        let hgdiobj = SelectObject(chdc, hbmp);
        if hgdiobj.is_invalid() {
            eprintln!("SelectObject failed: {:?}", GetLastError());
            DeleteDC(chdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(hgdiobj)
        }
    }

    unsafe fn print_window(
        hwnd: HWND,
        hdc: HDC,
        chdc: CreatedHDC,
        hbmp: HBITMAP,
        flags: PRINT_WINDOW_FLAGS,
    ) -> Result<()> {
        if !PrintWindow(hwnd, chdc, flags).as_bool() {
            eprintln!("PrintWindow failed: {:?}", GetLastError());
            DeleteDC(chdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(())
        }
    }

    unsafe fn get_di_bits(
        hdc: HDC,
        chdc: CreatedHDC,
        hbmp: HBITMAP,
        h: u32,
        bmi: &mut BITMAPINFO,
        buf: &mut Vec<u8>,
    ) -> Result<()> {
        let dib = GetDIBits(
            chdc,
            hbmp,
            0,
            h,
            buf.as_mut_ptr() as *mut std::ffi::c_void,
            bmi,
            DIB_RGB_COLORS,
        );
        if dib == 0 || dib == ERROR_INVALID_PARAMETER.0 as i32 {
            eprintln!("GetDIBits failed: {:?}", GetLastError());
            DeleteDC(chdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc);
            Err(Error::WindowCaptureFailed)
        } else {
            Ok(())
        }
    }

    fn create_bitmap_info(w: i32, h: i32) -> BITMAPINFO {
        let bmp_info_header = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biSizeImage: 4 * w as u32 * h as u32,
            biPlanes: 1,
            biBitCount: 32,
            biWidth: w,
            biHeight: -h,
            biCompression: BI_RGB as u32,
            ..Default::default()
        };
        BITMAPINFO {
            bmiHeader: bmp_info_header,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GdiProvider;

impl Provider for GdiProvider {
    fn new() -> Self {
        Self
    }
}

impl WindowCaptureProvider for GdiProvider {
    fn capture_window(&self, window_id: WindowId) -> Result<ImageBuffer> {
        let hwnd = HWND(window_id.try_into()?);
        unsafe {
            let hdc = GdiHelper::get_dc(hwnd)?;
            let rect = GdiHelper::get_window_rect(hwnd, hdc)?;
            let (w, h) = (rect.right - rect.left, rect.bottom - rect.top);
            let chdc = GdiHelper::create_compatible_dc(hdc)?;
            let hbmp = GdiHelper::create_compatible_bitmap(hdc, chdc, w, h)?;
            let _hgdiobj = GdiHelper::select_object(hdc, chdc, hbmp)?;
            let mut bmpi = GdiHelper::create_bitmap_info(w, h);
            GdiHelper::print_window(
                hwnd,
                hdc,
                chdc,
                hbmp,
                PRINT_WINDOW_FLAGS(PW_RENDERFULLCONTENT | PW_CLIENTONLY.0),
            )?;
            let mut buf = {
                let mut buf = vec![0u8; (4 * w * h) as usize];
                GdiHelper::get_di_bits(hdc, chdc, hbmp, h as u32, &mut bmpi, &mut buf)?;
                buf
            };
            // Convert from BGRA to RGBA
            buf.chunks_exact_mut(4).for_each(|c| c.swap(0, 2));
            // Free handles
            DeleteDC(chdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc);
            ImageBuffer::from_raw(w as u32, h as u32, buf).ok_or(Error::WindowCaptureFailed)
        }
    }

    fn capture_focused_window(&self) -> Result<ImageBuffer> {
        unsafe {
            let hwnd = GetForegroundWindow();
            self.capture_window(hwnd.0 as u64)
        }
    }

    fn capture_window_area(&self, window_id: WindowId, area: Area) -> Result<ImageBuffer> {
        let hwnd = HWND(window_id.try_into()?);
        unsafe {
            let area = GenericArea::<i32, i32>::try_from(area)?;
            let hdc = GdiHelper::get_dc(hwnd)?;
            let chdc = GdiHelper::create_compatible_dc(hdc)?;
            let hbmp = GdiHelper::create_compatible_bitmap(hdc, chdc, area.width, area.height)?;
            let _hgdiobj = GdiHelper::select_object(hdc, chdc, hbmp)?;
            let mut bmpi = GdiHelper::create_bitmap_info(area.width, area.height);
            GdiHelper::print_window(
                hwnd,
                hdc,
                chdc,
                hbmp,
                PRINT_WINDOW_FLAGS(PW_RENDERFULLCONTENT | PW_CLIENTONLY.0),
            )?;
            let mut buf = {
                let mut buf = vec![0u8; (4 * area.width * area.height) as usize];
                GdiHelper::get_di_bits(hdc, chdc, hbmp, area.height as u32, &mut bmpi, &mut buf)?;
                buf
            };
            // Convert from BGRA to RGBA
            buf.chunks_exact_mut(4).for_each(|c| c.swap(0, 2));
            // Free handles
            DeleteDC(chdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc);
            ImageBuffer::from_raw(area.width as u32, area.height as u32, buf).ok_or(Error::WindowCaptureFailed)
        }
    }

    fn capture_focused_window_area(&self, area: Area) -> Result<ImageBuffer> {
        unsafe {
            let hwnd = GetForegroundWindow();
            self.capture_window_area(hwnd.0 as u64, area)
        }
    }
}

impl ScreenCaptureProvider for GdiProvider {
    fn capture_screen(&self, _screen_id: ScreenId) -> Result<ImageBuffer> {
        unimplemented!()
        // unsafe {
        //     let hdc = GdiHelper::get_dc(HWND::default())?;
        //     let chdc = GdiHelper::create_compatible_dc(hdc)?;
        // }
    }

    fn capture_current_screen(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl AreaCaptureProvider for GdiProvider {
    fn capture_area(&self, _area: Area) -> Result<ImageBuffer> {
        unimplemented!()
    }
}

impl FullCaptureProvider for GdiProvider {
    fn capture_full(&self) -> Result<ImageBuffer> {
        unimplemented!()
    }
}
