use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use hashbrown::HashMap;
use prims::aabb::Aabb2;
use prims::color::{ColorConsts, Rgb, Rgba};
use prims::matrix::{Mat4, mat4f};
use prims::rotation::Quat;
use prims::size::{Size3, size3f};
use prims::spatial::CubeFace;
use prims::vector::{Vec4, vec2f, vec3f, vec4f, vec4u};

use crate::{AssetError, Buffer, BufferUsagesBuilderExt, ColorTargetStateExt, DepthStencilStateExt, Gpu, Texture, ToBinding, Uniform, shaders};

pub type TextureCoords3d = Arc<HashMap<String, Aabb2<f32>>>;

#[derive(Debug)]
pub struct Core3d {
    env: Uniform<Env3d>,

    texture_sampler: wgpu::Sampler,
    albedo_texture: Texture,
    emissive_texture: Texture,
    height_texture: Texture,
    normal_texture: Texture,
    orm_texture: Texture,
    texture_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_coords: TextureCoords3d,

    lights: Vec<Light>,
    lights_buffer: Buffer<Light>,
    lights_bind_group_layout: wgpu::BindGroupLayout,
    lights_bind_group: wgpu::BindGroup,

    render_pipeline: wgpu::RenderPipeline,
}

impl Core3d {
    pub fn new(gpu: &Gpu) -> Result<Self, AssetError> {
        let env = gpu
            .build_uniform(&Env3d {
                view_proj: Mat4::IDENTITY,
                camera_pos: Vec4::ZERO,
                jitter: Vec4::ZERO,
                light_count: 0,
            })
            .use_copy_dst()
            .create();

        let sampler = gpu.build_sampler().create();

        let albedo_texture = gpu
            .build_texture(2)
            .data(&[Rgba::<u8>::WHITE])
            .fill()
            .use_texture_binding()
            .create();
        let emissive_texture = gpu
            .build_texture(2)
            .data(&[Rgba::<u8>::BLACK])
            .fill()
            .use_texture_binding()
            .create();
        let height_texture = gpu
            .build_texture(2)
            .data(&[Rgba::<u8>::BLACK])
            .fill()
            .use_texture_binding()
            .create();
        let normal_texture = gpu
            .build_texture(2)
            .data(&[Rgba::<u8> {
                r: 127,
                g: 127,
                b: 255,
                a: 255,
            }])
            .fill()
            .use_texture_binding()
            .create();
        let orm_texture = gpu
            .build_texture(2)
            .data(&[Rgba::<u8>::WHITE])
            .fill()
            .use_texture_binding()
            .create();

        let (texture_bind_group, texture_bind_group_layout) = gpu
            .build_bind_group()
            .add_entry(sampler.to_binding())
            .add_entry(albedo_texture.to_binding())
            .add_entry(emissive_texture.to_binding())
            .add_entry(height_texture.to_binding())
            .add_entry(normal_texture.to_binding())
            .add_entry(orm_texture.to_binding())
            .create_with_layout();

        let lights_buffer = gpu
            .build_buffer()
            .size(size_of::<vec4u>() + size_of::<Light>())
            .use_storage()
            .use_copy_dst()
            .create();
        let (lights_bind_group, lights_bind_group_layout) = gpu
            .build_bind_group()
            .add_entry(lights_buffer.to_binding().storage())
            .create_with_layout();

        let pipeline_layout = gpu
            .build_pipeline_layout()
            .add_bind_group_layout(env.bind_group_layout())
            .add_bind_group_layout(&texture_bind_group_layout)
            .add_bind_group_layout(&lights_bind_group_layout)
            .create();
        let shader_module = shaders::world(&gpu, "Herbolution 3D Shader Module");
        let render_pipeline = gpu
            .build_render_pipeline()
            .layout(&pipeline_layout)
            .depth_stencil(wgpu::DepthStencilState::f32_less())
            .shader(&shader_module, "vs", "fs")
            .add_color_target(wgpu::ColorTargetState::bgra_all())
            .create();

        Ok(Self {
            env,
            texture_sampler: sampler,
            albedo_texture,
            emissive_texture,
            height_texture,
            normal_texture,
            orm_texture,
            texture_bind_group,
            texture_bind_group_layout: texture_bind_group_layout.into_owned(),
            texture_coords: Arc::new(HashMap::new()),
            lights: vec![],
            lights_buffer,
            lights_bind_group_layout: lights_bind_group_layout.into_owned(),
            lights_bind_group,

            render_pipeline,
        })
    }

