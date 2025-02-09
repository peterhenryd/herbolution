use crate::engine::geometry;
use crate::engine::geometry::quad::Quad;
use crate::engine::gpu::Gpu;
use crate::engine::mesh::Mesh;
use crate::engine::pipeline::Pipeline;
use crate::engine::surface::Surface;
use crate::engine::texture::depth::DepthTexture;
use crate::engine::uniform::{AsByteStructUniformExt, Uniform};
use crate::listener::{InputEvent, Listener};
use crate::world::build_textures;
use crate::world::camera::proj::perspective::Perspective;
use crate::world::camera::Camera;
use crate::world::transform::{Rotation, Transform};
use math::vector::vec3;
use wgpu::{include_wgsl, CompareFunction, DepthBiasState, DepthStencilState, ShaderStages, StencilState, TextureFormat};
use winit::dpi::PhysicalSize;
use crate::world::lighting::Lighting;

pub struct Renderer {
    pub(crate) gpu: Gpu,
    pub pipeline: Pipeline,
    pub camera: Uniform<Camera<Perspective>>,
    pub lighting: Lighting,
    pub(crate) quad_mesh: Mesh,
    pub(crate) depth_texture: DepthTexture,
}

impl Renderer {
    pub fn create(gpu: Gpu, surface: &Surface) -> Self {
        let size = surface.get_size();
        let proj = Perspective::from(size);
        let camera = Camera::new(Transform::new(vec3::zero(), Rotation::default()), proj)
            .into_uniform(&gpu, "camera");
        let lighting = Lighting::create(&gpu);
        let pipeline = gpu
            .build_pipeline("world", surface.get_format())
            .with_shader(include_wgsl!("shader.wgsl"))
            .build_binding("world_camera", |b| b
                .with_uniform(ShaderStages::VERTEX_FRAGMENT, &camera)
                .finish(),
            )
            .build_binding("world_texture", |b| build_textures(&gpu, b))
            .with_binding(lighting.binding.clone())
            .with_buffers(geometry::get_vertex_instance_buffer_layouts())
            .with_depth_stencil(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            })
            .finish();
        let quad_mesh = Quad::create_mesh(&gpu);
        let depth_texture = DepthTexture::create(&gpu, size);

        Self { gpu, pipeline, camera, lighting, quad_mesh, depth_texture }
    }

    pub fn camera(&self) -> &Transform {
        &self.camera.transform
    }
}

impl Listener for Renderer {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.depth_texture.resize(size);
        self.camera.edit(|c| c.proj.resize(size));
    }

    fn on_input(&mut self, _: &InputEvent) {}
}