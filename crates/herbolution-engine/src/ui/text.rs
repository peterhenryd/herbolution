use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{MultisampleState, RenderPass};
use winit::dpi::PhysicalSize;
use crate::gpu::Gpu;
use crate::gpu::surface::Surface;

pub struct UiText {
    gpu: Gpu,
    pub(crate) font_system: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: TextRenderer,
    pub displays: TextDisplays,
}

pub struct TextDisplays(Vec<TextDisplay>);

impl TextDisplays {
    pub fn get(&mut self, handle: TextHandle) -> &mut TextDisplay {
        &mut self.0[handle.0]
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Copy, Clone)]
pub struct TextHandle(usize);

impl UiText {
    pub fn create(gpu: &Gpu, surface: &Surface<'_>) -> Self {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_fonts_dir("assets/font");

        let swash_cache = SwashCache::new();
        let cache = Cache::new(gpu.device.as_ref());

        let mut viewport = Viewport::new(&gpu.device, &cache);
        let PhysicalSize { width, height } = surface.get_size();
        viewport.update(&gpu.queue, Resolution { width, height });

        let mut atlas = TextAtlas::new(
            &gpu.device,
            &gpu.queue,
            &cache,
            surface.get_texture_format(),
        );
        let renderer =
            TextRenderer::new(&mut atlas, &gpu.device, MultisampleState::default(), None);

        Self {
            gpu: gpu.clone(),
            font_system,
            swash_cache,
            cache,
            viewport,
            atlas,
            renderer,
            displays: TextDisplays(vec![]),
        }
    }

    pub fn resize(&mut self, PhysicalSize { width, height }: PhysicalSize<u32>) {
        self.viewport
            .update(&self.gpu.queue, Resolution { width, height });
    }

    pub fn prepare(&mut self) {
        let Resolution { width, height } = self.viewport.resolution();
        let bounds = TextBounds {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
        let text_areas = self.displays.0.iter().map(|x| TextArea {
            buffer: &x.buffer,
            left: x.pos[0],
            top: x.pos[1],
            scale: x.scale,
            bounds,
            default_color: Color::rgb(0, 0, 0),
            custom_glyphs: &[],
        });

        self.renderer
            .prepare(
                &self.gpu.device,
                &self.gpu.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .expect("Failed to prepare text renderer");
    }

    pub fn build_text(&mut self, font_size: f32) -> TextBuilder {
        let buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(font_size, font_size * 1.5),
        );

        TextBuilder {
            font_system: &mut self.font_system,
            vec: &mut self.displays.0,
            text: "".to_string(),
            buffer,
            attrs: Attrs::new(),
            shaping: Shaping::Basic,
            scale: 1.0,
            pos: [0.0; 2],
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .expect("Failed to render text");
    }

    pub fn cleanup(&mut self) {
        self.atlas.trim();
    }
}

#[must_use]
pub struct TextBuilder<'a> {
    font_system: &'a mut FontSystem,
    vec: &'a mut Vec<TextDisplay>,
    text: String,
    buffer: Buffer,
    attrs: Attrs<'static>,
    shaping: Shaping,
    scale: f32,
    pos: [f32; 2],
}

impl<'a> TextBuilder<'a> {
    pub fn append(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.pos = [x, y];
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.attrs.color_opt = Some(color);
        self
    }

    pub fn with_font(mut self, name: &'static str) -> Self {
        self.attrs.family = Family::Name(name);
        self
    }

    pub fn finish(mut self) -> TextHandle {
        self.buffer
            .set_text(self.font_system, &self.text, self.attrs, self.shaping);
        self.buffer.shape_until_scroll(self.font_system, false);

        let index = self.vec.len();
        self.vec.push(TextDisplay {
            buffer: self.buffer,
            pos: self.pos,
            scale: self.scale,
            text: self.text,
            attrs: self.attrs,
            shaping: self.shaping,
        });
        TextHandle(index)
    }
}

pub struct TextDisplay {
    buffer: Buffer,
    pos: [f32; 2],
    scale: f32,

    text: String,
    attrs: Attrs<'static>,
    shaping: Shaping,
}

impl TextDisplay {
    pub fn edit(&mut self, font_system: &mut FontSystem, f: impl FnOnce(TextEditor) -> TextEditor) {
        let editor = f(TextEditor {
            text: self.text.clone(),
            attrs: self.attrs,
            shaping: self.shaping,
        });

        self.text = editor.text;
        self.attrs = editor.attrs;
        self.shaping = editor.shaping;

        self.buffer
            .set_text(font_system, &self.text, self.attrs, self.shaping);
    }
}

pub struct TextEditor {
    text: String,
    attrs: Attrs<'static>,
    shaping: Shaping,
}

impl TextEditor {
    pub fn replace_color(mut self, color: Color) -> Self {
        self.attrs.color_opt = Some(color);
        self
    }

    pub fn replace_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn append(mut self, text: &str) -> Self {
        self.text.push_str(text);
        self
    }

    pub fn prepend(mut self, text: &str) -> Self {
        self.text.insert_str(0, text);
        self
    }
}
