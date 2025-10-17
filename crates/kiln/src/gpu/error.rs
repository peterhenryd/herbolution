pub type GpuResult<T> = Result<T, GpuError>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum GpuError {
    Surface(#[from] wgpu::SurfaceError),
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    RequestDevice(#[from] wgpu::RequestDeviceError),
}