    pub fn env(&mut self) -> &mut Uniform<Env3d> {
        &mut self.env
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Env3d {
    pub view_proj: mat4f,
    pub camera_pos: vec4f,
    pub jitter: vec4f,
    pub light_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pos: vec4f,
    color: Rgba<f32>,
    dir: vec4f,
    packed_vars: vec4f,
}

impl Light {
    #[inline]
    pub fn dir(dir: vec3f, color: Rgb<f32>, radiance: f32) -> Self {
        Self::from(DirLight { dir, color, radiance })
    }

    #[inline]
    pub fn point(pos: vec3f, color: Rgb<f32>, radiance: f32) -> Self {
        Self::from(PointLight { pos, color, radiance })
    }
}

impl From<DirLight> for Light {
    #[inline]
    fn from(value: DirLight) -> Self {
        Self {
            pos: Vec4::ZERO,
            color: value.color.extend(1.0),
            dir: value.dir.extend(0.0),
            packed_vars: Vec4::new(value.radiance, f32::from_bits(0), 0.0, 0.0),
        }
    }
}

impl From<PointLight> for Light {
    #[inline]
    fn from(value: PointLight) -> Self {
        Self {
            pos: value.pos.extend(0.0),
            color: value.color.extend(1.0),
            dir: Vec4::ZERO,
            packed_vars: Vec4::new(value.radiance, f32::from_bits(1), 0.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct DirLight {
    pub dir: vec3f,
    pub color: Rgb<f32>,
    pub radiance: f32,
}

#[derive(Debug)]
pub struct PointLight {
    pub pos: vec3f,
    pub color: Rgb<f32>,
    pub radiance: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex3d {
    pub pos: vec3f,
    pub uv: vec2f,
    pub normal: vec3f,
    pub tangent: vec3f,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance3d {
    model_0: vec4f,
    model_1: vec4f,
    model_2: vec4f,
    model_3: vec4f,
    uv: vec4f,
    color: Rgba<f32>,
    ao: vec4f,
}

impl Instance3d {
    pub fn new(pos: vec3f, quat: Quat, size: size3f, visual: impl Into<Visual>, ao: Option<vec4f>) -> Self {
        let model = quat.to_axes().mul_cols(size.to_vec3());
        let (uv, color) = match visual.into() {
            Visual::Uv(coords) => (coords.pack_transform(), Rgba::TRANSPARENT),
            Visual::Color(color) => (Vec4::ZERO, color),
        };

        Self {
            model_0: model.x.extend(0.0),
            model_1: model.y.extend(0.0),
            model_2: model.z.extend(0.0),
            model_3: pos.extend(1.0),
            uv,
            color,
            ao: ao.unwrap_or(Vec4::ONE),
        }
    }
}

pub enum Visual {
    Uv(Aabb2<f32>),
    Color(Rgba<f32>),
}

impl From<Aabb2<f32>> for Visual {
    fn from(value: Aabb2<f32>) -> Self {
        Self::Uv(value)
    }
}

impl From<Rgba<f32>> for Visual {
    fn from(value: Rgba<f32>) -> Self {
        Self::Color(value)
    }
}

pub fn cube(position: vec3f, color: Rgba<f32>) -> impl IntoIterator<Item = Instance3d> {
    CubeFace::values()
        .map(CubeFace::rotation)
        .map(move |rotation| Instance3d::new(position, rotation, Size3::splat(1.0), color, None))
}
