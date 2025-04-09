use std::fmt::{Debug, Formatter};
use crate::gpu::handle::Handle;
use glyphon::cosmic_text::CacheKeyFlags;
use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, Stretch, Style,
    SwashCache, TextArea, TextAtlas, TextBounds, Viewport, Weight,
};
use pulz_arena::{Arena, Index};
use math::color::Rgba;
use math::size::Size2;
use math::vector::vec2f;
use wgpu::{CompareFunction, DepthStencilState, MultisampleState, RenderPass, TextureFormat};
use winit::dpi::PhysicalSize;

pub struct TextRenderer {
    inner: glyphon::TextRenderer,
    font_system: FontSystem,
    swash_cache: SwashCache,
    text_atlas: TextAtlas,
    viewport: Viewport,
}

impl TextRenderer {
    pub fn create(handle: &Handle, Size2 { width, height }: Size2<u32>, format: TextureFormat) -> Self {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_fonts_dir("assets/font");

        let cache = Cache::new(&handle.device);
        let swash_cache = SwashCache::new();
        let mut text_atlas = TextAtlas::new(&handle.device, &handle.queue, &cache, format);

        let inner = glyphon::TextRenderer::new(
            &mut text_atlas,
            &handle.device,
            MultisampleState::default(),
            Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
        );

        let mut viewport = Viewport::new(&handle.device, &cache);
        viewport.update(&handle.queue, Resolution { width, height });

        Self {
            inner,
            font_system,
            swash_cache,
            text_atlas,
            viewport,
        }
    }

    pub fn prepare(&mut self, handle: &Handle, frame: &TextFrame) {
        let mut buffers = vec![];
        for (_, TextSection {
            content,
            font_size,
            line_height,
            color: c,
            ..
        }) in frame.sections.iter()
        {
            let mut buffer = Buffer::new(
                &mut self.font_system,
                Metrics::new(*font_size, *line_height),
            );
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
        let bounds = TextBounds {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
        for (i, (_, TextSection { pos, .. })) in frame.sections.iter().enumerate() {
            text_areas.push(TextArea {
                buffer: &buffers[i],
                left: pos.x,
                top: pos.y,
                scale: 1.0,
                bounds,
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            });
        }

        self.inner
            .prepare_with_depth(
                &handle.device,
                &handle.queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
                |_| 0.5
            )
            .expect("Failed to prepare text for rendering");
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.inner
            .render(&self.text_atlas, &self.viewport, render_pass)
            .expect("Failed to render text");
    }

    pub fn get_resolution(&self) -> PhysicalSize<u32> {
        let Resolution { width, height } = self.viewport.resolution();
        PhysicalSize::new(width, height)
    }

    pub fn cleanup(&mut self) {
        self.text_atlas.trim();
    }

    pub fn set_size(&mut self, handle: &Handle, Size2 { width, height }: Size2<u32>) {
        self.viewport.update(&handle.queue, Resolution { width, height });
    }
}

#[derive(Debug)]
pub struct TextSection {
    pub pos: vec2f,
    pub content: String,
    pub font_size: f32,
    pub line_height: f32,
    pub color: Rgba<u8>,
}

#[derive(Default)]
pub struct TextFrame {
    pub sections: Arena<TextSection>,
}

impl Debug for TextFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.sections.iter().map(|(_, x)| x)).finish()
    }
}

pub struct TextId(pub(super) Index);