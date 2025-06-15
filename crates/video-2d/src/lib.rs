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
use gpu::shader::Stage;
use gpu::texture::Texture;
use gpu::{MeshId, load_shader, mesh, pipeline, sampler, shader};
use math::proj::Orthographic;

use crate::vertex::{Instance2d, Instance2dPayload, Vertex2d};

mod atlas;
mod draw;
mod font;
pub mod text;
pub mod ui;
pub mod vertex;

pub type Mesh2d = Mesh<Vertex2d, u16>;
pub type Meshes2d = Meshes<Vertex2d, u16>;

pub type GrowBuffer2d = GrowBuffer<Instance2dPayload>;
pub type Buffer2d = Buffer<Instance2dPayload>;
pub type Sets2d = Sets<Instance2dPayload>;

pub use draw::Drawing;
use math::size::Size2;
use math::vector::Vec3;

use crate::atlas::Atlas;
use crate::font::Fonts;

#[derive(Debug)]
pub struct Renderer {
    pub(crate) pipeline_map: pipeline::Map<RenderType, 2>,
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

impl Renderer {
    pub fn create(handle: &Handle, _: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(handle, 1, Usage::UNIFORM | Usage::COPY_DST);

        let mut fonts = Fonts::build();
        fonts.set_filter(filter);
        fonts.add_font(
            Font::from_bytes(read("assets/font/editundo.ttf").unwrap(), FontSettings::default()).unwrap(),
            [12.0, 36.0],
        );
        let fonts = fonts.finish();
        let atlas = Atlas::create(handle, &fonts);

        let mut meshes = Meshes2d::new(handle);
        let quad_mesh = meshes.create_and_insert_from(mesh::tl_quad);

        Self {
            pipeline_map: pipeline::Map::create(
                handle,
                &RenderType2dOptions {
                    camera_buffer: &camera_buffer,
                    shader_module: &load_shader!(handle, "shader.wgsl"),
                    texture: &atlas.texture,
                },
            ),
            camera_buffer,
            meshes,
            instance_sets: Sets::new(handle),
            atlas,
            quad_mesh,
        }
    }

    pub fn update_camera(&mut self, handle: &Handle, camera: &Camera<Orthographic>) {
        self.camera_buffer
            .write(handle, 0, &[camera.payload()])
            .expect("Failed to update 2D camera buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes2d {
        &mut self.meshes
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    pub fn set_resolution(&mut self, handle: &Handle, resolution: Size2<u32>) {
        self.update_camera(
            handle,
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

        [camera_bind_group, texture_bind_group]
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
