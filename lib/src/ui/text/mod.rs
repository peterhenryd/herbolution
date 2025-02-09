use crate::engine::gpu::Gpu;
use crate::engine::surface::Surface;
use crate::listener::{InputEvent, Listener};
use glyphon::cosmic_text::CacheKeyFlags;
use glyphon::{Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, Stretch, Style, SwashCache, TextArea, TextAtlas, TextBounds, Viewport, Weight};
use math::color::Color4;
use math::vector::vec2f;
use wgpu::{Device, MultisampleState, Queue, RenderPass};
use winit::dpi::PhysicalSize;

pub struct TextRenderer {
    device: Device,
    queue: Queue,
    inner: glyphon::TextRenderer,
    font_system: FontSystem,
    swash_cache: SwashCache,
    text_atlas: TextAtlas,
    viewport: Viewport,
}

impl TextRenderer {
    pub fn create(gpu: &Gpu, surface: &Surface) -> Self {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_fonts_dir("assets/font");

        let cache = Cache::new(&gpu.device);
        let swash_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&gpu.device, &gpu.queue, &cache, surface.get_format());

        let inner = glyphon::TextRenderer::new(&mut text_atlas, &gpu.device, MultisampleState::default(), None);

        let mut viewport = Viewport::new(&gpu.device, &cache);
        let PhysicalSize { width, height } = surface.get_size();
        viewport.update(&gpu.queue, Resolution { width, height });

        Self { device: gpu.device.clone(), queue: gpu.queue.clone(), inner, font_system, swash_cache, text_atlas, viewport }
    }

    pub fn prepare(&mut self, text_sections: Vec<TextSection>) {
        let mut buffers = vec![];
        for TextSection { content, font_size, line_height, color: c, .. } in &text_sections {
            let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(*font_size, *line_height));
            let attrs = Attrs {
                color_opt: Some(Color::rgba(c.r, c.g, c.b, c.a)),
                family: Family::Name("Edit Undo BRK"),
                stretch: Stretch::Normal,
                style: Style::Normal,
                weight: Weight::NORMAL,
                metadata: 0,
                cache_key_flags: CacheKeyFlags::empty(),
                metrics_opt: None,
            };
            buffer.set_text(&mut self.font_system, &content, attrs, Shaping::Basic);

            buffers.push(buffer);
        }

        let mut text_areas = vec![];
        let Resolution { width, height } = self.viewport.resolution();
        let bounds = TextBounds { left: 0, top: 0, right: width as i32, bottom: height as i32 };
        for (i, TextSection { position, .. }) in text_sections.iter().enumerate() {
            text_areas.push(TextArea {
                buffer: &buffers[i],
                left: position.x,
                top: position.y,
                scale: 1.0,
                bounds,
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            });
        }

        self.inner.prepare(&self.device, &self.queue, &mut self.font_system, &mut self.text_atlas, &self.viewport, text_areas, &mut self.swash_cache)
            .expect("Failed to prepare text for rendering");
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.inner.render(&self.text_atlas, &self.viewport, render_pass).expect("Failed to render text");
    }

    pub fn get_resolution(&self) -> PhysicalSize<u32> {
        let Resolution { width, height } = self.viewport.resolution();
        PhysicalSize::new(width, height)
    }

    pub fn cleanup(&mut self) {
        self.text_atlas.trim();
    }
}

impl Listener for TextRenderer {
    fn on_window_resized(&mut self, PhysicalSize { width, height }: PhysicalSize<u32>) {
        self.viewport.update(&self.queue, Resolution { width, height });
    }

    fn on_input(&mut self, _: &InputEvent) {}
}

pub struct TextSection {
    pub position: vec2f,
    pub content: String,
    pub font_size: f32,
    pub line_height: f32,
    pub color: Color4<u8>,
}