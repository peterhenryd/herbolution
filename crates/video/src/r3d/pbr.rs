use std::path::{Path, PathBuf};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use crate::Handle;
use crate::mem::texture::{AtlasTextureCoord, Texture};

#[derive(Debug)]
pub struct PbrTextures {
    pub(super) albedo: Texture,
    pub(super) normal: Texture,
    pub(super) specular: Texture,
    pub(crate) coords: Vec<AtlasTextureCoord>
}

impl PbrTextures {
    pub fn create(handle: &Handle, texture_paths: &PbrTexturePaths) -> Self {
        let mut diffuse_images = Vec::with_capacity(texture_paths.vec.len());
        let mut normal_images = Vec::with_capacity(texture_paths.vec.len());
        let mut specular_images = Vec::with_capacity(texture_paths.vec.len());
        
        for path in &texture_paths.vec {
            let diffuse_image = image::open(&path.albedo).unwrap();
            let dimensions = diffuse_image.dimensions();
            
            let normal_image = read_image_or_create_identity(path.normal.as_ref(), dimensions, Rgba([128, 128, 255, 255]));
            let specular_image = read_image_or_create_identity(path.specular.as_ref(), dimensions, Rgba([255, 255, 255, 255]));
            
            diffuse_images.push(diffuse_image);
            normal_images.push(normal_image);
            specular_images.push(specular_image);
        }
        
        let (albedo, coords) = Texture::atlas(handle, diffuse_images).unwrap();
        let (normal, _) = Texture::atlas(handle, normal_images).unwrap();
        let (specular, _) = Texture::atlas(handle, specular_images).unwrap();
        
        Self {
            albedo,
            normal,
            specular,
            coords,
        }
    }
}

fn read_image_or_create_identity(path: Option<&PathBuf>, dimensions: (u32, u32), color: Rgba<u8>) -> DynamicImage {
    if let Some(path) = path {
        let image = image::open(path).unwrap();
        assert_eq!(image.dimensions(), dimensions, "Pbr image texture dimensions do not match diffuse texture dimensions");
        
        return image::open(path).unwrap();
    }

    let mut image = RgbaImage::new(dimensions.0, dimensions.1);
    
    for pixel in image.pixels_mut() {
        *pixel = color;
    }
    
    image.into()
}

pub struct PbrTexturePaths {
    pub vec: Vec<PbrTexturePath>,
}

impl PbrTexturePaths {
    pub fn new_suffixed(base: impl AsRef<Path>, names: &[&str], normal_suffix: &str, specular_suffix: &str) -> Self {
        let base = base.as_ref();
        Self {
            vec: names.iter()
                .map(|name| { 
                    PbrTexturePath {
                        albedo: base.join(format!("{}.png", name)),
                        normal: {
                            let path = base.join(format!("{}_{}.png", name, normal_suffix));
                            path.exists().then_some(path)
                        },
                        specular: {
                            let path = base.join(format!("{}_{}.png", name, specular_suffix));
                            path.exists().then_some(path)
                        }, 
                    } 
                })
                .collect(),
        }
    }
}

pub struct PbrTexturePath {
    pub albedo: PathBuf,
    pub normal: Option<PathBuf>,
    pub specular: Option<PathBuf>,
}