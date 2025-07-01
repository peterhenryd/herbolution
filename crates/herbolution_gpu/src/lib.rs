extern crate herbolution_math as math;

pub mod bind_group;
pub mod buffer;
pub mod camera;
pub mod frame;
pub mod handle;
pub mod instance;
pub mod mesh;
pub mod payload;
pub mod pipeline;
pub mod sampler;
pub mod shader;
pub mod surface;
pub mod texture;

use crate::texture::SampleCount;
pub use bind_group::{BindGroup, BindGroupBuilder};
pub use buffer::{Buffer, GrowBuffer};
pub use camera::{Camera, CameraPayload};
pub use frame::Frame;
pub use handle::Handle;
pub use instance::{SetId, Sets};
use math::size::size2u;
pub use mesh::{Index, Mesh, MeshId, Meshes, Vertex};
pub use payload::Payload;
pub use sampler::Sampler;
pub use surface::Surface;
pub use texture::{AtlasError, AtlasTextureCoord, Texture};

pub fn create<'w>(target: impl Into<wgpu::SurfaceTarget<'w>>, resolution: impl Into<size2u>, sample_count: SampleCount) -> (Handle, Surface<'w>) {
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(target)
        .expect("Failed to create surface");

    let handle = Handle::create(&instance, &surface);
    let surface = Surface::new(&handle, surface, resolution.into(), sample_count);

    (handle, surface)
}
