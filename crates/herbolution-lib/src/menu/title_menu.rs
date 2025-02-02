use crate::engine::Engine;
use crate::fs::save::SaveOptions;
use crate::menu::{Menu, NextState};
use crate::ui::text::TextHandle;
use glyphon::Color;
use rand::random;
use std::time::Duration;
use wgpu::RenderPass;
use winit::keyboard::KeyCode;

pub struct TitleMenu {
    color_text: Option<TextHandle>,
    r: f32,
    i: f32,
}

impl TitleMenu {
    pub fn new() -> Self {
        Self {
            color_text: None,
            r: 0.0,
            i: 1.0,
        }
    }
}

impl Menu for TitleMenu {
    fn show(&mut self, engine: &mut Engine) {
        engine
            .ui
            .text
            .build_text(90.)
            .with_text("Herbolution")
            .with_font("Edit Undo BRK")
            .with_color(Color::rgb(255, 255, 255))
            .with_position(120., 100.)
            .finish();

        engine
            .ui
            .text
            .build_text(60.)
            .with_text(format!("Version {}", env!("CARGO_PKG_VERSION")))
            .with_font("Edit Undo BRK")
            .with_color(Color::rgb(255, 255, 255))
            .with_position(120., 180.)
            .finish();

        self.color_text = Some(
            engine
                .ui
                .text
                .build_text(150.)
                .with_text("Press Enter to Play")
                .with_font("Edit Undo BRK")
                .with_color(Color::rgb(255, 255, 255))
                .with_position(120., 300.)
                .finish(),
        );
    }

    fn update(&mut self, dt: Duration, engine: &mut Engine) -> Option<NextState> {
        self.r += self.i * dt.as_secs_f32() * 0.5;
        if self.r < 0.0 || self.r > 1.0 {
            self.i *= -1.0;
        }

        engine
            .ui
            .text
            .displays
            .get(self.color_text.unwrap())
            .edit(&mut engine.ui.text.font_system, |editor| {
                editor.replace_color(Color::rgb((self.r * 255.0) as u8, 255, 255))
            });

        if engine.input.is_key_down(KeyCode::Enter) {
            let address = engine.fs.saves.create(SaveOptions {
                name: "default".to_string(),
                seed: random(),
            });

            return Some(NextState::Game(address.into()));
        }

        None
    }

    fn hide(&mut self, engine: &mut Engine) {
        engine.ui.text.displays.clear();
    }

    fn render(&self, engine: &Engine, render_pass: &mut RenderPass<'_>) {
        engine.ui.text.render(render_pass);
    }
}
