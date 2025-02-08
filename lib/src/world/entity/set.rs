use crate::world::entity::Entity;

#[derive(Debug)]
pub struct EntitySet {
    vec: Vec<Entity>,
}

impl EntitySet {
    pub const fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }

    pub fn tick_all(&mut self) {
        for entity in &mut self.vec {
            entity.tick();
        }
    }
}