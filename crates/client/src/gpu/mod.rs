use crate::gpu::binding::DepthTexture;
use math::color::Rgba;
use math::size::Size2;
use pollster::FutureExt;
use thiserror::Error;
use wgpu::{Color, CommandEncoder, CompositeAlphaMode, CreateSurfaceError, Device, DeviceDescriptor, Features, Instance, Limits, LoadOp, MemoryHints, Operations, PowerPreference, PresentMode, Queue, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterError, RequestAdapterOptions, RequestDeviceError, StoreOp, SurfaceConfiguration, SurfaceError, SurfaceTarget, SurfaceTexture, TextureFormat, TextureUsages, TextureView, Trace};

pub mod binding;
pub mod geometry;
pub mod renderer;
pub mod ext;

pub struct Gpu {
    //adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

pub struct Surface<'w> {
    inner: wgpu::Surface<'w>,
    config: SurfaceConfiguration,
}

impl Surface<'_> {
    pub fn next_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.inner.get_current_texture()
    }

    pub fn set_size(&mut self, gpu: &Gpu, size: Size2<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.inner.configure(&gpu.device, &self.config);
    }

    pub fn size(&self) -> Size2<u32> {
        Size2::new(self.config.width, self.config.height)
    }

    pub fn format(&self) -> TextureFormat {
        self.config.format
    }
}

pub struct GpuOptions {
    pub resolution: Size2<u32>,
    pub power_preference: PowerPreference,
    pub memory_hints: MemoryHints,
    pub present_mode: PresentMode,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum GpuError {
    CreateSurface(#[from] CreateSurfaceError),
    Surface(#[from] SurfaceError),
    RequestAdapter(#[from] RequestAdapterError),
    RequestDevice(#[from] RequestDeviceError),
}

pub fn create<'w>(target: impl Into<SurfaceTarget<'w>>, options: GpuOptions) -> Result<(Gpu, Surface<'w>), GpuError> {
    let instance = Instance::default();
    let surface = instance.create_surface(target)?;
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: options.power_preference,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .block_on()?;
    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: options.memory_hints,
            trace: Trace::Off,
        })
        .block_on()?;

    let surface_config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: TextureFormat::Bgra8UnormSrgb,
        width: options.resolution.width,
        height: options.resolution.height,
        present_mode: options.present_mode,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::PostMultiplied,
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);

    Ok((
        Gpu { /*adapter,*/ device, queue },
        Surface { inner: surface, config: surface_config },
    ))
}

pub struct GpuFrame {
    encoder: CommandEncoder,
    surface_texture: SurfaceTexture,
    surface_texture_view: TextureView,
    depth_texture_view: TextureView,
}

impl GpuFrame {
    pub fn create(gpu: &Gpu, surface: &Surface, depth_texture: &DepthTexture) -> Result<Self, GpuError> {
        let encoder = gpu.device.create_command_encoder(&Default::default());
        let surface_texture = surface.next_texture()?;
        let surface_texture_view = surface_texture.texture.create_view(&Default::default());
        let depth_texture_view = depth_texture.view.clone();

        Ok(Self {
            encoder,
            surface_texture,
            surface_texture_view,
            depth_texture_view,
        })
    }

    pub fn finish(self, gpu: &Gpu) {
        gpu.queue.submit(Some(self.encoder.finish()));
        self.surface_texture.present();
    }

    pub fn start_pass(&mut self, clear_color: Option<Rgba<f64>>) -> RenderPass {
        self.encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.surface_texture_view,
                resolve_target: None,
                ops: Operations {
                    load: clear_color
                        .map(|Rgba { r, g, b, a }| LoadOp::Clear(Color { r, g, b, a }))
                        .unwrap_or(LoadOp::Load),
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_texture_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }
}