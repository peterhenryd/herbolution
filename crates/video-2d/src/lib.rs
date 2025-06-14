use std::path::PathBuf;
use gpu::buffer::{Buffer, GrowBuffer, Usage};
use gpu::camera::{Camera, CameraPayload};
use gpu::handle::Handle;
use gpu::instance::Sets;
use gpu::{load_shader, pipeline, sampler, shader};
use gpu::bind_group::BindGroup;
use gpu::mesh::{Mesh, Meshes};
use gpu::pipeline::{Face, PipelineOptions};
use gpu::sampler::Filter;
use gpu::shader::Stage;
use gpu::texture::{AtlasTextureCoord, Texture};
use math::proj::Orthographic;
use crate::vertex::{Instance2d, Instance2dPayload, Vertex2d};

pub mod vertex;
mod draw;

pub type Mesh2d = Mesh<Vertex2d, u16>;
pub type Meshes2d = Meshes<Vertex2d, u16>;

pub type GrowBuffer2d = GrowBuffer<Instance2dPayload>;
pub type Buffer2d = Buffer<Instance2dPayload>;
pub type Sets2d = Sets<Instance2dPayload>;

pub use draw::Drawing;

#[derive(Debug)]
pub struct Renderer {
    pub(crate) pipeline_map: pipeline::Map<RenderType, 2>,
    camera_buffer: Buffer<CameraPayload>,
    texture_coords: Vec<AtlasTextureCoord>,
    pub(crate) meshes: Meshes2d,
    pub(crate) instance_sets: Sets2d,
}

pub struct Options {
    pub texture_paths: Vec<PathBuf>,
}

impl Renderer {
    pub fn create(handle: &Handle, options: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(handle, 1, Usage::UNIFORM | Usage::COPY_DST);

        let images = options.texture_paths.into_iter()
            .map(|x| image::open(x).unwrap())
            .collect::<Vec<_>>();
        let (texture, texture_coords) = Texture::atlas(handle, images).unwrap();

        Self {
            pipeline_map: pipeline::Map::create(handle, &RenderType2dOptions {
                camera_buffer: &camera_buffer,
                shader_module: &load_shader!(handle, "shader.wgsl"),
                texture: &texture
            }),
            camera_buffer,
            texture_coords,
            meshes: Meshes::new(handle),
            instance_sets: Sets::new(handle),
        }
    }

    pub fn update_camera(&mut self, handle: &Handle, camera: &Camera<Orthographic>) {
        self.camera_buffer.write(handle, 0, &[camera.payload()]).expect("Failed to update 2D camera buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes2d {
        &mut self.meshes
    }

    pub fn texture_coords(&self) -> &[AtlasTextureCoord] {
        &self.texture_coords
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderType;

#[derive(Debug)]
pub struct RenderType2dOptions<'a> {
    camera_buffer: &'a Buffer<CameraPayload>,
    shader_module: &'a shader::Module,
    texture: &'a Texture,
}

impl pipeline::Key<2> for RenderType {
    type Options<'a> = RenderType2dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self];

    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; 2] {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, Stage::VERTEX_FRAGMENT)
            .finish(handle);

        let sampler = handle.create_sampler(sampler::Options { filter: Filter::Pixelated });
        let texture_bind_group = BindGroup::build()
            .with_sampler(&sampler)
            .with_texture(options.texture)
            .finish(handle);

        [
            camera_bind_group,
            texture_bind_group,
        ]
    }

    fn pipeline_options<'a>(&self, _: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[
                Vertex2d::LAYOUT,
                Instance2d::LAYOUT,
            ],
            cull_mode: Some(Face::Back),
            depth_write_enabled: false,
        }
    }
}
