pub use wgpu::include_wgsl;

pub use wgpu::ShaderModule as Module;
pub use wgpu::ShaderStages as Stage;

#[macro_export]
macro_rules! load_shader {
    ($handle:expr, $path:literal) => {
        ($handle).device().create_shader_module($crate::shader::include_wgsl!($path))
    };
}
