use std::path::PathBuf;
use crate::camera::{Camera, CameraPayload};
use crate::gpu::Handle;
use crate::load_shader;
use crate::mem::bind_group::BindGroup;
use crate::mem::buffer::{Buffer, GrowBuffer};
use crate::pipeline::{PipelineOptions, Pipelines, RenderType};
use math::proj::Orthographic;
use wgpu::{AddressMode, Face, FilterMode, SamplerDescriptor, ShaderModule, ShaderStages};
use crate::mem::instance::InstanceSets;
use crate::mem::mesh::{Mesh, Meshes};
use crate::mem::texture::{AtlasTextureCoord, Texture};
use crate::r2d::vertex::{Instance2d, Instance2dPayload, Vertex2d};

pub mod vertex;

pub type Mesh2d = Mesh<Vertex2d, u16>;
pub type Meshes2d = Meshes<Vertex2d, u16>;

pub type GrowBuffer2d = GrowBuffer<Instance2dPayload>;
pub type Buffer2d = Buffer<Instance2dPayload>;
pub type Sets2d = InstanceSets<Instance2dPayload>;

#[derive(Debug)]
pub struct Renderer2d {
    pub(crate) pipelines: Pipelines<RenderType2d, 2>,
    camera_buffer: Buffer<CameraPayload>,
    pub(crate) meshes: Meshes2d,
    pub(crate) instance_sets: Sets2d,
    texture_coords: Vec<AtlasTextureCoord>,
}

pub struct Options {
    pub texture_paths: Vec<PathBuf>,
}

impl Renderer2d {
    pub fn create(handle: &Handle, options: Options) -> Self {
        let camera_buffer = Buffer::with_capacity(handle, 1, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        
        let images = options.texture_paths.into_iter()
            .map(|x| image::open(x).unwrap())
            .collect::<Vec<_>>();
        let (texture, texture_coords) = Texture::atlas(handle, images).unwrap();
        
        Self {
            pipelines: Pipelines::create(handle, &RenderType2dOptions {
                camera_buffer: &camera_buffer,
                shader_module: &load_shader!(handle, "shader.wgsl"),
                texture: &texture
            }),
            camera_buffer,
            meshes: Meshes::new(handle),
            instance_sets: InstanceSets::new(handle),
            texture_coords,
        }
    }
    
    pub fn update_camera(&mut self, handle: &Handle, camera: &Camera<Orthographic>) {
        self.camera_buffer.write(handle, 0, &[camera.payload()]).expect("Failed to update 2D camera buffer");
    }
    
    pub fn meshes(&mut self) -> &mut Meshes2d {
        &mut self.meshes
    }
    
    pub fn instance_sets(&mut self) -> &mut Sets2d {
        &mut self.instance_sets
    }
    
    pub fn texture_coords(&self) -> &[AtlasTextureCoord] {
        &self.texture_coords
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] 
pub struct RenderType2d;
 
#[derive(Debug)]
pub struct RenderType2dOptions<'a> {
    camera_buffer: &'a Buffer<CameraPayload>,
    shader_module: &'a ShaderModule,
    texture: &'a Texture,
}

impl RenderType<2> for RenderType2d {
    type Options<'a> = RenderType2dOptions<'a>;
    const ENTRIES: &'static [Self] = &[Self];

    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; 2] {
        let camera_bind_group = BindGroup::build()
            .with_buffer(options.camera_buffer, ShaderStages::VERTEX_FRAGMENT)
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