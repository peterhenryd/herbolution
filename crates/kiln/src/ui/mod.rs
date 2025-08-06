use std::cmp::max;
use std::fmt::Debug;
use std::iter::zip;

use bytemuck::{Pod, Zeroable};
use hashbrown::HashMap;
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption, AtlasMipOption};
use prims::aabb::Aabb2;
use prims::calculate_atlas_size;
use prims::color::{ColorConsts, Rgba};
use prims::matrix::{Mat4, mat4f};
use prims::size::size2f;
use prims::vector::{Vec2, Vec4, vec2f, vec4f};

use crate::{AssetError, Assets, BindGroupLayoutEntryBuilder, ColorTargetStateExt, DepthStencilStateExt, Gpu, Texture, ToBinding, Uniform, shaders};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Env2d {
    pub view_proj: mat4f,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Quad {
    transform: vec4f,
    visual: vec4f,
    mask: vec4f,
    packed_vars: vec4f,
}

impl Quad {
    pub fn new(pos: vec2f, size: size2f, rotation: f32, border_radius: f32, visual: impl Into<Visual2d>, mask: Option<Aabb2<f32>>) -> Self {
        let mut flags = 0;
        let visual = match visual.into() {
            Visual2d::Image { uv } => uv.pack_transform(),
            Visual2d::Color(color) => {
                flags |= 1;
                color.to_vec4()
            }
        };

        Self {
            transform: Vec4::new(pos.x, pos.y, size.width, size.height),
            visual,
            mask: mask
                .map(Aabb2::pack_transform)
                .unwrap_or(Vec4::ZERO),
            packed_vars: Vec4::new(border_radius, rotation, f32::from_bits(flags), 0.0),
        }
    }
}

pub enum Visual2d {
    Image { uv: Aabb2<f32> },
    Color(Rgba<f32>),
}

impl From<Aabb2<f32>> for Visual2d {
    fn from(value: Aabb2<f32>) -> Self {
        Self::Image { uv: value }
    }
}

impl From<Rgba<f32>> for Visual2d {
    fn from(value: Rgba<f32>) -> Self {
        Self::Color(value)
    }
}

#[derive(Debug)]
pub struct Core2d {
    env: Uniform<Env2d>,
    texture_sampler: wgpu::Sampler,
    image_texture: Texture,
    //mask_texture: Texture,
    texture_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    image_coords: HashMap<String, Aabb2<f32>>,
    render_pipeline: wgpu::RenderPipeline,
}

impl Core2d {
    pub fn new(gpu: &Gpu) -> Result<Self, AssetError> {
        let env = gpu
            .build_uniform(&Env2d { view_proj: Mat4::IDENTITY })
            .create();
        let texture_sampler = gpu.build_sampler().create();

        let image_texture = gpu
            .build_texture(1)
            .label("Herbolution 2d Image Atlas Texture")
            .data(&[Rgba::<u8>::WHITE])
            .use_texture_binding()
            .use_copy_dst()
            .create();
        let texture_bind_group_layout = gpu
            .build_bind_group_layout()
            .label("Herbolution 2D Image Atlas Texture Bind Group Layout")
            .add_entry(BindGroupLayoutEntryBuilder::sampler(0, wgpu::ShaderStages::FRAGMENT).build())
            .add_entry(BindGroupLayoutEntryBuilder::texture(1, wgpu::ShaderStages::FRAGMENT).build())
            .add_entry(BindGroupLayoutEntryBuilder::texture(2, wgpu::ShaderStages::FRAGMENT).build())
            .create();
        let texture_bind_group = gpu
            .build_bind_group()
            .label("Herbolution 2D Image Atlas Texture Bind Group")
            .layout(&texture_bind_group_layout)
            .add_entry(texture_sampler.to_binding())
            .add_entry(image_texture.to_binding())
            //.add_entry(mask_texture.to_binding())
            .create();

        let pipeline_layout = gpu
            .build_pipeline_layout()
            .label("Herbolution 2D Render Pipeline Layout")
            .add_bind_group_layout(env.bind_group_layout())
            .add_bind_group_layout(&texture_bind_group_layout)
            .create();
        let shader_module = shaders::ui(&gpu, "Herbolution 2D Shader Module");
        let render_pipeline = gpu
            .build_render_pipeline()
            .layout(&pipeline_layout)
            .depth_stencil(wgpu::DepthStencilState::f32_less())
            .shader(&shader_module, "vs", "fs")
            .add_vertex_buffer_layout(wgpu::VertexBufferLayout {
                array_stride: 0,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[],
            })
            .add_vertex_buffer_layout(wgpu::VertexBufferLayout {
                array_stride: size_of::<Quad>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![],
            })
            .add_color_target(wgpu::ColorTargetState::bgra_all())
            .create();

        Ok(Self {
            env,
            texture_sampler,
            image_texture,
            //mask_texture,
            texture_bind_group,
            texture_bind_group_layout,
            image_coords: HashMap::new(),
            render_pipeline,
        })
    }

    pub fn write_images(&mut self, gpu: &Gpu, assets: &Assets, image_names: &[&str]) -> Result<(), AssetError> {
        if image_names.is_empty() {
            return Ok(());
        }

        let entries = image_names
            .iter()
            .map(|name| {
                assets.load_image(name).map(|texture| AtlasEntry {
                    texture,
                    mip: AtlasEntryMipOption::Repeat,
                })
            })
            .try_collect::<Vec<_>>()?;
        let max_dimension = entries
            .iter()
            .map(|entry| max(entry.texture.width(), entry.texture.height()))
            .max()
            .unwrap_or(1);

        let atlas_size = calculate_atlas_size(image_names.len() as u32, max_dimension);
        let atlas = image_atlas::create_atlas(&AtlasDescriptor {
            max_page_count: 1,
            size: atlas_size,
            mip: AtlasMipOption::NoMip,
            entries: &entries,
        })?;

        self.image_texture = gpu
            .build_texture(atlas.size)
            .label("Herbolution 2d Image Atlas Texture")
            .data(&atlas.textures[0].mip_maps[0])
            .use_texture_binding()
            .use_copy_dst()
            .create();

        self.image_coords.clear();
        self.image_coords.extend(zip(
            image_names.iter().map(|s| (*s).to_owned()),
            atlas.texcoords.into_iter().map(|tc| Aabb2 {
                min: Vec2::new(tc.min_x, tc.min_y).cast::<f32>() / tc.size as f32,
                max: Vec2::new(tc.max_x, tc.max_y).cast::<f32>() / tc.size as f32,
            }),
        ));

        self.texture_bind_group = gpu
            .build_bind_group()
            .label("Herbolution 2D Image Atlas Texture Bind Group")
            .layout(&self.texture_bind_group_layout)
            .add_entry(self.texture_sampler.to_binding())
            .add_entry(self.image_texture.to_binding())
            //.add_entry(&self.mask_texture.to_binding())
            .create();

        Ok(())
    }

    pub fn env(&mut self) -> &mut Uniform<Env2d> {
        &mut self.env
    }
}
