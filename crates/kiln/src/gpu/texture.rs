use std::cmp::max;

use bytemuck::cast_slice;
use image::Pixel;
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasError, AtlasMipOption, Texcoord};
use prims::aabb::Aabb2;
use prims::calculate_atlas_size;
use prims::color::ColorPod;
use prims::size::{Size2, size2u};
use prims::vector::{Vec2, vec2u};
use wgpu::util::DeviceExt;
use wgpu::wgt::TextureDataOrder;
pub use wgpu::{Sampler, TextureDimension, TextureFormat, TextureUsages, TextureView};

use crate::Gpu;

#[derive(Debug, PartialEq)]
pub struct Texture {
    inner: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}

impl Texture {
    pub fn size(&self) -> size2u {
        let extent = self.inner.size();
        Size2::new(extent.width, extent.height)
    }

    pub fn depth_layers(&self) -> u32 {
        self.inner.size().depth_or_array_layers
    }

    pub fn write<C: ColorPod>(&self, gpu: &Gpu, position: vec2u, size: size2u, data: &[C]) {
        gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.inner,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: position.x,
                    y: position.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            cast_slice(data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width),
                rows_per_image: Some(size.height),
            },
            wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
        );
    }
}

impl AsRef<TextureView> for Texture {
    fn as_ref(&self) -> &TextureView {
        &self.view
    }
}

impl Gpu {
    pub fn build_texture(&self, size: impl Into<size2u>) -> TextureBuilder<'_, '_> {
        TextureBuilder {
            gpu: self,
            size: size.into(),
            mip_level_count: 1,
            depth_layers: 1,
            format: TextureFormat::Rgba8UnormSrgb,
            sample_count: 1,
            usages: TextureUsages::empty(),
            dimension: TextureDimension::D2,
            data: None,
            fill: false,
            label: None,
            data_order: TextureDataOrder::LayerMajor,
        }
    }
}

pub struct TextureBuilder<'gpu, 'a> {
    gpu: &'gpu Gpu,
    size: size2u,
    mip_level_count: u32,
    depth_layers: u32,
    format: TextureFormat,
    sample_count: u32,
    usages: TextureUsages,
    dimension: TextureDimension,
    data: Option<&'a [u8]>,
    fill: bool,
    label: Option<&'a str>,
    data_order: TextureDataOrder,
}

