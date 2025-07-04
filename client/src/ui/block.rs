use lib::color::Rgba;
use taffy::Rect;

use crate::ui::Node;

pub struct BlockNode {
    pub child: Node,
    pub background_color: Rgba<f32>,
    pub border: Option<Border>,
}

pub struct Border {
    pub radius: Rect<f32>,
    pub color: Rgba<f32>,
    pub width: f32,
}
