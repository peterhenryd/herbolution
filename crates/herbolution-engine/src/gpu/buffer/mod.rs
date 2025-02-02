use wgpu::{BufferBindingType, ShaderStages};

pub mod uniform;

pub trait Buffer: AsRef<wgpu::Buffer> {
    fn binding_type(&self) -> BufferBindingType;

    fn visibility(&self) -> ShaderStages;
}
