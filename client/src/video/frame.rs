use std::mem::transmute;

use lib::color::Rgba;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, StoreOp, TextureView};

use crate::video::gpu;
use crate::video::gpu::SurfaceTexture;
use crate::video::resource::{SampleCount, Texture};

pub struct Frame<'h> {
    pub(crate) handle: &'h gpu::Handle,
    state: Option<State>,
}

impl<'h> Frame<'h> {
    pub fn new(handle: &'h gpu::Handle, surface: &gpu::Surface, clear_color: Option<Rgba<f64>>) -> Self {
        let surface_texture = surface
            .create_texture()
            .expect("Failed to get current texture");
        let depth_view = surface.depth_texture().view().clone();
        let clear_color = clear_color.map(|Rgba { r, g, b, a }| Color { r, g, b, a });

        let state = State::create(handle, surface_texture, clear_color, depth_view, surface.sample_count());

        Self { handle, state: Some(state) }
    }

    pub fn pass(&mut self) -> &mut RenderPass<'_> {
        unsafe { transmute(&mut self.state.as_mut().unwrap().pass) }
    }

    pub fn advance_pass(&mut self, clear_color: Option<Rgba<f64>>, no_depth: bool) {
        let state = self.state.as_mut().unwrap();

        drop(state.pass.take());

        let clear_color = clear_color.map(|Rgba { r, g, b, a }| Color { r, g, b, a });
        state.pass = Some(
            begin_pass(
                &mut state.encoder,
                &state.view,
                state.resolve_target.as_ref(),
                clear_color,
                &state.depth_view,
                !no_depth,
            )
            .forget_lifetime(),
        );
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        let Some(State { encoder, pass, surface, .. }) = self.state.take() else {
            return;
        };

        drop(pass);
        self.handle.queue().submit(Some(encoder.finish()));
        drop(surface);
    }
}

struct State {
    surface: SurfaceTexture,
    encoder: CommandEncoder,
    view: TextureView,
    resolve_target: Option<TextureView>,
    depth_view: TextureView,
    pass: Option<RenderPass<'static>>,
}

impl State {
    fn create(gpu: &gpu::Handle, surface: SurfaceTexture, clear_color: Option<Color>, depth_view: TextureView, sample_count: SampleCount) -> Self {
        let mut encoder = gpu
            .device()
            .create_command_encoder(&Default::default());

        let (view, resolve_target) = if sample_count.is_multi() {
            let staging_texture = Texture::empty(gpu, surface.resolution(), surface.format(), sample_count)
                .view()
                .clone();

            (staging_texture, Some(surface.as_ref().clone()))
        } else {
            (surface.as_ref().clone(), None)
        };

        let pass = begin_pass(&mut encoder, &view, resolve_target.as_ref(), clear_color, &depth_view, true).forget_lifetime();

        Self {
            encoder,
            surface,
            view,
            resolve_target,
            depth_view,
            pass: Some(pass),
        }
    }
}

fn begin_pass<'e>(
    encoder: &'e mut CommandEncoder,
    view: &TextureView,
    resolve_target: Option<&TextureView>,
    clear_color: Option<Color>,
    depth_view: &TextureView,
    depth: bool,
) -> RenderPass<'e> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target,
            ops: Operations {
                load: clear_color
                    .map(LoadOp::Clear)
                    .unwrap_or(LoadOp::Load),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: depth_view,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: depth
                    .then_some(StoreOp::Store)
                    .unwrap_or(StoreOp::Discard),
            }),
            stencil_ops: None,
        }),
        occlusion_query_set: None,
        timestamp_writes: None,
    })
}
