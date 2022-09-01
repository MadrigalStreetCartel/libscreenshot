use std::time;
use image::{DynamicImage, ImageFormat};


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