use image::{self, ImageBuffer, Rgba};
use std::path::Path;

const DEF_IMAGE_SIZE: u32 = 256;

pub struct Assets {
    pub tex: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Assets {
    pub fn load() -> Assets {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(DEF_IMAGE_SIZE, DEF_IMAGE_SIZE);

        for pixel in img.enumerate_pixels_mut() {
            *pixel.2 = image::Rgba([255,255,255,255]);
        }
        let image = image::open(Path::new("resources/texture.png"));
            let tex: ImageBuffer<Rgba<u8>, Vec<u8>> = match image {
                Ok(image_result) => image_result.to_rgba8(),
                Err(_image_error) => img.clone()
            };
        Assets {
            tex,
        }
    }
}
