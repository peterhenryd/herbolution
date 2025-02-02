pub mod runtime;
pub mod app;
pub mod fs;

pub fn window_attributes(
    title: impl Into<String>,
    resolution: (u32, u32)
) -> winit::window::WindowAttributes {
    use winit::{dpi::LogicalSize, window::WindowAttributes};

    WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::from(resolution))
}