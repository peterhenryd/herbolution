use crate::ui::block::BlockNode;
use crate::ui::flex::FlexNode;
use crate::ui::text::TextNode;

mod block;
mod flex;
mod text;

pub enum Node {
    Block(Box<BlockNode>),
    Flex(FlexNode),
    Text(TextNode),
}
