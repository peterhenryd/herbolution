use std::fmt::Debug;
use std::hash::Hash;
use hashbrown::HashMap;
use tracing::error;
use wgpu::{BindGroup, BindGroupLayout, IndexFormat, RenderPass, RenderPipeline};
use crate::gpu::binding::UniqueBindGroup;
use crate::gpu::geometry::{InstanceBuffer, Mesh, Primitive};
use crate::gpu::Gpu;

pub struct Renderer<T> {
    pipeline_map: HashMap<T, RenderPipeline>,
    bind_groups: Vec<BindGroup>,
}

pub trait RenderType: Debug + Copy + Eq + Hash {
    type Vertex: Primitive;
    type Instance: Primitive;

    fn create_render_pipeline(&self, gpu: &Gpu, bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline;

    fn set_bind_groups(&self, render_pass: &mut RenderPass, bind_groups: &[BindGroup]);
}

impl<T> Renderer<T>
where T: RenderType {
    pub fn create(gpu: &Gpu, bind_groups: Vec<UniqueBindGroup>, types: &[T]) -> Self {
        let layouts = bind_groups.iter()
            .map(|x| &x.layout)
            .collect::<Vec<_>>();
        let pipeline_map = types.iter()
            .map(|x| (*x, x.create_render_pipeline(gpu, &layouts)))
            .collect();
        let bind_groups = bind_groups.into_iter()
            .map(|x| x.group)
            .collect();

        Self { pipeline_map, bind_groups }
    }

    pub fn render_group(&self, render_pass: &mut RenderPass, group: RenderGroup<T>) {
        let Some(pipeline) = self.pipeline_map.get(&group.render_type) else {
            return error!("Render type {:?} not found in pipeline map", group.render_type);
        };

        render_pass.set_pipeline(pipeline);
        group.render_type.set_bind_groups(render_pass, &self.bind_groups);
        for &InstancedMesh { mesh, instance_buffers } in group.instanced_meshes {
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint16);

            for instance_buffer in instance_buffers {
                render_pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));
                render_pass.draw_indexed(0..mesh.index_count, 0, 0..instance_buffer.count);
            }
        }
    }
}

pub struct RenderGroup<'a, T> {
    pub render_type: T,
    pub instanced_meshes: &'a [InstancedMesh<'a>],
}

#[derive(Debug, Copy, Clone)]
pub struct InstancedMesh<'a> {
    pub mesh: &'a Mesh,
    pub instance_buffers: &'a [&'a InstanceBuffer],
}