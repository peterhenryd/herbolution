use crate::engine::gpu::Gpu;
use image::DynamicImage;
use std::path::Path;
use wgpu::{Extent3d, TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};

pub mod depth;

pub struct Texture(wgpu::Texture);

impl Texture {
    pub fn open(gpu: &Gpu, path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let name = path.as_ref().display().to_string();
        let image = image::open(path)?;
        Ok(Self::from_image(gpu, &name, image))
    }

    pub fn from_bytes(gpu: &Gpu, name: &str, width: u32, height: u32, bytes: &[u8]) -> Self {
        let size = Extent3d { width, height, depth_or_array_layers: 1 };
        let texture = gpu.device
            .create_texture(&TextureDescriptor {
                label: Some(&format!("herbolution_{}_texture", name)),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::RENDER_ATTACHMENT |
                    TextureUsages::TEXTURE_BINDING |
                    TextureUsages::COPY_DST,
                view_formats: &[],
            });

        gpu.queue.write_texture(
            texture.as_image_copy(),
            bytes,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            size,
        );

        Self(texture)
    }

    pub fn from_image(gpu: &Gpu, name: &str, image: DynamicImage) -> Self {
        Self::from_bytes(gpu, name, image.width(), image.height(), &image.to_rgba8().as_raw())
    }

    pub fn create_view(&self) -> TextureView {
        self.0.create_view(&TextureViewDescriptor::default())
    }
}

impl Gpu {
    pub fn open_texture(&self, path: impl AsRef<Path>) -> image::ImageResult<Texture> {
        Texture::open(&self, path)
    }
}