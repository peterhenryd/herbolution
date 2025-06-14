pub type Module = wgpu::ShaderModule;
pub type Stage = wgpu::ShaderStages;

pub use wgpu::include_wgsl;

#[macro_export]
macro_rules! load_shader {
    ($handle:expr, $path:literal) => {
        ($handle).device().create_shader_module($crate::shader::include_wgsl!($path))
    };
}
