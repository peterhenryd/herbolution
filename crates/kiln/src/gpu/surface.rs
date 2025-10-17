use kiln_types::size::size2u;
use crate::Gpu;

pub struct Surface {
    inner: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl Surface {
    pub fn new(gpu: &Gpu, target: impl Into<wgpu::SurfaceTarget<'static>>, size: size2u) -> crate::GpuResult<Self> {
        let inner = gpu.instance.create_surface(target)?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
            view_formats: vec![],
        };

        inner.configure(&gpu.device, &config);

        Ok(Self {
            inner,
            config,
        })
    }

    #[inline]
    pub fn inner(&self) -> &wgpu::Surface<'static> {
        &self.inner
    }

    #[inline]
    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }
}

impl Surface {
    pub fn configure(&mut self, gpu: &Gpu, mut f: impl FnMut(&mut wgpu::SurfaceConfiguration)) {
        f(&mut self.config);
        self.inner.configure(&gpu.device, &self.config);
    }
}
