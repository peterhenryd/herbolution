use crate::ui::Node;

pub struct FlexNode {
    pub direction: Direction,
    pub children: Vec<Node>,
}

pub enum Direction {
    Row,
    Column,
}
