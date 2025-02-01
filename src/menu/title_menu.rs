use std::time::Duration;
use glyphon::Color;
use wgpu::RenderPass;
use crate::menu::{Menu, MenuType};
use crate::ui::text::TextHandle;
use crate::ui::Ui;

pub struct TitleMenu {
    title_text: Option<TextHandle>,
    r: f32,
    i: f32,
}

impl TitleMenu {
    pub fn new() -> Self {
        Self { title_text: None, r: 0.0, i: 1.0 }
    }
}

impl Menu for TitleMenu {
    fn show(&mut self, ui: &mut Ui) {
        self.title_text = Some(ui.text.build_text(90.)
            .with_text("Herbolution")
            .with_font("Edit Undo BRK")
            .with_color(Color::rgb(255, 255, 255))
            .with_position(120., 100.)
            .finish());

        ui.text.build_text(60.)
            .with_text(format!("Version {}", env!("CARGO_PKG_VERSION")))
            .with_font("Edit Undo BRK")
            .with_color(Color::rgb(255, 255, 255))
            .with_position(120., 180.)
            .finish();
    }

    fn update(&mut self, dt: Duration, ui: &mut Ui) -> Option<MenuType> {
        self.r += self.i * dt.as_millis() as f32 / 8.0;
        if self.r > 255.0 || self.r < 0.0 {
            self.i *= -1.0;
        }

        ui.text.displays.get(self.title_text.unwrap()).edit(&mut ui.text.font_system, |editor| {
            editor.replace_color(Color::rgb(self.r as u8, 255, 255))
        });

        None
    }

    fn hide(&mut self, ui: &mut Ui) {
        ui.text.displays.clear();
    }

    fn render(&self, ui: &Ui, render_pass: &mut RenderPass<'_>) {
        ui.text.render(render_pass);
    }
}