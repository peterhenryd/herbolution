use kiln::{DrawUi, Text};
use prims::aabb::Aabb2;
use prims::color::Rgba;
use prims::size::{Size2, size2f};
use prims::vector::{Vec2, vec2f};
use winit::event::MouseButton;

use crate::app::input::{ClickEvent, InputFrame};

#[derive(Debug)]
pub struct Ui {
    nodes: Vec<UiNode>,
    events: Vec<UiEvent>,
}

impl Ui {
    pub fn build() -> UiBuilder {
        UiBuilder {
            nodes: vec![],
            layout_direction: LayoutDirection::Row,
            gap: 0.0,
            padding: Size2::ZERO,
        }
    }

    pub fn events(&mut self, input: &InputFrame) -> &[UiEvent] {
        self.events.clear();

        for (index, node) in self.nodes.iter().enumerate() {
            match node {
                UiNode::Button(button) => {
                    for &ClickEvent {
                        button: mouse_button,
                        position,
                    } in &input.click_events
                    {
                        if !button
                            .bounds
                            .contains_rounded(position.cast(), 32.0)
                        {
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

    pub fn render(&self, ui: &mut DrawUi<'_, '_, '_>) {
        for node in &self.nodes {
            match node {
                UiNode::Button(button) => {
                    ui.draw_rect(button.bounds, button.color, 32.0);

                    let text_position = button.bounds.min + (button.padding / 2.0).to_vec2();
                    ui.draw_text(text_position, &button.text);
                }
                UiNode::Text(text) => {
                    ui.draw_text(text.bounds.min, &text.text);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct UiBuilder {
    nodes: Vec<UiNode>,
    layout_direction: LayoutDirection,
    gap: f32,
    padding: size2f,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub padding: size2f,
    pub color: Rgba<f32>,
    pub text: Text,
}

impl UiBuilder {
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

    pub fn add_button(&mut self, button: Button) -> ButtonId {
        let index = self.nodes.len();
        self.nodes.push(UiNode::Button(UiButton {
            bounds: Aabb2::ZERO,
            color: button.color,
            text: button.text,
            padding: button.padding,
        }));
        ButtonId { index }
    }

    pub fn add_text(&mut self, text: Text) -> usize {
        let index = self.nodes.len();
        self.nodes
            .push(UiNode::Text(UiText { bounds: Aabb2::ZERO, text }));
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

    fn compute_layout(&mut self, mut pen: vec2f) {
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if i != 0 {
                pen += self
                    .layout_direction
                    .vectorize(Vec2::splat(self.gap));
            }

            match node {
                UiNode::Button(button) => {
                    button.bounds.set_pos(pen);
                    pen += self
                        .layout_direction
                        .vectorize(button.bounds.size().to_vec2());
                }
                UiNode::Text(text) => {
                    text.bounds.set_pos(pen);
                    pen += self.layout_direction.vectorize(Vec2::ZERO);
                }
            }
        }
    }

    pub fn finish(mut self) -> Ui {
        self.compute_layout(self.padding.to_vec2());

        Ui {
            nodes: self.nodes,
            events: vec![],
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
