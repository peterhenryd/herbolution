use std::collections::HashMap;

use fontdue::Metrics;
use image::{ImageBuffer, RgbaImage};

use crate::video::gpu;
use crate::video::resource::{AtlasTextureCoord, Texture};
use crate::video::ui::font::{FontId, Fonts};

#[derive(Debug)]
pub struct Atlas {
    pub(crate) texture: Texture,
    pub font_coords: HashMap<AtlasFontId, FontCoord>,
}

impl Atlas {
    pub fn create(gpu: &gpu::Handle, fonts: &Fonts) -> Self {
        let mut images = Vec::new();

        let mut font_ids = vec![];
        for (font_id, msf) in fonts.iter() {
            for &font_size in msf.sizes() {
                for &c in msf.font().chars().keys() {
                    if !fonts.has_char(c) {
                        continue;
                    }

                    let (metrics, bitmap) = msf.font().rasterize(c, font_size);
                    let mut bytes = Vec::with_capacity(bitmap.len() * 4);
                    for b in bitmap.into_iter() {
                        bytes.extend([255; 3]);
                        bytes.push(b);
                    }

                    let image: RgbaImage = ImageBuffer::from_raw(metrics.width as u32, metrics.height as u32, bytes).unwrap();
                    images.push(image.into());

                    font_ids.push((AtlasFontId::new(font_id, c, font_size), metrics));
                }
            }
        }

        let (texture, coords) = Texture::atlas(gpu, images).unwrap();
        let mut coords = coords.into_iter();

        let mut font_coords = HashMap::new();
        for (font_id, metrics) in font_ids {
            let texture = coords.next().unwrap();
            font_coords.insert(font_id, FontCoord { texture, metrics });
        }

        Self { texture, font_coords }
    }

    pub fn glyph_coord(&self, font_id: FontId, char: char, font_size: f32) -> Option<&FontCoord> {
        self.font_coords
            .get(&AtlasFontId::new(font_id, char, font_size))
    }
}

#[derive(Debug)]
pub struct FontCoord {
    pub texture: AtlasTextureCoord,
    pub metrics: Metrics,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AtlasFontId {
    pub font_id: FontId,
    char: char,
    size: u32,
}

impl AtlasFontId {
    pub fn new(font_id: FontId, char: char, size: f32) -> Self {
        Self {
            font_id,
            char,
            size: size.to_bits(),
        }
    }
}
