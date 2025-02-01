pub mod window;
pub mod gpu;
pub mod surface;
pub mod time;
pub mod state;

use crate::app::gpu::Gpu;
use crate::app::state::State;
use crate::app::surface::Surface;
use crate::app::time::DeltaTime;
use crate::app::window::Window;
use crate::ui::Ui;
use lazy_winit::ApplicationInit;
use wgpu::Instance;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::OsError;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};
use crate::menu::MenuType;
use crate::world::World;

pub struct App<'w> {
    window: Window,
    gpu: Gpu,
    surface: Surface<'w>,
    delta_time: DeltaTime,
    state: State,
    ui: Ui,
    world: World,
}

impl App<'_> {
    fn update(&mut self) {
        let dt = self.delta_time.next();

        self.state.update(dt, &mut self.ui, &mut self.world);
        self.state.render(&self.gpu, &self.surface, &mut self.ui, &self.world);
    }
}

fn create_window(event_loop: &ActiveEventLoop) -> Result<Window, OsError> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const RESOLUTION: (u32, u32) = (1920, 1080);

    let attributes = WindowAttributes::default()
        .with_title(format!("Herbolution {VERSION}"))
        .with_inner_size(PhysicalSize::<u32>::from(RESOLUTION));
    Window::new(event_loop, attributes)
}

fn create_gpu_and_surface<'w, 's>(window: &'w Window) -> anyhow::Result<(Gpu, Surface<'s>)> {
    let instance = Instance::default();
    let inner_surface = instance.create_surface(window.get_handle())?;
    let gpu = Gpu::create(&instance, &inner_surface)?;
    let surface = Surface::new(&gpu, inner_surface, window.get_size());

    Ok((gpu, surface))
}

impl ApplicationInit for App<'_> {
    type Args = ();

    fn new(event_loop: &ActiveEventLoop, _: ()) -> Self {
        let window = create_window(event_loop).expect("Failed to create window");
        let (gpu, surface) = create_gpu_and_surface(&window).expect("Failed to create GPU and surface");
        let delta_time = DeltaTime::default();
        let state = State::new(MenuType::Title);
        let ui = Ui::create(&gpu, &surface);
        let world = World::create(&gpu, &surface);

        Self {
            window,
            gpu,
            surface,
            delta_time,
            state,
            ui,
            world,
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        use WindowEvent::*;
        match event {
            Resized(size) => {
                self.surface.resize(size);
                self.ui.resize(size);
            }
            CloseRequested => {
                event_loop.exit();
            }
            RedrawRequested => {
                self.update();
                self.window.request_update();
            }
            _ => {}
        }
    }
}