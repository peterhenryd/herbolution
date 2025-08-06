#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    RequestDevice(#[from] wgpu::RequestDeviceError),
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    Surface(#[from] wgpu::SurfaceError),
    Poll(#[from] wgpu::PollError),
    BlasAsync(#[from] wgpu::BlasAsyncError),
    BufferAsync(#[from] wgpu::BufferAsyncError),
}
