use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture, TextureView};
pub use wgpu::RenderPass;
use math::color::Rgba;
use crate::gpu::{Gpu, GpuError};
use crate::gpu::handle::Handle;

pub struct GpuFrame {
    encoder: CommandEncoder,
    surface_texture: SurfaceTexture,
    surface_texture_view: TextureView,
    depth_texture_view: TextureView,
}

impl GpuFrame {
    pub fn create(gpu: &Gpu) -> Result<Self, GpuError> {
        let encoder = gpu.handle.device.create_command_encoder(&Default::default());
        let surface_texture = gpu.surface.texture()?;
        let surface_texture_view = surface_texture.texture.create_view(&Default::default());
        let depth_texture_view = gpu.depth_texture.view.clone();

        Ok(Self {
            encoder,
            surface_texture,
            surface_texture_view,
            depth_texture_view,
        })
    }

    pub fn finish(self, handle: &Handle) {
        handle.queue.submit(Some(self.encoder.finish()));
        self.surface_texture.present();
    }

    pub fn start_pass(&mut self, clear_color: Option<Rgba<f64>>) -> RenderPass {
        self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.surface_texture_view,
                resolve_target: None,
                ops: Operations {
                    load: clear_color
                        .map(|Rgba { r, g, b, a }| LoadOp::Clear(Color { r, g, b, a }))
                        .unwrap_or(LoadOp::Load),
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }
}