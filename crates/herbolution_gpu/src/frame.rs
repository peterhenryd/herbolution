use std::mem::transmute;

use math::color::Rgba;
pub use wgpu::RenderPass as Pass;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, StoreOp, TextureView};

use crate::surface::SurfaceTexture;
use crate::texture::SampleCount;
use crate::{Handle, Surface, Texture};

/// A specialized command encoder for rendering to a surface with automatic queue submission and texture presentation.
pub struct Frame<'h> {
    pub handle: &'h Handle,
    state: Option<State>,
}

pub struct Options {
    pub clear_color: Option<Rgba<f64>>,
}

impl<'h> Frame<'h> {
    pub fn new(handle: &'h Handle, surface: &Surface, options: Options) -> Self {
        let surface_texture = surface
            .create_texture()
            .expect("Failed to get current texture");
        let depth_view = surface.depth_texture().view().clone();

        let clear_color = options
            .clear_color
            .map(|Rgba { r, g, b, a }| Color { r, g, b, a });

        let state = State::create(handle, surface_texture, clear_color, depth_view, surface.sample_count());

        Self { handle, state: Some(state) }
    }

    pub fn pass(&mut self) -> &mut Pass<'_> {
        unsafe {
            // Narrows the static lifetime of the RenderPass to the lifetime of the Frame.
            transmute(&mut self.state.as_mut().unwrap().pass)
        }
    }

    pub fn advance_pass(&mut self, clear_color: Option<Rgba<f64>>, no_depth: bool) {
        let state = self.state.as_mut().unwrap();

        drop(state.pass.take());

        let clear_color = clear_color.map(|Rgba { r, g, b, a }| Color { r, g, b, a });
        state.pass = Some(
            create_pass(
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
        if let Some(State { encoder, pass, .. }) = self.state.take() {
            drop(pass);
            self.handle.queue().submit(Some(encoder.finish()));
        };
    }
}

struct State {
    // Don't drop until the encoder is submitted
    _surface: SurfaceTexture,
    encoder: CommandEncoder,
    view: TextureView,
    resolve_target: Option<TextureView>,
    depth_view: TextureView,
    pass: Option<Pass<'static>>,
}

impl State {
    fn create(gpu: &Handle, surface: SurfaceTexture, clear_color: Option<Color>, depth_view: TextureView, sample_count: SampleCount) -> Self {
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

        let pass = create_pass(&mut encoder, &view, resolve_target.as_ref(), clear_color, &depth_view, true).forget_lifetime();

        Self {
            encoder,
            _surface: surface,
            view,
            resolve_target,
            depth_view,
            pass: Some(pass),
        }
    }
}

fn create_pass<'e>(
    encoder: &'e mut CommandEncoder,
    view: &TextureView,
    resolve_target: Option<&TextureView>,
    clear_color: Option<Color>,
    depth_view: &TextureView,
    depth: bool,
) -> Pass<'e> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view,
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
