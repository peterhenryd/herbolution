use std::fs::read;
use std::path::PathBuf;

use fontdue::{Font, FontSettings};
use gpu::bind_group::BindGroup;
use gpu::buffer::{Buffer, GrowBuffer, Usage};
use gpu::camera::{Camera, CameraPayload, View};
use gpu::handle::Handle;
use gpu::instance::Sets;
use gpu::mesh::{Mesh, Meshes};
use gpu::pipeline::PipelineOptions;
use gpu::sampler::Filter;
use gpu::texture::Texture;
use gpu::{load_shader, pipeline, sampler, shader, MeshId};
use math::proj::Orthographic;

pub mod atlas;
pub mod brush;
pub mod font;
pub mod text;
pub mod vertex;

pub type Mesh2d = Mesh<Vertex2d, u16>;
pub type Meshes2d = Meshes<Vertex2d, u16>;

pub type GrowBuffer2d = GrowBuffer<Instance2dPayload>;
pub type Buffer2d = Buffer<Instance2dPayload>;
pub type Sets2d = Sets<Instance2dPayload>;

use gpu::pipeline::map::PipelineMap;
use gpu::shader::Stage;
use math::size::size2u;
use math::vec::Vec3;

use crate::painter::atlas::Atlas;
use crate::painter::font::Fonts;
use crate::painter::vertex::{Instance2d, Instance2dPayload, Vertex2d};

#[derive(Debug)]
pub struct Painter {
    pub(crate) pipeline_map: PipelineMap<RenderType>,
    camera_buffer: Buffer<CameraPayload>,
    pub(crate) meshes: Meshes2d,
    pub(crate) instance_sets: Sets2d,
    atlas: Atlas,
    quad_mesh: MeshId,
}

pub struct Options {
    pub texture_paths: Vec<PathBuf>,
}

fn filter(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | ',' | '?' | '!' | '-' | '_' | ':' | ';' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\')
}

impl Painter {
    pub fn create(gpu: &Handle, _: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(gpu, 1, Usage::UNIFORM | Usage::COPY_DST);

        let mut fonts = Fonts::build();
        fonts.set_filter(filter);
        fonts.add_font(
            Font::from_bytes(read("assets/font/editundo.ttf").unwrap(), FontSettings::default()).unwrap(),
            [12.0, 24.0, 36.0],
        );
        let fonts = fonts.finish();
        let atlas = Atlas::create(gpu, &fonts);

        let mut meshes = Meshes2d::new(gpu);
        let quad_mesh = meshes.create_and_insert_from(|handle| Mesh::read(handle, "assets/mesh/quad_unit.toml"));

        Self {
            pipeline_map: PipelineMap::create(
                gpu,
                &RenderType2dOptions {
                    camera_buffer: &camera_buffer,
                    shader_module: &load_shader!(gpu, "shader.wgsl"),
                    texture: &atlas.texture,
                },
            ),
            camera_buffer,
            meshes,
            instance_sets: Sets::new(gpu),
            atlas,
            quad_mesh,
        }
    }

    pub fn update_camera(&mut self, gpu: &Handle, camera: &Camera<Orthographic>) {
        self.camera_buffer
            .write(gpu, 0, &[camera.payload()])
            .expect("Failed to update 2D camera buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes2d {
        &mut self.meshes
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    pub fn set_resolution(&mut self, gpu: &Handle, resolution: size2u) {
        self.update_camera(
            gpu,
            &Camera {
                position: Vec3::ZERO,
                view: View::Forward,
                proj: Orthographic::from(resolution),
            },
        );
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

impl pipeline::Key for RenderType {
    type Options<'a> = RenderType2dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup> {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, Stage::VERTEX_FRAGMENT)
            .finish(gpu);

        let sampler = gpu.create_sampler(sampler::Options { filter: Filter::Pixelated });
        let texture_bind_group = BindGroup::build()
            .with_sampler(&sampler)
            .with_texture(options.texture)
            .finish(gpu);

        vec![camera_bind_group, texture_bind_group]
    }

    fn pipeline_options<'a>(&self, _: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a> {
        PipelineOptions {
            shader_module: options.shader_module,
            vertex_buffer_layouts: &[Vertex2d::LAYOUT, Instance2d::LAYOUT],
            cull_mode: None,
            depth_write_enabled: false,
        }
    }
}
