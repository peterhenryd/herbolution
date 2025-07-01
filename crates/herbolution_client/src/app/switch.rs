use std::sync::Arc;

use engine::{painter, sculptor, video, Engine};
use gpu::SampleCount;
use math::color::{Color, Rgba};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

/// A switch that allows the application to be resumed or suspended by the operating system gracefully.
pub enum Switch<'w> {
    Resumed { window: Arc<Window>, engine: Engine<'w> },
    Suspended(Option<Arc<Window>>),
}

const RESOLUTION: (u32, u32) = (1920, 1080);

impl Switch<'_> {
    pub(super) fn resume(&mut self, event_loop: &ActiveEventLoop) {
        let cached_window;
        match self {
            Switch::Resumed { .. } => return,
            Switch::Suspended(window) => cached_window = window.take(),
        }

        let window = cached_window.unwrap_or_else(|| create_window(event_loop));
        let engine = Engine::create(
            window.clone(),
            engine::Options {
                video: video::Options {
                    resolution: RESOLUTION.into(),
                    clear_color: Rgba::<u8>::from_rgb(117, 255, 250).into(),
                    painter: painter::Options {
                        texture_paths: vec!["assets/texture/dirt.png".into()],
                    },
                    sculptor: sculptor::Options {},
                    sample_count: SampleCount::Multi,
                },
            },
        );

        *self = Switch::Resumed { window, engine };
    }

    pub(super) fn suspend(&mut self) {
        let cached_window;
        if let Self::Resumed { window, .. } = self {
            cached_window = Some(Arc::clone(window));
        } else {
            cached_window = None;
        }

        *self = Switch::Suspended(cached_window);
    }
}

fn create_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const RESOLUTION: (u32, u32) = (1920, 1080);

    let attributes = WindowAttributes::default()
        .with_title(format!("Herbolution {}", VERSION))
        .with_inner_size::<PhysicalSize<u32>>(RESOLUTION.into());
    let window = event_loop
        .create_window(attributes)
        .expect("Failed to create window");

    Arc::new(window)
}
