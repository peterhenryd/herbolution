pub mod chunk_mesh;
pub mod geometry;
mod model;

use crate::engine::camera::{Camera, ViewRotation};
use crate::engine::gpu::buffer::uniform::Uniform;
use crate::engine::gpu::pipeline::Pipeline;
use crate::engine::gpu::surface::Surface;
use crate::engine::gpu::texture::Texture;
use crate::engine::gpu::Gpu;
use crate::engine::projection::{Perspective, Projection};
use crate::game::chunk::material::Material;
use crate::game::chunk::section::ChunkSection;
use crate::world::chunk_mesh::ChunkMesh;
use crate::world::geometry::quad::Quad;
use crate::world::geometry::vertex::WorldVertex;
use crate::world::model::Model;
use glam::{IVec3, Vec3};
use std::collections::HashMap;
use wgpu::{include_wgsl, FilterMode, RenderPass, SamplerBindingType, ShaderStages};
use winit::dpi::PhysicalSize;

/// The entire rendering state for the world of the game.
pub struct WorldRenderer {
    gpu: Gpu,
    pipeline: Pipeline,
    quad_model: Model,
    pub chunk_meshes: HashMap<IVec3, ChunkMesh>,
    pub camera: Uniform<Camera<Perspective>>,
}

impl WorldRenderer {
    pub fn create(gpu: &Gpu, surface: &Surface) -> Self {
        let camera = Camera::perspective(Vec3::ZERO, ViewRotation::default(), surface.get_size());
        let camera = Uniform::new(gpu, "camera", camera, ShaderStages::VERTEX);

        let textures = [Texture::open(gpu, "assets/texture/grass.png").unwrap()];
        let texture_views = textures
            .iter()
            .map(Texture::create_rgba_2d_array_view)
            .collect::<Vec<_>>();
        let texture_view_refs = texture_views.iter().collect::<Vec<_>>();
        let sampler = gpu.create_sampler(FilterMode::Nearest);

        Self {
            gpu: gpu.clone(),
            pipeline: gpu
                .build_pipeline("world", surface.get_texture_format())
                .with_shader(include_wgsl!("shader.wgsl"))
                .with_vertex_buffer_layout(WorldVertex::layout())
                .with_vertex_buffer_layout(Quad::layout())
                .build_binding("world_camera", |b| b.with_uniform_buffer(&camera))
                .build_binding("world_textures", |b| {
                    b.with_texture_array(&texture_view_refs)
                        .with_sampler(SamplerBindingType::Filtering, &sampler)
                })
                .finish(),
            chunk_meshes: HashMap::new(),
            quad_model: Quad::model(gpu),
            camera,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.camera.projection.resize(size);
    }

    pub fn update(&self) {
        self.camera.write_changes();
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.pipeline.bind(render_pass);
        render_pass.set_vertex_buffer(0, self.quad_model.mesh_buffer.slice(..));
        render_pass.set_index_buffer(
            self.quad_model.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        for mesh in self.chunk_meshes.values() {
            mesh.render(render_pass);
        }
    }

    pub fn add_chunk_mesh(&mut self, section: &ChunkSection) {
        let mut mesh = ChunkMesh::new(&self.gpu, section.position);
        for (i, &material) in section.data.iter().enumerate() {
            if let Material::Air = material {
                continue;
            }

            mesh.set_linear_cube(i, material);
        }
        mesh.update();
        self.chunk_meshes.insert(section.position, mesh);
    }
}
