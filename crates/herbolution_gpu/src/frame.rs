use std::mem::transmute;

use math::color::Rgba;
use math::size::Size2;
pub use wgpu::RenderPass as Pass;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, StoreOp, SurfaceTexture, TextureView};

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
            .inner
            .get_current_texture()
            .expect("Failed to get current texture");
        let depth_view = surface.depth_texture.view().clone();

        let clear_color = options
            .clear_color
            .map(|Rgba { r, g, b, a }| Color { r, g, b, a });

        Self {
            handle,
            state: Some(State::create(handle, surface_texture, clear_color, depth_view, handle.is_msaa_enabled())),
        }
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
        if let Some(State { encoder, surface, pass, .. }) = self.state.take() {
            drop(pass);
            self.handle.queue().submit(Some(encoder.finish()));
            surface.present();
        };
    }
}

struct State {
    encoder: CommandEncoder,
    surface: SurfaceTexture,
    view: TextureView,
    resolve_target: Option<TextureView>,
    depth_view: TextureView,
    pass: Option<Pass<'static>>,
}

impl State {
    fn create(gpu: &Handle, surface: SurfaceTexture, clear_color: Option<Color>, depth_view: TextureView, msaa: bool) -> Self {
        let mut encoder = gpu
            .device()
            .create_command_encoder(&Default::default());
        let surface_view = surface.texture.create_view(&Default::default());

        let (view, resolve_target) = if msaa {
            let resolution = Size2::new(surface.texture.width(), surface.texture.height());
            let staging_texture = Texture::empty(gpu, resolution, surface.texture.format(), true)
                .view()
                .clone();

            (staging_texture, Some(surface_view))
        } else {
            (surface_view, None)
        };

        let pass = create_pass(&mut encoder, &view, resolve_target.as_ref(), clear_color, &depth_view, true).forget_lifetime();

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
