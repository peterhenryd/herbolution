pub use font::*;

mod font;
mod mesh;

use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageError, Rgba, RgbaImage};
use image_atlas::AtlasError;
use thiserror::Error;

#[derive(Debug)]
pub struct Assets {
    dir_path: PathBuf,
}

impl Assets {
    pub fn new(dir_path: PathBuf) -> Self {
        Self { dir_path }
    }

    pub fn load_pbr_material(&self, name: impl AsRef<str>) -> Result<PbrMaterial, AssetError> {
        let open_optional_pbr_image = |path: &Path, name: &str, dimensions: (u32, u32), pixel: Rgba<u8>| match image::open(path.join(name)) {
            Ok(image) if image.dimensions() == dimensions => Ok(image.into_rgba8()),
            Ok(image) => Err(AssetError::MismatchedTextureSize {
                name: name.to_owned(),
                expected: dimensions,
                found: image.dimensions(),
            }),
            Err(ImageError::IoError(_)) => Ok(RgbaImage::from_pixel(dimensions.0, dimensions.1, pixel)),
            Err(e) => Err(AssetError::Image(e)),
        };

        let path = self
            .dir_path
            .join("pbrMaterial")
            .join(name.as_ref());
        let albedo_path = path.join("albedo.png");

        if !albedo_path.is_file() {
            return Err(AssetError::MissingAlbedo {
                name: name.as_ref().to_owned(),
            });
        }

        let albedo_image = image::open(&albedo_path)?;
        let dimensions = albedo_image.dimensions();

        Ok(PbrMaterial {
            albedo_image: albedo_image.into_rgba8(),
            emissive_image: open_optional_pbr_image(&path, "emissive.png", dimensions, Rgba([0, 0, 0, 255]))?,
            height_image: open_optional_pbr_image(&path, "height.png", dimensions, Rgba([0, 0, 0, 255]))?,
            normal_image: open_optional_pbr_image(&path, "normal.png", dimensions, Rgba([128, 128, 255, 255]))?,
            orm_image: open_optional_pbr_image(&path, "orm.png", dimensions, Rgba([255, 255, 0, 255]))?,
        })
    }

    pub fn load_image(&self, name: impl AsRef<str>) -> Result<RgbaImage, AssetError> {
        let path = self
            .dir_path
            .join(name.as_ref())
            .with_extension("png");
        Ok(image::open(path)?.into_rgba8())
    }
}

#[derive(Debug)]
pub struct PbrMaterial {
    pub albedo_image: RgbaImage,
    pub emissive_image: RgbaImage,
    pub height_image: RgbaImage,
    pub normal_image: RgbaImage,
    pub orm_image: RgbaImage,
}

#[derive(Debug, Error)]
pub enum AssetError {
    #[error(transparent)]
    Atlas(#[from] AtlasError),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Pbr material {name} is missing an albedo texture")]
    MissingAlbedo { name: String },
    #[error("Mismatched texture size")]
    MismatchedTextureSize { name: String, expected: (u32, u32), found: (u32, u32) },
}

pub struct Mesh<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u16>,
}
