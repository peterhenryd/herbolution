use std::fmt::{Debug, Formatter};
use std::hint::unreachable_unchecked;
use std::mem::{replace, MaybeUninit};

pub struct Octree<T> {
    values: Vec<MaybeUninit<T>>,
    nodes: Vec<Node>,
}

#[derive(Copy, Clone)]
enum Node {
    Leaf { index: usize },
    Branch { start_index: usize },
}

impl<T> Octree<T> {
    pub fn new(root: T) -> Self {
        Self {
            values: vec![MaybeUninit::new(root)],
            nodes: vec![Node::Leaf { index: 0 }],
        }
    }

    pub fn traverse(&self) -> Traverse<'_, T> {
        Traverse { tree: self, index: 0 }
    }

    pub fn traverse_mut(&mut self) -> TraverseMut<'_, T> {
        TraverseMut { tree: self, index: 0 }
    }
}

impl<T: Debug> Debug for Octree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Octree ")?;

        debug_node(f, self, 0, false)?;

        Ok(())
    }
}

fn debug_node<T: Debug>(f: &mut Formatter<'_>, tree: &Octree<T>, index: usize, comma: bool) -> std::fmt::Result {
    match tree.nodes[index] {
        Node::Leaf { index } => {
            write!(f, " {:?}", unsafe { tree.values[index].assume_init_ref() })?;
            if comma {
                write!(f, ",")?;
            }
        }
        Node::Branch { start_index } => {
            write!(f, " {{")?;
            for i in 0..8 {
                debug_node(f, tree, start_index + i, i != 7)?;
            }
            write!(f, " }}")?;

            if comma {
                write!(f, ",")?;
            }
        }
    }

    Ok(())
}

pub trait Subdivide<const N: usize> {
    fn subdivide(self) -> [Self; N]
    where
        Self: Sized;
}

pub struct Traverse<'a, T> {
    tree: &'a Octree<T>,
    index: usize,
}

impl<'a, T> Traverse<'a, T> {
    pub fn get(&self) -> Option<&T> {
        match self.tree.nodes[self.index] {
            Node::Leaf { index } => Some(unsafe { self.tree.values[index].assume_init_ref() }),
            _ => None,
        }
    }

    pub fn enter(&self, index: impl Into<NodeIndex>) -> Option<Traverse<'a, T>> {
        match self.tree.nodes[self.index] {
            Node::Leaf { .. } => None,
            Node::Branch { start_index } => Some(Traverse {
                tree: self.tree,
                index: start_index + index.into().0,
            }),
        }
    }
}

impl<'a, T> Traverse<'a, T> {}

pub struct NodeIndex(usize);

impl NodeIndex {
    pub const fn new(value: usize) -> Option<Self> {
        if value < 8 { Some(Self(value)) } else { None }
    }
}

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        Self::new(value).expect("NodeIndex must be in range 0..8")
    }
}

pub struct TraverseMut<'a, T> {
    tree: &'a mut Octree<T>,
    index: usize,
}

impl<'a, T> TraverseMut<'a, T> {
    pub fn get(&self) -> Option<&T> {
        match &self.tree.nodes[self.index] {
            &Node::Leaf { index } => Some(unsafe { self.tree.values[index].assume_init_ref() }),
            _ => None,
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self.tree.nodes[self.index] {
            Node::Leaf { index } => Some(unsafe { self.tree.values[index].assume_init_mut() }),
            _ => None,
        }
    }

    pub fn subdivide_by(&mut self, mut f: impl FnMut(T) -> [T; 8]) -> bool {
        let start_index = self.tree.values.len();
        let leaf_node = &mut self.tree.nodes[self.index];
        if !matches!(leaf_node, Node::Leaf { .. }) {
            return false;
        }

        let old_node = replace(leaf_node, Node::Branch { start_index });

        match old_node {
            Node::Leaf { index } => {
                let old_value = &mut self.tree.values[index];
                let old_value = unsafe { replace(old_value, MaybeUninit::uninit()).assume_init() };
                let values = f(old_value).map(MaybeUninit::new);
                self.tree.values.extend(values);
                self.tree
                    .nodes
                    .extend((start_index..start_index + 8).map(|index| Node::Leaf { index }));
            }
            Node::Branch { .. } => unsafe { unreachable_unchecked() },
        }

        true
    }

    pub fn subdivide(&mut self) -> bool
    where
        T: Subdivide<8>,
    {
        self.subdivide_by(Subdivide::subdivide)
    }

    pub fn enter(&mut self, index: impl Into<NodeIndex>) -> Option<TraverseMut<'_, T>> {
        match &self.tree.nodes[self.index] {
            Node::Leaf { .. } => None,
            &Node::Branch { start_index } => Some(TraverseMut {
                tree: self.tree,
                index: start_index + index.into().0,
            }),
        }
    }
}
