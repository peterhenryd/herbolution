use crate::engine::gpu::Gpu;
use wgpu::{Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};
use winit::dpi::PhysicalSize;

pub struct DepthTexture(Device, (Texture, TextureView));

impl DepthTexture {
    pub fn create(gpu: &Gpu, size: PhysicalSize<u32>) -> Self {
        Self(gpu.device.clone(), create_depth_texture(&gpu.device, size))
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let Self(device, x) = self;
        *x = create_depth_texture(device, size);
    }
}

impl AsRef<TextureView> for DepthTexture {
    fn as_ref(&self) -> &TextureView {
        &self.1.1
    }
}

fn create_depth_texture(device: &Device, PhysicalSize { width, height }: PhysicalSize<u32>) -> (Texture, TextureView) {
    let texture = device.create_texture(&TextureDescriptor {
        size: Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        label: Some("herbolution_depth_texture"),
        view_formats: &[],
    });
    let view = texture.create_view(&TextureViewDescriptor::default());
    (texture, view)
}