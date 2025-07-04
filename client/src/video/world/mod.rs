use lib::proj::Proj;
use lib::vector::vec3d;
pub use vertex::{Instance3d, Vertex3d};
use wgpu::{BufferUsages, Face, ShaderModule, ShaderStages};
pub use world::{World, WorldPayload};

use crate::video::camera::{VideoCamera, View};
use crate::video::gpu;
use crate::video::resource::{BindGroup, Buffer, CompiledShaders, Meshes, PipelineMap, PipelineOptions, PipelineType, SampleCount, Sets, ShaderSources};

pub mod chisel;
pub mod vertex;
pub mod world;

#[derive(Debug)]
pub struct Sculptor {
    gpu: gpu::Handle,
    pub(crate) pipeline_map: PipelineMap<RenderType>,
    camera_buffer: Buffer<VideoCamera>,
    world_buffer: Buffer<WorldPayload>,
    shaders: CompiledShaders,
    pub(crate) meshes: Meshes<Vertex3d>,
    pub(crate) sets: Sets<Instance3d>,
}

pub struct Options {}

impl Sculptor {
    pub fn create(gpu: &gpu::Handle, sample_count: SampleCount) -> Self {
        let camera_buffer = Buffer::create(gpu, 1, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let world_buffer = Buffer::create(gpu, 1, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let shaders = ShaderSources::default()
            .with("world", include_str!("shaders/world.wgsl"))
            .compile(gpu)
            .expect("Failed to compile shaders");

        Self {
            gpu: gpu.clone(),
            pipeline_map: PipelineMap::create(
                gpu,
                &RenderType3dOptions {
                    camera_buffer: &camera_buffer,
                    world_buffer: &world_buffer,
                    shader_module: shaders.get_module("world").unwrap(),
                },
                sample_count,
            ),
            camera_buffer,
            world_buffer,
            shaders,
            meshes: Meshes::new(gpu),
            sets: Sets::new(gpu),
        }
    }

    pub fn set_sample_count(&mut self, gpu: &gpu::Handle, sample_count: SampleCount) {
        self.pipeline_map.set_sample_count(
            gpu,
            sample_count,
            &RenderType3dOptions {
                camera_buffer: &self.camera_buffer,
                world_buffer: &self.world_buffer,
                shader_module: self.shaders.get_module("world").unwrap(),
            },
        );
    }

    pub fn update_camera(&mut self, position: vec3d, view: View, proj: impl Proj) {
        let camera = VideoCamera::new(position, view, proj);
        self.camera_buffer
            .write(&self.gpu, 0, &[camera])
            .expect("Failed to update 3D camera buffer");
    }

    pub fn update_world(&mut self, world: &World) {
        self.world_buffer
            .write(&self.gpu, 0, &[world.payload()])
            .expect("Failed to update world buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes<Vertex3d> {
        &mut self.meshes
    }

    pub fn sets(&mut self) -> &mut Sets<Instance3d> {
        &mut self.sets
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RenderType {
    Terrain,
    Sky,
}

#[derive(Debug)]
pub struct RenderType3dOptions<'a> {
    camera_buffer: &'a Buffer<VideoCamera>,
    world_buffer: &'a Buffer<WorldPayload>,
    shader_module: &'a ShaderModule,
}

impl PipelineType for RenderType {
    type Options<'a> = RenderType3dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self::Terrain, Self::Sky];

    fn create_bind_groups(gpu: &gpu::Handle, options: &Self::Options<'_>) -> Vec<BindGroup> {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, ShaderStages::VERTEX_FRAGMENT)
            .finish(gpu);

        let world_bind_group = BindGroup::build()
            .with_buffer(options.world_buffer, ShaderStages::VERTEX_FRAGMENT)
            .finish(gpu);

        vec![camera_bind_group, world_bind_group]
    }

    fn pipeline_options<'a>(&self, _: &gpu::Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[Vertex3d::LAYOUT, Instance3d::LAYOUT],
            cull_mode: Some(match self {
                RenderType::Terrain => Face::Back,
                RenderType::Sky => Face::Front,
            }),
            depth_write_enabled: matches!(self, RenderType::Terrain),
        }
    }
}
