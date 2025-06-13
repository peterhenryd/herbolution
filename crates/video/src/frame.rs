use crate::mem::buffer::Buffer;
use crate::mem::mesh::MeshId;
use crate::r2d::vertex::Instance2dPayload;
use crate::r2d::{RenderType2d, Renderer2d};
use crate::r3d::{Instance3dPayload, RenderType3d, Renderer3d};
use crate::Video;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture, TextureView};
use math::color::Rgba;
use crate::gpu::Handle;
use crate::mem::instance::InstanceSetId;
use crate::text::{TextFrame, TextRenderer};

pub struct Frame<'v, 't> {
    handle: &'v Handle,
    r2d: &'v Renderer2d,
    r3d: &'v Renderer3d,
    text_renderer: &'v mut TextRenderer,
    surface_texture: SurfaceTexture,
    command_encoder: CommandEncoder,
    render_pass: RenderPass<'static>,
    text_frame: Option<&'t TextFrame>,
}

impl<'v, 't> Frame<'v, 't> {
    pub fn create(video: &'v mut Video<'_>) -> Self {
        let surface_texture = video.surface.inner
            .get_current_texture()
            .expect("Failed to get current texture");
        let surface_view = surface_texture.texture.create_view(&Default::default());
        let mut command_encoder = video.handle.device().create_command_encoder(&Default::default());
        let depth_view = &video.surface.depth_texture.view;
        let Rgba { r, g, b, a } = video.clear_color;
        let clear_color = Some(Color { r, g, b, a });
        let render_pass = create_render_pass(&mut command_encoder, &surface_view, clear_color, depth_view);
        
        Self {
            handle: &video.handle,
            r2d: &video.r2d,
            r3d: &video.r3d,
            text_renderer: &mut video.text,
            surface_texture,
            command_encoder,
            render_pass,
            text_frame: None,
        }
    }
    
    pub fn draw_2d<'f>(&'f mut self, render_type: RenderType2d) -> Frame2d<'f, 'v, 't> { 
        self.r2d.pipelines.load_into_render_pass(render_type, &mut self.render_pass);
        
        Frame2d { frame: self, mesh_index_count: None }
    }
    
    pub fn draw_3d<'f>(&'f mut self, render_type: RenderType3d) -> Frame3d<'f, 'v, 't> {
        self.r3d.pipelines.load_into_render_pass(render_type, &mut self.render_pass);
        
        Frame3d { frame: self, mesh_index_count: None }
    }
    
    pub fn draw_text(&mut self, frame: &'t TextFrame) {
        self.text_frame = Some(frame);
    }
    
    pub fn submit(mut self) {
        if let Some(frame) = self.text_frame {
            self.text_renderer.prepare(self.handle, frame);
        }
        
        self.text_renderer.render(&mut self.render_pass);
        self.text_renderer.cleanup();
        
        drop(self.render_pass);
        
        let command_buffer = self.command_encoder.finish();
        self.handle.queue().submit(Some(command_buffer));
        
        self.surface_texture.present();
    }
}

fn create_render_pass(
    encoder: &mut CommandEncoder,
    surface_view: &TextureView,
    clear_color: Option<Color>,
    depth_view: &TextureView,
) -> RenderPass<'static> {
    encoder
        .begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &surface_view,
                resolve_target: None,
                ops: Operations {
                    load: clear_color.map(LoadOp::Clear).unwrap_or(LoadOp::Load),
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        })
        .forget_lifetime()
}

pub struct Frame2d<'f, 'v, 't> {
    frame: &'f mut Frame<'v, 't>,
    mesh_index_count: Option<u32>,
}

impl Frame2d<'_, '_, '_> {
    pub fn load_mesh(&mut self, id: MeshId) { 
        let mesh = self.frame.r2d.meshes.get(id);
        self.mesh_index_count = Some(mesh.load_into_render_pass(&mut self.frame.render_pass));
    }
    
    pub fn draw(&mut self, buffer: &Buffer<Instance2dPayload>) {
        let Some(index_count) = self.mesh_index_count else {
            panic!("2D mesh must be loaded before drawing");
        };

        let buffer = buffer.as_ref();
        if buffer.len() == 0 {
            return;
        }
        
        self.frame.render_pass.set_vertex_buffer(1, buffer.inner.slice(..));
        self.frame.render_pass.draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
    }
    
    pub fn draw_from_set(&mut self, id: InstanceSetId) {
        self.draw(self.frame.r2d.instance_sets.get(id));
    }
}

pub struct Frame3d<'f, 'v, 't> {
    frame: &'f mut Frame<'v, 't>,
    mesh_index_count: Option<u32>,
}

impl Frame3d<'_, '_, '_> {
    pub fn load_mesh(&mut self, id: MeshId) { 
        let mesh = self.frame.r3d.meshes.get(id);
        self.mesh_index_count = Some(mesh.load_into_render_pass(&mut self.frame.render_pass));
    }
    
    pub fn draw(&mut self, buffer: impl AsRef<Buffer<Instance3dPayload>>) {
        let Some(index_count) = self.mesh_index_count else {
            panic!("3D mesh must be loaded before drawing");
        };
        
        let buffer = buffer.as_ref();
        if buffer.len() == 0 {
            return;
        }
        
        self.frame.render_pass.set_vertex_buffer(1, buffer.inner.slice(..));
        self.frame.render_pass.draw_indexed(0..index_count, 0, 0..buffer.len() as u32);
    }
    
    pub fn draw_from_set(&mut self, id: InstanceSetId) {
        self.draw(self.frame.r3d.instance_sets.get(id));
    }
}