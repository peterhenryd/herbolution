macro_rules! shaders {
    ($($ident:ident => $name:literal),+ $(,)?) => {
        $(
        pub fn $ident(gpu: &$crate::gpu::Gpu, label: impl AsRef<str>) -> wgpu::ShaderModule {
            gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label.as_ref()),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(concat!($name, ".wgsl"))))
            })
        }
        )+
    };
}

shaders! {
    pbr => "pbr",
    ui => "ui",
    world => "world",
}
