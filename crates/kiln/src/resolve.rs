#[derive(Debug, Copy, Clone)]
pub struct ResolveTextureView<'a> {
    view: &'a wgpu::TextureView,
    resolve_target: Option<&'a wgpu::TextureView>,
}

impl<'a> From<&'a wgpu::TextureView> for ResolveTextureView<'a> {
    fn from(view: &'a wgpu::TextureView) -> Self {
        Self { view, resolve_target: None }
    }
}

pub struct Msaa<'a> {
    src: &'a wgpu::TextureView,
}
