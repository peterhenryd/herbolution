pub use chisel::Chisel;
use gpu::{
    BindGroup, Buffer, BufferUsage, Camera, CameraPayload, CompiledShaders, CullMode, Handle, PipelineMap, PipelineOptions, PipelineType, SampleCount,
    ShaderModule, ShaderSources, ShaderStage,
};
use math::proj::Perspective;
pub use vertex::{Instance3d, Instance3dPayload, Vertex3d};
pub use world::{World, WorldPayload};

pub mod chisel;
pub mod vertex;
pub mod world;

pub type Mesh = gpu::Mesh<Vertex3d, u16>;
pub type Meshes = gpu::Meshes<Vertex3d, u16>;

pub type GrowBuffer3d = gpu::GrowBuffer<Instance3dPayload>;
pub type Buffer3d = Buffer<Instance3dPayload>;
pub type Sets = gpu::Sets<Instance3dPayload>;

#[derive(Debug)]
pub struct Sculptor {
    handle: Handle,
    pub(crate) pipeline_map: PipelineMap<RenderType>,
    camera_buffer: Buffer<CameraPayload>,
    world_buffer: Buffer<WorldPayload>,
    shaders: CompiledShaders,
    pub(crate) meshes: Meshes,
    pub(crate) sets: Sets,
}

pub struct Options {}

impl Sculptor {
    pub fn create(gpu: &Handle, sample_count: SampleCount) -> Self {
        let camera_buffer = Buffer::with_capacity(gpu, 1, BufferUsage::UNIFORM | BufferUsage::COPY_DST);
        let world_buffer = Buffer::with_capacity(gpu, 1, BufferUsage::UNIFORM | BufferUsage::COPY_DST);
        let shaders = ShaderSources::default()
            .with("world", include_str!("shaders/world.wgsl"))
            .compile(gpu)
            .expect("Failed to compile shaders");

        Self {
            handle: Handle::clone(gpu),
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

    pub fn set_sample_count(&mut self, gpu: &Handle, sample_count: SampleCount) {
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

    pub fn update_camera(&mut self, camera: &Camera<Perspective>) {
        self.camera_buffer
            .write(&self.handle, 0, &[camera.payload()])
            .expect("Failed to update 3D camera buffer");
    }

    pub fn update_world(&mut self, world: &World) {
        self.world_buffer
            .write(&self.handle, 0, &[world.payload()])
            .expect("Failed to update world buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes {
        &mut self.meshes
    }

    pub fn sets(&mut self) -> &mut Sets {
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
    camera_buffer: &'a Buffer<CameraPayload>,
    world_buffer: &'a Buffer<WorldPayload>,
    shader_module: &'a ShaderModule,
}

impl PipelineType for RenderType {
    type Options<'a> = RenderType3dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self::Terrain, Self::Sky];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup> {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, ShaderStage::VERTEX_FRAGMENT)
            .finish(gpu);

        let world_bind_group = BindGroup::build()
            .with_buffer(options.world_buffer, ShaderStage::VERTEX_FRAGMENT)
            .finish(gpu);

        vec![camera_bind_group, world_bind_group]
    }

    fn pipeline_options<'a>(&self, _: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[Vertex3d::LAYOUT, Instance3d::LAYOUT],
            cull_mode: Some(match self {
                RenderType::Terrain => CullMode::Back,
                RenderType::Sky => CullMode::Front,
            }),
            depth_write_enabled: matches!(self, RenderType::Terrain),
        }
    }
}
