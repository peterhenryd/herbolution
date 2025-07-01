use taffy::{NodeId, Style, TaffyTree};

use crate::painter::brush::Brush;
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

pub struct Ui {
    tree: TaffyTree<Node>,
    root_id: NodeId,
}

impl Ui {
    pub fn render(&self, brush: &mut Brush) {
        let _ = (brush, &self.tree, self.root_id);
    }
}

pub struct UiBuilder {
    tree: TaffyTree<Node>,
}

impl UiBuilder {
    pub fn new() -> Self {
        Self { tree: TaffyTree::new() }
    }

    pub fn finish(mut self, root: Node) -> Ui {
        let root_id = self
            .tree
            .new_leaf_with_context(Style::default(), root)
            .unwrap();

        Ui { tree: self.tree, root_id }
    }
}
