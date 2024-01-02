use image::{self, ImageBuffer, Rgba};
use std::path::Path;

pub struct Assets {
    pub tex: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Assets {
    pub fn load() -> Assets {
        Assets {
            tex: image::open(Path::new("resources/texture.png"))
                .unwrap()
                .to_rgba8(),
        }
    }
}
