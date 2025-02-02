use std::path::Path;
use wgpu::{
    AddressMode, CompareFunction, Device, Extent3d, FilterMode, ImageDataLayout, Sampler,
    SamplerDescriptor, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
};
use winit::dpi::PhysicalSize;
use crate::gpu::Gpu;

pub struct DepthTexture(wgpu::Texture, pub(crate) TextureView, Sampler);

impl DepthTexture {
    pub(crate) const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn create(
        device: &Device,
        PhysicalSize {
            width: w,
            height: h,
        }: PhysicalSize<u32>,
    ) -> DepthTexture {
        let size = Extent3d {
            width: w.max(1),
            height: h.max(1),
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            compare: Some(CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self(texture, view, sampler)
    }
}

pub struct Texture(wgpu::Texture);

impl Texture {
    pub fn open(gpu: &Gpu, path: impl AsRef<Path>) -> image::ImageResult<Self> {
        let image = image::open(path)?;
        Ok(Self::from_image(gpu, image))
    }

    pub fn from_image(gpu: &Gpu, image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = gpu.device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        gpu.queue.write_texture(
            texture.as_image_copy(),
            &image.to_rgba8(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        Self(texture)
    }

    pub fn create_rgba_2d_array_view(&self) -> TextureView {
        self.0.create_view(&TextureViewDescriptor {
            label: None,
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::D2Array),
            aspect: TextureAspect::All,
            ..Default::default()
        })
    }
}