impl<'a> TextureBuilder<'_, 'a> {
    pub fn depth_layers(mut self, depth_layers: u32) -> Self {
        self.depth_layers = depth_layers;
        self
    }

    pub fn pixel_format<P: Pixel>(self) -> Self {
        self.format(match P::CHANNEL_COUNT {
            1 => TextureFormat::R8Unorm,
            2 => TextureFormat::Rg8Unorm,
            4 => TextureFormat::Rgba8UnormSrgb,
            n => panic!("{n} is not a valid number of pixel channels"),
        })
    }

    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    pub fn dimension(mut self, dimension: TextureDimension) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn data<C: ColorPod>(mut self, data: &'a [C]) -> Self {
        self.data = Some(cast_slice(data));
        self
    }

    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl TextureBuilder<'_, '_> {
    pub fn format(mut self, format: TextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn sbgra8(self) -> Self {
        self.format(TextureFormat::Bgra8UnormSrgb)
    }

    pub fn srgba8(self) -> Self {
        self.format(TextureFormat::Rgba8UnormSrgb)
    }

    pub fn depth32(self) -> Self {
        self.format(TextureFormat::Depth32Float)
    }

    pub fn depth32_stencil8(self) -> Self {
        self.format(TextureFormat::Depth32FloatStencil8)
    }

    pub fn depth24_plus(self) -> Self {
        self.format(TextureFormat::Depth24Plus)
    }

    pub fn depth24_plus_stencil8(self) -> Self {
        self.format(TextureFormat::Depth24Plus)
    }

    pub fn rgba8(self) -> Self {
        self.format(TextureFormat::Rgba8Unorm)
    }

    pub fn rg8(self) -> Self {
        self.format(TextureFormat::Rg8Unorm)
    }

    pub fn r8(self) -> Self {
        self.format(TextureFormat::R8Unorm)
    }
}

impl TextureBuilder<'_, '_> {
    pub fn usage(mut self, usage: TextureUsages) -> Self {
        self.usages.insert(usage);
        self
    }

    pub fn use_copy_src(self) -> Self {
        self.usage(TextureUsages::COPY_SRC)
    }

    pub fn use_copy_dst(self) -> Self {
        self.usage(TextureUsages::COPY_DST)
    }

    pub fn use_texture_binding(self) -> Self {
        self.usage(TextureUsages::TEXTURE_BINDING)
    }

    pub fn use_storage_binding(self) -> Self {
        self.usage(TextureUsages::STORAGE_BINDING)
    }

    pub fn use_render_attachment(self) -> Self {
        self.usage(TextureUsages::RENDER_ATTACHMENT)
    }

    pub fn use_storage_atomic(self) -> Self {
        self.usage(TextureUsages::STORAGE_ATOMIC)
    }
}

impl TextureBuilder<'_, '_> {
    pub fn create(self) -> Texture {
        let descriptor = wgpu::TextureDescriptor {
            label: self.label,
            size: wgpu::Extent3d {
                width: self.size.width,
                height: self.size.height,
                depth_or_array_layers: self.depth_layers,
            },
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        };
        let inner;
        if let Some(mut data) = self.data
            && !data.is_empty()
        {
            let mut backing_data = None;
            if self.fill {
                let texel_size;
                match self.format.block_copy_size(None) {
                    None => panic!("{:?} cannot be as a format with TextureBuilder", self.format),
                    Some(x) => texel_size = x as usize,
                }

                if data.len() != texel_size {
                    panic!(
                        "Filled textures must use a texel-size corresponding to their format: expected {} (for {:?}), found {}.",
                        texel_size,
                        self.format,
                        data.len()
                    );
                }

                backing_data = Some(vec![0; texel_size]);
                for i in 0..self.size.area() as usize {
                    let i_texel = i * texel_size;
                    backing_data.as_mut().unwrap()[i_texel..i_texel + texel_size].copy_from_slice(data);
                }

                data = backing_data.as_ref().unwrap().as_slice();
            }

            inner = self
                .gpu
                .device
                .create_texture_with_data(&self.gpu.queue, &descriptor, self.data_order, data);
        } else {
            inner = self.gpu.device.create_texture(&descriptor);
        }

        let view = inner.create_view(&Default::default());

        Texture { inner, view }
    }
}

pub struct TextureAtlasBuilder<I> {
    max_dimension: u32,
    images: Vec<I>,
}

impl<I> TextureAtlasBuilder<I> {}

impl Gpu {
    pub fn create_atlas_texture<I>(&self, images: Vec<I>) -> Result<(Texture, Vec<Aabb2<f32>>), AtlasError>
    where
        I: image::GenericImage,
        I::Pixel: Pixel<Subpixel = u8> + 'static,
    {
        let (max_width, max_height) = images
            .iter()
            .map(|image| image.dimensions())
            .fold((0, 0), |(w0, h0), (w1, h1)| (max(w0, w1), max(h0, h1)));
        let max_dimension = max(max_width, max_height);

        let entries = images
            .into_iter()
            .map(|texture| AtlasEntry {
                texture,
                mip: AtlasEntryMipOption::Clamp,
            })
            .collect::<Vec<_>>();
        let atlas = image_atlas::create_atlas(&AtlasDescriptor {
            max_page_count: 1,
            size: calculate_atlas_size(entries.len() as u32, max_dimension),
            mip: AtlasMipOption::NoMip,
            entries: &entries,
        })?;

        let texture = self
            .build_texture(atlas.size)
            .pixel_format::<I::Pixel>()
            .use_texture_binding()
            .use_copy_dst()
            .create();
        let handles = atlas
            .texcoords
            .iter()
            .map(
                |&Texcoord {
                     min_x,
                     min_y,
                     max_x,
                     max_y,
                     size,
                     ..
                 }| Aabb2 {
                    min: Vec2::new(min_x, min_y).cast() / size as f32,
                    max: Vec2::new(max_x, max_y).cast() / size as f32,
                },
            )
            .collect::<Vec<_>>();

        Ok((texture, handles))
    }
}

pub struct SamplerBuilder<'gpu> {
    gpu: &'gpu Gpu,
}

impl SamplerBuilder<'_> {
    pub fn create(self) -> Sampler {
        self.gpu
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: Default::default(),
                address_mode_v: Default::default(),
                address_mode_w: Default::default(),
                mag_filter: Default::default(),
                min_filter: Default::default(),
                mipmap_filter: Default::default(),
                lod_min_clamp: 0.0,
                lod_max_clamp: 0.0,
                compare: None,
                anisotropy_clamp: 0,
                border_color: None,
            })
    }
}

impl Gpu {
    pub fn build_sampler(&self) -> SamplerBuilder<'_> {
        SamplerBuilder { gpu: self }
    }
}
