use crate::session::Session;
use engine::video::text::TextFrame;
use engine::{video, Engine};
use lib::fs::save::{SaveAttributes, WorldAttributes, WorldDescriptor};
use lib::fs::Fs;
use lib::TrackMut;
use math::color::{Color, Rgba};
use math::size::Size2;
use std::path::PathBuf;
use std::random::random;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;
use engine::video::{r2d, r3d};
use engine::video::r3d::pbr::PbrTexturePaths;

pub mod handler;

pub struct App<'w> {
    engine: Engine<'w>,
    fs: Fs,
    text: TrackMut<TextFrame>,
    session: Option<Session>,
}

impl App<'_> {
    pub fn new(event_loop: &ActiveEventLoop, data_dir: PathBuf) -> Self {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const RESOLUTION: (u32, u32) = (1920, 1080);

        let attributes = WindowAttributes::default()
            .with_title(format!("Herbolution {}", VERSION))
            .with_inner_size(PhysicalSize::<u32>::from(RESOLUTION));
        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window");

        let window = Arc::new(window);
        let engine = Engine::create(window, engine::Options {
            video: video::Options {
                r2d: r2d::Options {
                    texture_paths: vec![
                        "assets/texture/dirt.png".into(),
                    ],
                },
                r3d: r3d::Options { 
                    pbr_texture_paths: PbrTexturePaths::new_suffixed("assets/texture", &[
                        "stone", "dirt", "grass", "grass_side"
                    ], "normal", "specular"),
                },
                clear_color: Rgba::<u8>::from_rgb(117, 255, 250).into()
            },
        });
        let text = TextFrame::default().into();
        let fs = Fs::new(data_dir);

        Self { engine, fs, text, session: None }
    }

    pub fn set_resolution(&mut self, size: Size2<u32>) {
        self.engine.video.set_resolution(size);

        if let Some(session) = &mut self.session {
            session.set_resolution(size);
        }
    }

    pub fn update(&mut self) {
        if self.session.is_none() {
            let save = self.fs.create_or_open_save("default", SaveAttributes {
                title: "Default".to_string(),
                default_world: WorldAttributes { 
                    name: "world".to_string(), 
                    descriptor: WorldDescriptor {
                        title: "Overworld".to_string(), 
                        seed: random(),
                    } 
                },
            }).unwrap();
            self.session = Some(Session::create(save, self.engine.video.resolution(), &mut self.engine));
        }

        let input = self.engine.input.take_frame();
        self.session.as_mut().unwrap().update(&mut self.engine, &input, &mut self.text);
    }

    pub fn render(&mut self) {
        let mut frame = self.engine.video.create_frame();

        if let Some(session) = &mut self.session {
            session.render(&mut frame);
        }

        if let Some(text) = self.text.take_modified() {
            frame.draw_text(text);
        }

        frame.submit();
        self.engine.window.request_redraw();
    }

    pub fn exit(&mut self) {
        if let Some(session) = &mut self.session {
            session.exit();
        }
    }
}