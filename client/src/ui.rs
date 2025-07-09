use crate::app::{Render, Update};
use crate::input::ClickEvent;
use crate::video::ui::brush::Text;
use crate::video::ui::Painter;
use lib::aabb::Aabb2;
use lib::color::Rgba;
use lib::size::{size2f, Size2};
use lib::vector::{vec2f, Vec2};
use winit::event::MouseButton;

#[derive(Debug)]
pub struct Ui {
    nodes: Vec<UiNode>,
    events: Vec<UiEvent>,
    background_color: Rgba<f64>,
}

impl Ui {
    pub fn build(painter: &Painter) -> UiBuilder<'_> {
        UiBuilder {
            painter,
            nodes: vec![],
            layout_direction: LayoutDirection::Row,
            gap: 0.0,
            padding: Size2::ZERO,
            background_color: Rgba::TRANSPARENT,
        }
    }

    pub fn events(&mut self, ctx: &mut Update) -> &[UiEvent] {
        self.events.clear();

        for (index, node) in self.nodes.iter().enumerate() {
            match node {
                UiNode::Button(button) => {
                    for &ClickEvent {
                        button: mouse_button,
                        position,
                    } in &ctx.input.click_events
                    {
                        if !button.bounds.contains(position.cast()) {
                            continue;
                        }

                        if mouse_button != MouseButton::Left {
                            continue;
                        }

                        self.events
                            .push(UiEvent::Clicked(ButtonId { index }))
                    }
                }
                _ => {}
            }
        }

        &self.events
    }

    pub fn render(&self, ctx: &mut Render) {
        if self.background_color != Rgba::TRANSPARENT {
            ctx.frame.clear_color(self.background_color);
        }

        let mut brush = ctx.frame.draw_2d();

        for node in &self.nodes {
            match node {
                UiNode::Button(button) => {
                    brush.draw_rect(button.bounds, button.color);

                    let text_position = button.bounds.min + (button.padding / 2.0).to_vec2();
                    brush.draw_text(text_position, &button.text);
                }
                UiNode::Text(text) => {
                    brush.draw_text(text.bounds.min, &text.text);
                }
            }
        }
    }
}

pub struct UiBuilder<'p> {
    painter: &'p Painter,
    nodes: Vec<UiNode>,
    layout_direction: LayoutDirection,
    gap: f32,
    padding: size2f,
    background_color: Rgba<f64>,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub padding: size2f,
    pub color: Rgba<f32>,
    pub text: Text,
}

impl UiBuilder<'_> {
    pub fn with_layout_direction(mut self, direction: LayoutDirection) -> Self {
        self.layout_direction = direction;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_padding(mut self, padding: size2f) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_background_color(mut self, color: Rgba<f64>) -> Self {
        self.background_color = color;
        self
    }

    pub fn add_button(&mut self, button: Button) -> ButtonId {
        let index = self.nodes.len();
        self.nodes.push(UiNode::Button(UiButton {
            bounds: Aabb2::sized(
                Vec2::ZERO,
                self.painter
                    .compute_text_size(&button.text)
                    .expect("Failed to compute text size")
                    + button.padding,
            ),
            color: button.color,
            text: button.text,
            padding: button.padding,
        }));
        ButtonId { index }
    }

    pub fn add_text(&mut self, text: Text) -> usize {
        let index = self.nodes.len();
        self.nodes.push(UiNode::Text(UiText {
            bounds: Aabb2::sized(
                Vec2::ZERO,
                self.painter
                    .compute_text_size(&text)
                    .expect("Failed to compute text size"),
            ),
            text,
        }));
        index
    }

    pub fn with_button(mut self, button: Button, id: &mut Option<ButtonId>) -> Self {
        *id = Some(self.add_button(button));
        self
    }

    pub fn with_text(mut self, text: Text) -> Self {
        self.add_text(text);
        self
    }

    pub fn finish(mut self) -> Ui {
        let mut pen = self.padding.to_vec2();

        for (i, node) in self.nodes.iter_mut().enumerate() {
            if i != 0 {
                pen += self
                    .layout_direction
                    .vectorize(Vec2::splat(self.gap));
            }

            match node {
                UiNode::Button(button) => {
                    button.bounds.set_position(pen);
                    pen += self
                        .layout_direction
                        .vectorize(button.bounds.size().to_vec2());
                }
                UiNode::Text(text) => {
                    text.bounds.set_position(pen);
                    pen += self.layout_direction.vectorize(
                        self.painter
                            .compute_text_size(&text.text)
                            .expect("Failed to compute text size")
                            .to_vec2(),
                    );
                }
            }
        }

        Ui {
            nodes: self.nodes,
            events: vec![],
            background_color: self.background_color,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UiEvent {
    Clicked(ButtonId),
}

#[derive(Debug, Clone)]
struct UiButton {
    bounds: Aabb2<f32>,
    color: Rgba<f32>,
    text: Text,
    padding: size2f,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ButtonId {
    index: usize,
}

#[derive(Debug)]
enum UiNode {
    Button(UiButton),
    Text(UiText),
}

#[derive(Debug, Copy, Clone)]
pub enum LayoutDirection {
    Row,
    Column,
}

impl LayoutDirection {
    fn vectorize(self, value: vec2f) -> vec2f {
        match self {
            LayoutDirection::Row => Vec2::new(value.x, 0.0),
            LayoutDirection::Column => Vec2::new(0.0, value.y),
        }
    }
}

#[derive(Debug)]
pub struct UiText {
    bounds: Aabb2<f32>,
    text: Text,
}
