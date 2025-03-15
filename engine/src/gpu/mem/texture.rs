use std::ops::Deref;
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, BindingResource, BindingType, Extent3d, ShaderStages, TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension};
use math::size::Size2;
use crate::gpu::handle::Handle;
use crate::gpu::mem::bind_group::{AddBindEntries, BindEntry};

pub struct Texture {
    inner: wgpu::Texture,
    pub(crate) view: TextureView,
}

impl Texture {
    pub fn open(handle: &Handle, path: impl AsRef<Path>) -> image::ImageResult<Self> {
        Ok(Self::from_image(handle, image::open(path)?))
    }

    pub fn from_rgba_bytes(handle: &Handle, size: Size2<u32>, bytes: &[u8]) -> Self {
        let texture = Self::empty(handle, TextureFormat::Rgba8UnormSrgb, size);

        handle.queue.write_texture(
            texture.inner.as_image_copy(),
            bytes,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width * 4),
                rows_per_image: Some(size.height),
            },
            Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
        );

        texture
    }

    pub fn from_image(handle: &Handle, image: DynamicImage) -> Self {
        Self::from_rgba_bytes(
            handle,
            Size2::from(image.dimensions()),
            &image.to_rgba8(),
        )
    }

    pub fn empty(handle: &Handle, format: TextureFormat, size: Size2<u32>) -> Self {
        let size = Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 };
        let inner = handle.device
            .create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST,
                view_formats: &[],
            });
        let view = inner.create_view(&TextureViewDescriptor::default());

        Self { inner, view }
    }
}

impl AddBindEntries for Texture {
    fn add_entries<'a>(&'a self, entries: &mut Vec<BindEntry<'a>>) {
        let binding = entries.len() as u32;
        entries.push(BindEntry {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: BindingResource::TextureView(&self.view)
            },
        });
    }
}

pub struct DepthTexture(Texture);

impl DepthTexture {
    pub fn create(handle: &Handle, size: Size2<u32>) -> Self {
        Self(Texture::empty(handle, TextureFormat::Depth32Float, size))
    }

    pub fn set_size(&mut self, handle: &Handle, size: Size2<u32>) {
        self.0 = Texture::empty(handle, TextureFormat::Depth32Float, size);
    }
}

impl Deref for DepthTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Handle {
    pub fn create_depth_texture(&self, size: Size2<u32>) -> DepthTexture {
        DepthTexture::create(self, size)
    }

    pub fn read_texture(&self, path: impl AsRef<Path>) -> image::ImageResult<Texture> {
        Texture::open(self, path)
    }

    pub fn create_texture_from_bytes(&self, size: Size2<u32>, bytes: &[u8]) -> Texture {
        Texture::from_rgba_bytes(self, size, bytes)
    }

    pub fn create_texture_from_image(&self, image: DynamicImage) -> Texture {
        Texture::from_image(self, image)
    }
}
