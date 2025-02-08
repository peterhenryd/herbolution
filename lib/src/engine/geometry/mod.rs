use wgpu::VertexBufferLayout;

pub mod cube;
pub mod instance;
pub mod quad;
pub mod vertex;

pub fn get_vertex_instance_buffer_layouts() -> [VertexBufferLayout<'static>; 2] {
    [
        vertex::Vertex::LAYOUT,
        instance::Instance::LAYOUT,
    ]
}