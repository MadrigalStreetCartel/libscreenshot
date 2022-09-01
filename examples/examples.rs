use image::{DynamicImage};
use libscreenshot::{get_full_capture_provider, get_area_capture_provider, get_screen_capture_provider, get_window_capture_provider, WindowCaptureProvider};

mod write_to_file;
use write_to_file::write_to_file;

fn main() {
    window_capture();
}

// FullCapture
fn full_capture() {
    match get_full_capture_provider() {
        Some(provider) => {
            let r = DynamicImage::from(provider.capture_full().unwrap());
            let output_ext = ("png".to_string()).to_lowercase();
            let output_format = image::ImageFormat::Png;
            write_to_file(output_ext, output_format, r);
        }
        _ => {
            println!("Capturing Full failed");
        }
    }
}

// ScreenCapture
fn current_screen_capture() {
    match get_screen_capture_provider() {
        Some(provider) => {
            let r = DynamicImage::from(provider.capture_current_screen().unwrap());
            let output_ext = ("png".to_string()).to_lowercase();
            let output_format = image::ImageFormat::Png;
            write_to_file(output_ext, output_format, r);
        }
        _ => {
            println!("Capturing Screen failed");
        }
    }
}

// AreaCapture
fn area_capture() {
    unimplemented!();
}

// WindowCapture for focused window
fn window_capture() {
    match get_window_capture_provider() {
        Some(provider) => {
            let r = DynamicImage::from(provider.capture_focused_window().unwrap());
            //let r = DynamicImage::from(provider.capture_window(insert_window_id_here).unwrap());
            let output_ext = ("png".to_string()).to_lowercase();
            let output_format = image::ImageFormat::Png;
            write_to_file(output_ext, output_format, r);
        }
        _ => {
            println!("Capturing Window failed");
        }
    }
}