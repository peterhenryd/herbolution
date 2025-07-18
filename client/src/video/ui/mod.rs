use std::fs::read;
use std::path::Path;
use std::slice;

use fontdue::{Font, FontSettings};
use lib::proj::Orthographic;
use lib::size::{size2f, size2u};
use lib::vector::Vec3;
use wgpu::{BufferUsages, ShaderModule, ShaderStages};

use crate::video::camera::{VideoCamera, View};
use crate::video::gpu;
use crate::video::gpu::Handle;
use crate::video::resource::{
    BindGroup, Buffer, CompiledShaders, Filter, Mesh, MeshId, Meshes, PipelineMap, PipelineOptions, PipelineType, SampleCount, SamplerOptions, Sets,
    ShaderSources, Texture,
};
use crate::video::ui::atlas::Atlas;
use crate::video::ui::brush::Text;
use crate::video::ui::font::{FontId, Fonts};
use crate::video::ui::vertex::{Instance2d, Vertex2d};

pub mod atlas;
pub mod brush;
pub mod font;
pub mod vertex;

#[derive(Debug)]
pub struct Painter {
    pub(crate) pipeline_map: PipelineMap<RenderType>,
    camera_buffer: Buffer<VideoCamera>,
    pub(crate) meshes: Meshes<Vertex2d>,
    pub(crate) instance_sets: Sets<Instance2d>,
    atlas: Atlas,
    shaders: CompiledShaders,
    quad_mesh: MeshId,
}

fn filter(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | ',' | '?' | '!' | '-' | '_' | ':' | ';' | '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\')
}

impl Painter {
    pub fn create(gpu: &gpu::Handle, sample_count: SampleCount, asset_path: &Path) -> Self {
        let camera_buffer = Buffer::create(gpu, 1, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let shaders = ShaderSources::default()
            .with("core", include_str!("../shaders/ui.wgsl"))
            .compile(gpu)
            .expect("Failed to compile shaders");

        let mut fonts = Fonts::build();
        fonts.set_filter(filter);
        fonts.add_font(
            Font::from_bytes(read(asset_path.join("font/editundo.ttf")).unwrap(), FontSettings::default()).unwrap(),
            [12.0, 24.0, 36.0, 54.0, 96.0],
        );
        let fonts = fonts.finish();
        let atlas = Atlas::create(gpu, &fonts);

        let mut meshes = Meshes::new(gpu);
        let quad_mesh = meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad_unit.toml")));

        Self {
            pipeline_map: PipelineMap::create(
                gpu,
                &RenderType2dOptions {
                    camera_buffer: &camera_buffer,
                    shader_module: shaders.get_module("core").unwrap(),
                    texture: &atlas.texture,
                },
                sample_count,
            ),
            camera_buffer,
            meshes,
            shaders,
            instance_sets: Sets::new(gpu),
            atlas,
            quad_mesh,
        }
    }

    pub fn set_sample_count(&mut self, gpu: &Handle, sample_count: SampleCount) {
        self.pipeline_map.set_sample_count(
            gpu,
            sample_count,
            &RenderType2dOptions {
                camera_buffer: &self.camera_buffer,
                shader_module: &self.shaders.get_module("core").unwrap(),
                texture: &self.atlas.texture,
            },
        );
    }

    pub fn update_camera(&mut self, gpu: &Handle, camera: &VideoCamera) {
        self.camera_buffer
            .write(gpu, 0, slice::from_ref(camera))
            .expect("Failed to update 2D camera buffer");
    }

    pub fn meshes(&mut self) -> &mut Meshes<Vertex2d, u16> {
        &mut self.meshes
    }

    pub fn atlas(&self) -> &Atlas {
        &self.atlas
    }

    pub fn set_resolution(&mut self, gpu: &Handle, resolution: size2u) {
        self.update_camera(gpu, &VideoCamera::new(Vec3::ZERO, View::Forward, Orthographic::from(resolution)));
    }

    pub fn compute_text_size(&self, text: &Text) -> Result<size2f, char> {
        let mut width = 0.0;
        let mut height: f32 = 0.0;

        for char in text.content.chars() {
            let Some(coord) = self
                .atlas
                .glyph_coord(text.font_id, char, text.font_size)
            else {
                return Err(char);
            };

            width += coord.metrics.advance_width;
            height = height.max(coord.metrics.height as f32);
        }

        Ok(size2f::new(width, height))
    }

    pub fn default_font_id(&self) -> FontId {
        self.atlas
            .font_coords
            .iter()
            .next()
            .unwrap()
            .0
            .font_id
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderType;

#[derive(Debug)]
pub struct RenderType2dOptions<'a> {
    camera_buffer: &'a Buffer<VideoCamera>,
    shader_module: &'a ShaderModule,
    texture: &'a Texture,
}

impl PipelineType for RenderType {
    type Options<'a> = RenderType2dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup> {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, ShaderStages::VERTEX_FRAGMENT)
            .finish(gpu);

        let sampler = gpu.create_sampler(SamplerOptions { filter: Filter::Pixelated });
        let texture_bind_group = BindGroup::build()
            .with_sampler(&sampler)
            .with_texture(options.texture, true)
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
