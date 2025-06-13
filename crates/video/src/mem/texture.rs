use crate::gpu::Handle;
use bytemuck::{Pod, Zeroable};
use image::{DynamicImage, EncodableLayout, GenericImageView, RgbaImage};
pub use image_atlas::AtlasError;
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};
use math::size::Size2;
use math::vector::{vec2f, Vec2};
use std::cmp::max;
use std::path::Path;
use wgpu::{Extent3d, TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView};

#[derive(Debug)]
pub struct Texture {
    inner: wgpu::Texture,
    pub(crate) view: TextureView,
}

impl Texture {
    pub fn empty(handle: &Handle, size: impl Into<Size2<u32>>, format: TextureFormat) -> Self {
        let size = size.into();
        let inner = handle.device()
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: size.width,
                    height: size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[],
            });
        let view = inner.create_view(&Default::default());

        Self { inner, view }
    }

    pub fn depth(handle: &Handle, size: impl Into<Size2<u32>>) -> Self {
        Self::empty(handle, size, TextureFormat::Depth32Float)
    }
    
    pub fn from_data(handle: &Handle, size: impl Into<Size2<u32>>, data: impl AsRef<[u8]>) -> Self {
        let size = size.into();
        let texture = Self::empty(handle, size, TextureFormat::Rgba8UnormSrgb);
        
        handle.queue()
            .write_texture(
                texture.inner.as_image_copy(),
                data.as_ref(),
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(size.width * 4),
                    rows_per_image: Some(size.height),
                },
                Extent3d {
                    width: size.width,
                    height: size.height,
                    depth_or_array_layers: 1,
                },
            );
        
        texture
    }
    
    pub fn from_path(handle: &Handle, path: impl AsRef<Path>) -> image::ImageResult<Self> { 
        Ok(Self::from_image(handle, &image::open(path)?.to_rgba8()))
    }
    
    pub fn from_image(handle: &Handle, image: &RgbaImage) -> Self {
        Self::from_data(handle, image.dimensions(), image.as_bytes())
    }
    
    pub fn atlas(handle: &Handle, images: Vec<DynamicImage>) -> Result<(Self, Vec<AtlasTextureCoord>), AtlasError> {
        let (max_width, max_height) = images.iter()
            .map(|image| image.dimensions())
            .fold((0, 0), |(w0, h0), (w1, h1)| (max(w0, w1), max(h0, h1)));
        let max_dimension = max(max_width, max_height);
        
        let entries = images.into_iter()
            .map(|image| AtlasEntry {
                texture: image.to_rgba8(),
                mip: AtlasEntryMipOption::Clamp,
            })
            .collect::<Vec<_>>();
        let mut atlas = image_atlas::create_atlas(&AtlasDescriptor {
            max_page_count: 1,
            size: calculate_atlas_size(entries.len() as u32, max_dimension),
            mip: AtlasMipOption::NoMip,
            entries: &entries,
        })?;
        let image = atlas.textures.remove(0).mip_maps.remove(0);
        
        let texture = Self::from_image(handle, &image);
        let handles = atlas.texcoords.iter()
            .map(|c| AtlasTextureCoord {
                translation: Vec2::new(c.min_x as f32 / c.size as f32, c.min_y as f32 / c.size as f32),
                scale: Vec2::new((c.max_x - c.min_x) as f32 / c.size as f32, (c.max_y - c.min_y) as f32 / c.size as f32),
            })
            .collect::<Vec<_>>();

        Ok((texture, handles))
    }
}

pub fn calculate_atlas_size(count: u32, dimension: u32) -> u32 {
    if count == 0 || dimension == 0 {
        return 0;
    }
    
    let textures_per_side = (count as f64).sqrt().ceil() as u32;
    (textures_per_side * dimension).next_power_of_two()
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct AtlasTextureCoord {
    pub translation: vec2f,
    pub scale: vec2f,
}

impl AtlasTextureCoord {
    pub const NONE: Self = Self {
        translation: Vec2::ZERO,
        scale: Vec2::ZERO,
    };
}
