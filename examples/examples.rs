use std::time;
use image::{DynamicImage, ImageFormat};
use libscreenshot::{get_full_capture_provider, get_area_capture_provider, get_screen_capture_provider, get_window_capture_provider, WindowCaptureProvider};

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

pub fn write_to_file(output_ext: String, output_format: ImageFormat, image: DynamicImage) {
    let path = {
        let now = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => 0,
        };
        format!("{}.{}", now, output_ext)
    };

    image.save_with_format(path, output_format).unwrap();
}
