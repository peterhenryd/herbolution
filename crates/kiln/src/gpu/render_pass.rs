use crate::Gpu;

pub struct RenderPass<'gpu, 'encoder> {
    pub(crate) gpu: &'gpu Gpu,
    pub(crate) inner: wgpu::RenderPass<'encoder>,
}

impl<'gpu, 'encoder> RenderPass<'gpu, 'encoder> {}
