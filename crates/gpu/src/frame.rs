use std::mem::transmute;

use math::color::Rgba;
pub use wgpu::RenderPass as Pass;
use wgpu::{
    Color, CommandEncoder, Device, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, StoreOp, SurfaceTexture, TextureView,
};

use crate::{Handle, Surface};

/// A specialized command encoder for rendering to a surface with automatic queue submission and texture presentation.
pub struct Frame<'a> {
    pub handle: &'a Handle,
    state: Option<State>,
}

pub struct Options {
    pub clear_color: Option<Rgba<f64>>,
}

impl<'h> Frame<'h> {
    pub fn new(handle: &'h Handle, surface: &Surface, options: Options) -> Self {
        let surface_texture = surface
            .inner
            .get_current_texture()
            .expect("Failed to get current texture");
        let depth_view = surface.depth_texture.view().clone();

        let clear_color = options
            .clear_color
            .map(|Rgba { r, g, b, a }| Color { r, g, b, a });

        Self {
            handle,
            state: Some(State::create(handle.device(), surface_texture, clear_color, depth_view)),
        }
    }

    pub fn pass(&mut self) -> &mut Pass<'_> {
        unsafe {
            // Narrows the static lifetime of the RenderPass to the lifetime of the Frame.
            transmute(&mut self.state.as_mut().unwrap().pass)
        }
    }

    pub fn advance_pass(&mut self) {
        let state = self.state.as_mut().unwrap();

        drop(state.pass.take());
        state.pass = Some(create_pass(&mut state.encoder, &state.surface_view, None, &state.depth_view).forget_lifetime());
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };

        drop(state.pass);

        let command_buffer = state.encoder.finish();
        self.handle.queue().submit(Some(command_buffer));

        state.surface.present();
    }
}

struct State {
    encoder: CommandEncoder,
    surface: SurfaceTexture,
    surface_view: TextureView,
    depth_view: TextureView,
    pass: Option<Pass<'static>>,
}

impl State {
    fn create(device: &Device, surface: SurfaceTexture, clear_color: Option<Color>, depth_view: TextureView) -> Self {
        let mut encoder = device.create_command_encoder(&Default::default());
        let surface_view = surface
            .texture
            .create_view(&Default::default());
        let pass = create_pass(&mut encoder, &surface_view, clear_color, &depth_view).forget_lifetime();

        Self {
            encoder,
            surface,
            surface_view,
            depth_view,
            pass: Some(pass),
        }
    }
}

fn create_pass<'e>(encoder: &'e mut CommandEncoder, surface_view: &TextureView, clear_color: Option<Color>, depth_view: &TextureView) -> Pass<'e> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view: surface_view,
            resolve_target: None,
            ops: Operations {
                load: clear_color
                    .map(LoadOp::Clear)
                    .unwrap_or(LoadOp::Load),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: depth_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        occlusion_query_set: None,
        timestamp_writes: None,
    })
}
