use crate::camera::{Camera, CameraPayload};
use crate::gpu::Handle;
use crate::mem::instance::InstanceSets;
use crate::load_shader;
use crate::mem::bind_group::BindGroup;
use crate::mem::buffer::{Buffer, GrowBuffer};
use crate::mem::mesh::{Mesh, Meshes};
use crate::pipeline::{PipelineOptions, Pipelines, RenderType};
use math::proj::Perspective;
use wgpu::{AddressMode, BufferUsages, Face, FilterMode, SamplerDescriptor, ShaderModule, ShaderStages};

mod vertex; 
mod world;
pub mod pbr;

use crate::mem::texture::AtlasTextureCoord;
use crate::r3d::pbr::{PbrTexturePaths, PbrTextures};
pub use vertex::{Instance3d, Instance3dPayload, Vertex3d};
pub use world::{World, WorldPayload};

pub type Mesh3d = Mesh<Vertex3d, u16>;
pub type Meshes3d = Meshes<Vertex3d, u16>;

pub type GrowBuffer3d = GrowBuffer<Instance3dPayload>;
pub type Buffer3d = Buffer<Instance3dPayload>;
pub type Sets3d = InstanceSets<Instance3dPayload>;

#[derive(Debug)]
pub struct Renderer3d {
    handle: Handle,
    pub(crate) pipelines: Pipelines<RenderType3d, 3>,
    camera_buffer: Buffer<CameraPayload>,
    world_buffer: Buffer<WorldPayload>,
    pub(crate) meshes: Meshes3d,
    pub(crate) instance_sets: Sets3d,
    pbr_textures: PbrTextures,
}

pub struct Options { 
    pub pbr_texture_paths: PbrTexturePaths,
}

impl Renderer3d {
    pub fn create(handle: &Handle, options: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(handle, 1, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let world_buffer = Buffer::with_capacity(handle, 1, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        
        let pbr_textures = PbrTextures::create(handle, &options.pbr_texture_paths);
        
        Self {
            handle: Handle::clone(handle),
            pipelines: Pipelines::create(handle, &RenderType3dOptions {
                camera_buffer: &camera_buffer,
                world_buffer: &world_buffer,
                shader_module: &load_shader!(handle, "shader.wgsl"),
                pbr_textures: &pbr_textures,
            }),
            camera_buffer,
            world_buffer,
            pbr_textures,
            meshes: Meshes::new(handle),
            instance_sets: InstanceSets::new(handle),
        }
    }
    
    pub fn update_camera(&mut self, camera: &Camera<Perspective>) { 
        self.camera_buffer.write(&self.handle, 0, &[camera.payload()]).expect("Failed to update 3D camera buffer");
    }
    
    pub fn update_world(&mut self, world: &World) { 
        self.world_buffer.write(&self.handle, 0, &[world.payload()]).expect("Failed to update world buffer");
    }
    
    pub fn meshes(&mut self) -> &mut Meshes3d {
        &mut self.meshes
    }

    pub fn sets(&mut self) -> &mut Sets3d {
        &mut self.instance_sets
    }
    
    pub fn texture_coords(&self) -> &[AtlasTextureCoord] {
        &self.pbr_textures.coords
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] 
pub enum RenderType3d {
    Terrain,
    Sky,
}

#[derive(Debug)]
pub struct RenderType3dOptions<'a> {
    camera_buffer: &'a Buffer<CameraPayload>,
    world_buffer: &'a Buffer<WorldPayload>,
    shader_module: &'a ShaderModule,
    pbr_textures: &'a PbrTextures,
}

impl RenderType<3> for RenderType3d {
    type Options<'a> = RenderType3dOptions<'a>;
    const ENTRIES: &'static [Self] = &[
        Self::Terrain,
        Self::Sky,
    ];

    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; 3] {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, ShaderStages::VERTEX_FRAGMENT)
            .finish(handle);
        
        let world_bind_group = BindGroup::build()
            .with_buffer(options.world_buffer, ShaderStages::VERTEX_FRAGMENT)
            .finish(handle);
        
        let sampler = handle.device()
            .create_sampler(&SamplerDescriptor {
                label: None,
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Linear,
                ..Default::default()
            });
        let texture_bind_group = BindGroup::build()
            .with_sampler(&sampler)
            .with_texture(&options.pbr_textures.albedo)
            .with_texture(&options.pbr_textures.normal)
            .with_texture(&options.pbr_textures.specular)
            .finish(handle);
        
        [
            camera_bind_group,
            world_bind_group,
            texture_bind_group,
        ]
    }

    fn pipeline_options<'a>(&self, _: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[
                Vertex3d::LAYOUT,
                Instance3d::LAYOUT,
            ],
            cull_mode: Some(match self {
                RenderType3d::Terrain => Face::Front,
                RenderType3d::Sky => Face::Back,
            }),
            depth_write_enabled: matches!(self, RenderType3d::Terrain),
        }
    }
}