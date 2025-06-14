use std::borrow::Cow;
use std::mem::{transmute, ManuallyDrop};
use wgpu::{Color, CommandEncoder, Device, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment, StoreOp, SurfaceTexture, TextureView};
use crate::{Handle, Surface};

pub use wgpu::RenderPass as Pass;
use math::color::Rgba;

/// A specialized command encoder for rendering to a surface with automatic queue submission and texture presentation.
pub struct Frame<'q> {
    queue: Cow<'q, Queue>,
    state: Option<State>,
}

pub struct Options {
    pub clear_color: Option<Rgba<f64>>,
    pub depth: bool,
}

impl<'q> Frame<'q> {
    pub fn new(handle: &'q Handle, surface: &Surface, options: Options) -> Self {
        let surface_texture = surface.inner
            .get_current_texture()
            .expect("Failed to get current texture");
        let depth_view = options.depth.then_some(&surface.depth_texture.view);
        
        let clear_color = options.clear_color
            .map(|Rgba { r, g, b, a }| Color { r, g, b, a });
        
        Self {
            queue: Cow::Borrowed(handle.queue()),
            state: Some(State::create(handle.device(), surface_texture, clear_color, depth_view)),
        }
    }
    
    pub fn pass(&mut self) -> &mut Pass<'_> {
        unsafe {
            // Narrows the static lifetime of the RenderPass to the lifetime of the Frame.
            transmute(&mut self.state.as_mut().unwrap().pass)
        }
    }

    pub fn into_owned(self) -> Frame<'static> {
        let mut frame = ManuallyDrop::new(self);
        
        Frame {
            queue: Cow::Owned(frame.queue.clone().into_owned()),
            state: frame.state.take(),
        }
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };

        drop(state.pass);

        let command_buffer = state.encoder.finish();
        self.queue.submit(Some(command_buffer));
        
        state.surface.present();
    }
}

struct State {
    encoder: CommandEncoder,
    surface: SurfaceTexture,
    pass: Pass<'static>,
}

impl State {
    fn create(device: &Device, surface: SurfaceTexture, clear_color: Option<Color>, depth_view: Option<&TextureView>) -> Self {
        let mut encoder = device.create_command_encoder(&Default::default());
        let surface_view = surface.texture.create_view(&Default::default());
        let pass = create_pass(&mut encoder, &surface_view, clear_color, depth_view).forget_lifetime();
        
        Self {
            encoder,
            surface,
            pass,
        }
    }
}

fn create_pass<'e>(
    encoder: &'e mut CommandEncoder,
    surface_view: &TextureView,
    clear_color: Option<Color>,
    depth_view: Option<&TextureView>,
) -> Pass<'e> {
    let depth_stencil_attachment = depth_view
        .map(|view| RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        });
    
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(RenderPassColorAttachment {
            view: surface_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color.unwrap_or_default()),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment,
        occlusion_query_set: None,
        timestamp_writes: None,
    })
}