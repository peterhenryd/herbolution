use crate::gpu::binding::{AutoPayload, Payload, UniformBuffer};
use crate::gpu::ext::{RenderPipelineExt, RenderPipelineOptions};
use crate::gpu::geometry::Primitive;
use crate::gpu::renderer::RenderType;
use crate::gpu::Gpu;
use crate::state2d::text::{TextFrame, TextId, TextRenderer, TextSection};
use crate::uniform::Camera;
use bytemuck::{Pod, Zeroable};
use lib::Modify;
use math::num::traits::ConstZero;
use math::proj::Orthographic;
use math::size::Size2;
use math::vector::{vec2f, Vec3};
use wgpu::{include_wgsl, vertex_attr_array, BindGroup, BindGroupLayout, BufferAddress, Face, RenderPass, RenderPipeline, TextureFormat, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub mod text;

pub struct State2d {
    //renderer: Renderer<RenderType2d>,
    camera: Modify<Camera<Orthographic>>,
    camera_uniform: UniformBuffer<Camera<Orthographic>>,
    text_renderer: TextRenderer,
    text_frame: Modify<TextFrame>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Vertex2D {
    pub pos: vec2f,
}

impl Vertex2D {
    pub const fn new(pos: vec2f) -> Self {
        Self { pos }
    }
}

impl Primitive for Vertex2D {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        0 => Float32x2
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex2D>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instance2d {
    // TODO
}

impl Primitive for Instance2d {
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: 0,
        step_mode: VertexStepMode::Instance,
        attributes: &[],
    };
    const ATTRIBUTES: &'static [VertexAttribute] = &[];
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Instance2dPayload {
}

impl AutoPayload for Vertex2D {}

impl Payload for Instance2d {
    type Output = Instance2dPayload;

    fn payload(&self) -> Self::Output {
        Instance2dPayload {}
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct RenderType2d;

impl RenderType for RenderType2d {
    type Vertex = Vertex2D;
    type Instance = Instance2d;

    fn create_render_pipeline(&self, gpu: &Gpu, bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline {
        let shader_module = gpu.device.create_shader_module(include_wgsl!("shader.wgsl"));
        RenderPipeline::create(gpu, RenderPipelineOptions {
            bind_group_layouts,
            shader_module: &shader_module,
            input: &[
                Vertex2D::LAYOUT,
                Instance2d::LAYOUT,
            ],
            cull_mode: Face::Back,
            depth_write_enabled: true,
            texture_format: TextureFormat::Bgra8UnormSrgb,
        })
    }

    fn set_bind_groups(&self, render_pass: &mut RenderPass, bind_groups: &[BindGroup]) {
        render_pass.set_bind_group(0, &bind_groups[0], &[]);
    }
}

impl State2d {
    pub fn create(gpu: &Gpu, size: Size2<u32>, format: TextureFormat) -> Self {
        let camera: Modify<_> = Camera::new(Vec3::ZERO, Orthographic::from(size)).into();
        let camera_uniform = UniformBuffer::create(gpu, &*camera);

        /*let renderer = Renderer::create(gpu, vec![
            BindGroup::build().append(&camera_uniform, ShaderStages::VERTEX_FRAGMENT).finish(gpu),
        ], &[RenderType2d]);
         */
        let text_renderer = TextRenderer::create(gpu, size, format);
        let text_frame = TextFrame::default().into();

        Self {
            //renderer,
            camera,
            camera_uniform,
            text_renderer,
            text_frame,
        }
    }

    pub fn set_size(&mut self, gpu: &Gpu, size: Size2<u32>) {
        self.text_renderer.set_size(gpu, size);
    }

    pub fn update(&mut self, gpu: &Gpu) {
        if let Some(value) = self.camera.take_modified() {
            self.camera_uniform.write(gpu, value);
        }

        if let Some(text_frame) = self.text_frame.take_modified() {
            self.text_renderer.prepare(gpu, text_frame);
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        // self.renderer.render_group(render_pass, ..);
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