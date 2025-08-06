use std::path::PathBuf;
use std::sync::Arc;

use kiln::{Assets, DirLight};
use lib::fs::Fs;
use lib::util::{DeltaTime, IntervalCounter};
use prims::color::{ColorConsts, Rgb};
use prims::size::size2u;
use prims::vector::Vec3;
use time::Duration;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::app::resource::Resources;
use crate::app::state::{Splash, State};

pub mod handler;
pub mod input;
pub mod resource;
pub mod state;

#[derive(Debug)]
pub struct AppOptions {
    pub data_dir: PathBuf,
    pub resolution: size2u,
}

#[derive(Debug)]
pub struct App {
    fs: Fs,
    input: Input,
    delta_time: DeltaTime,
    fps: IntervalCounter,
    assets: Assets,

    window: Arc<Window>,
    gpu: kiln::Gpu,
    video: kiln::Video,

    core2d: kiln::Core2d,
    core3d: kiln::Core3d,
    resources: Resources,

    state: State,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop, options: AppOptions) -> Self {
        let fs = Fs::new(options.data_dir).expect("Failed to create file system");
        let input = Input::default();
        let delta_time = DeltaTime::new();
        let fps = IntervalCounter::new(Duration::SECOND);
        let assets = Assets::new(fs.path().join("assets"));

        let window_attributes = WindowAttributes::default()
            .with_title("Herbolution")
            .with_inner_size::<LogicalSize<u32>>(options.resolution.to_tuple().into());
        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");
        let window = Arc::new(window);

        let gpu = kiln::Gpu::default();
        let video = kiln::Video::new(&gpu, window.clone(), options.resolution).expect("Failed to create video");

        let core2d = kiln::Core2d::new(&gpu).expect("Failed to create 2d pipeline");
        let mut core3d = kiln::Core3d::new(&gpu).expect("Failed to create 3d pipeline");
        core3d.push_light(
            &gpu,
            DirLight {
                dir: Vec3::Y,
                color: Rgb::WHITE,
                radiance: 1.0,
            },
        );
        core3d
            .write_textures(&gpu, &assets, &["dirt", "grass", "stone"])
            .unwrap();
        let resources = Resources::load_standard(&gpu, fs.path().join("assets"));

        let state = State::Loading(Splash::new());

        Self {
            fs,
            input,
            delta_time,
            fps,
            assets,
            window,
            gpu,
            video,
            core2d,
            core3d,
            resources,
            state,
        }
    }
}

impl App {}
