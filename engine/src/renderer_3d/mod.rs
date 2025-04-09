use crate::camera::{Camera, Frustum};
use crate::gpu::handle::Handle;
use crate::gpu::mem::buffer::UnaryBuffer;
use crate::gpu::mem::model::InstanceGroup;
use crate::gpu::mem::payload::ShaderPayload;
use crate::renderer_3d::pipeline::Pipeline3D;
use crate::renderer_3d::vertex::Instance;
use lib::geometry::cuboid::face::{Face, Faces};
use math::color::{ColorConsts, Rgba};
use math::num::traits::ConstZero;
use math::proj::Perspective;
use math::size::Size2;
use math::vector::{vec3f, vec3i, Vec3};
use std::iter::once;
use bytemuck::Zeroable;
use vertex::InstanceShaderPayload;
use wgpu::{RenderPass, ShaderStages, TextureFormat};

pub mod vertex;
pub mod pipeline;

pub struct Renderer3D {
    pub camera: UnaryBuffer<Camera<Perspective>>,
    pipeline: Pipeline3D,
    highlight_tile: InstanceGroup,
    frustum: Frustum,
    skybox: InstanceGroup,
}

impl Renderer3D {
    pub fn create(handle: &Handle, size: Size2<u32>, format: TextureFormat) -> Self {
        let camera = Camera::new(Vec3::ZERO, Perspective::from(size));
        let camera = handle.create_unary_buffer(camera, ShaderStages::VERTEX_FRAGMENT);
        let pipeline = Pipeline3D::create(handle, &camera, format);
        let highlight_tile = InstanceGroup::create::<InstanceShaderPayload>(handle, &[]);
        let frustum = Frustum::new(camera.view_proj_matrix());
        let mut skybox = InstanceGroup::create::<InstanceShaderPayload>(handle, &[InstanceShaderPayload::zeroed(); 6]);
        update_skybox(&mut skybox, handle, camera.pos);

        Self { camera, pipeline, highlight_tile, frustum, skybox }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.camera.proj.set_size(size);
    }

    pub fn update(&mut self, handle: &Handle) {
        let is_dirty = self.camera.is_dirty();
        self.camera.submit(handle);
        if is_dirty {
            self.frustum = Frustum::new(self.camera.view_proj_matrix());
            update_skybox(&mut self.skybox, handle, self.camera.pos);
        }
    }

    pub fn set_highlighted_tile(&mut self, handle: &Handle, pos: Option<vec3i>) {
        let Some(pos) = pos else {
            self.highlight_tile.write::<InstanceShaderPayload>(handle, &[]);
            return;
        };

        let pos = pos.cast().unwrap();
        let instances = Faces::all().iter()
            .map(|x| x.variant().into_quat())
            .map(|quat| Instance {
                pos,
                quat,
                texture_index: 0,
                color: Rgba::BLACK,
                is_lit: false,
            }.payload())
            .collect::<Vec<_>>();
        self.highlight_tile.write(handle, &instances);
    }

    pub fn render(&self, render_pass: &mut RenderPass, chunk_meshes: impl Iterator<Item = (vec3i, &InstanceGroup)>) {
        let chunk_meshes = chunk_meshes
            .filter(|&(pos, _)| self.frustum.contains_cube(pos.cast().unwrap(), 32.0))
            .map(|(_, group)| group);

        self.pipeline.render(render_pass, &self.skybox, chunk_meshes, once(&self.highlight_tile));
    }
}

fn update_skybox(instance_group: &mut InstanceGroup, handle: &Handle, pos: vec3f) {
    instance_group.write(handle, &[
        Instance {
            pos,
            quat: Face::Front.into_quat(),
            texture_index: 4,
            ..Default::default()
        }.payload(),
        Instance {
            pos,
            quat: Face::Back.into_quat(),
            texture_index: 5,
            ..Default::default()
        }.payload(),
        Instance {
            pos,
            quat: Face::Top.into_quat(),
            texture_index: 6,
            ..Default::default()
        }.payload(),
        Instance {
            pos,
            quat: Face::Bottom.into_quat(),
            texture_index: 7,
            ..Default::default()
        }.payload(),
        Instance {
            pos,
            quat: Face::Left.into_quat(),
            texture_index: 8,
            ..Default::default()
        }.payload(),
        Instance {
            pos,
            quat: Face::Right.into_quat(),
            texture_index: 9,
            ..Default::default()
        }.payload(),
    ]);
}