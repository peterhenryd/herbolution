use wgpu::{RenderPass, ShaderStages, TextureFormat};
use lib::Modify;
use math::projection::orthographic::Orthographic;
use math::size::Size2;
use math::transform::Transform;
use crate::camera::Camera;
use crate::gpu::handle::Handle;
use crate::gpu::mem::buffer::UnaryBuffer;
use crate::renderer_2d::pipeline::Pipeline2D;
use crate::renderer_2d::text::{TextFrame, TextId, TextRenderer, TextSection};

pub mod pipeline;
pub mod vertex;
pub mod text;

pub struct Renderer2D {
    camera: UnaryBuffer<Camera<Orthographic>>,
    pipeline: Pipeline2D,
    text_renderer: TextRenderer,
    text_frame: Modify<TextFrame>,
}

impl Renderer2D {
    pub fn create(handle: &Handle, size: Size2<u32>, format: TextureFormat) -> Self {
        let camera = Camera::new(Transform::default(), Orthographic::from(size));
        let camera = handle.create_unary_buffer(camera, ShaderStages::VERTEX_FRAGMENT);
        let pipeline = Pipeline2D::create(handle, &camera, format);
        let text_renderer = TextRenderer::create(handle, size, format);
        let text_frame = TextFrame::default().into();

        Self {
            camera,
            pipeline,
            text_renderer,
            text_frame,
        }
    }

    pub fn update(&mut self, handle: &Handle) {
        self.camera.submit(handle);

        if let Some(text_frame) = self.text_frame.take_modified() {
            self.text_renderer.prepare(handle, text_frame);
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.text_renderer.render(render_pass);
    }

    pub fn add_text(&mut self, section: TextSection) -> TextId {
        TextId(self.text_frame.sections.insert(section))
    }

    pub fn remove_text(&mut self, id: TextId) {
        self.text_frame.sections.remove(id.0);
    }

    pub fn cleanup(&mut self) {
        self.text_renderer.cleanup();
    }
}