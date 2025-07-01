extern crate herbolution_math as math;

pub use bind_group::{BindGroup, BindGroupBuilder};
pub use buffer::{Buffer, BufferUsage, GrowBuffer};
pub use camera::{Camera, CameraPayload, View};
pub use frame::{Frame, FrameOptions, Pass};
pub use handle::Handle;
pub use instance::{SetId, Sets};
use math::size::size2u;
pub use mesh::{Index, Mesh, MeshId, Meshes, Vertex};
pub use payload::Payload;
pub use pipeline::map::{PipelineMap, PipelineType};
pub use pipeline::{vertex_attr_array, CullMode, PipelineOptions, VertexBufferLayout, VertexStepMode};
pub use sampler::{Filter, Sampler, SamplerOptions};
pub use shader::{CompileShaderError, CompiledShaders, ShaderModule, ShaderSources, ShaderStage};
pub use surface::{Surface, SurfaceTarget, SurfaceTexture};
pub use texture::{AtlasError, AtlasTextureCoord, SampleCount, Texture, TextureView};

mod bind_group;
mod buffer;
mod camera;
mod frame;
mod handle;
mod instance;
mod mesh;
mod payload;
mod pipeline;
mod sampler;
mod shader;
mod surface;
mod texture;

pub fn create<'w>(target: impl Into<wgpu::SurfaceTarget<'w>>, resolution: impl Into<size2u>, sample_count: SampleCount) -> (Handle, Surface<'w>) {
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(target)
        .expect("Failed to create surface");

    let handle = Handle::create(&instance, &surface);
    let surface = Surface::new(&handle, surface, resolution.into(), sample_count);

    (handle, surface)
}
