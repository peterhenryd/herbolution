pub mod text;

use std::time::Duration;
use wgpu::RenderPass;
use crate::engine::Engine;
use crate::ui::text::{TextRenderer, TextSection};

pub struct Ui {
    text_renderer: TextRenderer,
    text_sections: Option<Vec<TextSection>>,
}

impl Ui {
    pub(crate) fn cleanup(&mut self) {
        self.text_renderer.cleanup();
    }
}

impl Ui {
    pub fn create(engine: &Engine) -> Self {
        Self {
            text_renderer: TextRenderer::create(&engine.gpu, &engine.surface),
            text_sections: None,
        }
    }

    pub fn update(&mut self, _: Duration) {
        self.text_renderer.prepare(self.text_sections.take().unwrap_or_default());
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.text_renderer.render(render_pass);
    }

    pub fn push_text(&mut self, text_section: TextSection) {
        if let Some(text_sections) = &mut self.text_sections {
            text_sections.push(text_section);
        } else {
            self.text_sections = Some(vec![text_section]);
        }
    }

    pub fn get_text_renderer(&self) -> &TextRenderer {
        &self.text_renderer
    }
}