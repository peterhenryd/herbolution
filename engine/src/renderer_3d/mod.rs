use std::iter::once;
use wgpu::{RenderPass, ShaderStages, TextureFormat};
use lib::geometry::cuboid::face::Faces;
use vertex::InstanceShaderPayload;
use math::color::{ColorConsts, Rgb, Rgba};
use math::num::traits::ConstZero;
use math::projection::perspective::Perspective;
use math::size::Size2;
use math::vector::{vec3i, Vec3};
use crate::camera::Camera;
use crate::camera::frustum::Frustum;
use crate::gpu::handle::Handle;
use crate::gpu::mem::buffer::UnaryBuffer;
use crate::gpu::mem::model::InstanceGroup;
use crate::gpu::mem::payload::ShaderPayload;
use crate::renderer_3d::lighting::{Lighting, PointLight};
use crate::renderer_3d::pipeline::Pipeline3D;
use crate::renderer_3d::vertex::Instance;

pub mod vertex;
pub mod pipeline;
pub mod lighting;

pub struct Renderer3D {
    pub camera: UnaryBuffer<Camera<Perspective>>,
    pub lighting: Lighting,
    pipeline: Pipeline3D,
    highlight_tile: InstanceGroup,
}

impl Renderer3D {
    pub fn create(handle: &Handle, size: Size2<u32>, format: TextureFormat) -> Self {
        let camera = Camera::new(Vec3::ZERO, Perspective::from(size));
        let camera = handle.create_unary_buffer(camera, ShaderStages::VERTEX_FRAGMENT);
        let mut lighting = Lighting::create(handle);
        lighting.point_light_set.push(PointLight {
            color: Rgb::WHITE,
            intensity: 0.5,
            position: Vec3::new(0., 128., 0.),
            range: 32.0,
        });
        lighting.submit(handle);
        let pipeline = Pipeline3D::create(handle, &camera, &lighting, format);
        let highlight_tile = InstanceGroup::create::<InstanceShaderPayload>(handle, &[]);

        Self { camera, lighting, pipeline, highlight_tile }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.camera.projection.set_size(size);
    }

    pub fn update(&mut self, handle: &Handle) {
        self.camera.submit(handle);
        self.lighting.submit(handle);
    }

    pub fn set_highlighted_tile(&mut self, handle: &Handle, position: Option<vec3i>) {
        let Some(position) = position else {
            self.highlight_tile.write::<InstanceShaderPayload>(handle, &[]);
            return;
        };

        let position = position.cast().unwrap();
        let instances = Faces::all().iter()
            .map(|x| x.variant().into_quat())
            .map(|rotation| Instance {
                position,
                rotation,
                texture_index: 0,
                color: Rgba::BLACK,
            }.payload())
            .collect::<Vec<_>>();
        self.highlight_tile.write(handle, &instances);
    }

    pub fn render(&self, render_pass: &mut RenderPass, chunk_meshes: impl Iterator<Item = (vec3i, &InstanceGroup)>) {
        let frustum = Frustum::new(&self.camera);
        let chunk_meshes = chunk_meshes
            .filter(|&(position, _)| frustum.contains_chunk(position, 32))
            .map(|(_, group)| group);

        self.pipeline.render(render_pass, chunk_meshes, once(&self.highlight_tile));
    }
}