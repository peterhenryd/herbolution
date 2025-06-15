use std::sync::Arc;

use gpu::buffer::Usage;
use gpu::camera::{Camera, CameraPayload};
use gpu::handle::Handle;
use gpu::pipeline::{Face, PipelineOptions};
use gpu::sampler::Filter;
use gpu::shader::Stage;
use gpu::texture::AtlasTextureCoord;
use gpu::{BindGroup, Buffer, load_shader, pipeline, sampler, shader};
use math::proj::Perspective;

use crate::pbr::{PbrTexturePaths, PbrTextures};

mod draw;
pub mod pbr;
mod vertex;
mod world;

pub type Vertex = vertex::Vertex3d;
pub type Instance = vertex::Instance3d;
pub type InstancePayload = vertex::Instance3dPayload;

pub type World = world::World;
pub type WorldPayload = world::WorldPayload;

pub type Mesh = gpu::Mesh<Vertex, u16>;
pub type Meshes = gpu::Meshes<Vertex, u16>;

pub type GrowBuffer3d = gpu::GrowBuffer<InstancePayload>;
pub type Buffer3d = Buffer<InstancePayload>;
pub type Sets = gpu::Sets<InstancePayload>;

pub type Drawing<'q, 'f, 'r> = draw::Draw<'q, 'f, 'r>;

#[derive(Debug)]
pub struct Renderer {
    handle: Handle,
    pub(crate) pipeline_map: pipeline::Map<RenderType, 3>,
    camera_buffer: Buffer<CameraPayload>,
    world_buffer: Buffer<WorldPayload>,
    pub(crate) meshes: Meshes,
    pub(crate) sets: Sets,
    pbr_textures: PbrTextures,
}

pub struct Options {
    pub pbr_texture_paths: PbrTexturePaths,
}

impl Renderer {
    pub fn create(handle: &Handle, options: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(handle, 1, Usage::UNIFORM | Usage::COPY_DST);
        let world_buffer = Buffer::with_capacity(handle, 1, Usage::UNIFORM | Usage::COPY_DST);

        let pbr_textures = PbrTextures::create(handle, &options.pbr_texture_paths);

        Self {
            handle: Handle::clone(handle),
            pipeline_map: pipeline::Map::create(
                handle,
                &RenderType3dOptions {
                    camera_buffer: &camera_buffer,
                    world_buffer: &world_buffer,
                    shader_module: &load_shader!(handle, "shaders/world.wgsl"),
                    pbr_textures: &pbr_textures,
                },
            ),
            camera_buffer,
            world_buffer,
            pbr_textures,
            meshes: Meshes::new(handle),
            sets: Sets::new(handle),
        }
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

    pub fn texture_coords(&self) -> &Arc<[AtlasTextureCoord]> {
        &self.pbr_textures.coords
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
    shader_module: &'a shader::Module,
    pbr_textures: &'a PbrTextures,
}

impl pipeline::Key<3> for RenderType {
    type Options<'a> = RenderType3dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self::Terrain, Self::Sky];

    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; 3] {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, Stage::VERTEX_FRAGMENT)
            .finish(handle);

        let world_bind_group = BindGroup::build()
            .with_buffer(options.world_buffer, Stage::VERTEX_FRAGMENT)
            .finish(handle);

        let sampler = handle.create_sampler(sampler::Options { filter: Filter::Pixelated });
        let texture_bind_group = BindGroup::build()
            .with_sampler(&sampler)
            .with_texture(&options.pbr_textures.albedo)
            .with_texture(&options.pbr_textures.normal)
            .with_texture(&options.pbr_textures.specular)
            .finish(handle);

        [camera_bind_group, world_bind_group, texture_bind_group]
    }

    fn pipeline_options<'a>(&self, _: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[Vertex::LAYOUT, Instance::LAYOUT],
            cull_mode: Some(match self {
                RenderType::Terrain => Face::Front,
                RenderType::Sky => Face::Back,
            }),
            depth_write_enabled: matches!(self, RenderType::Terrain),
        }
    }
}
