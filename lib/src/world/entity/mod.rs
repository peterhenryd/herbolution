use math::vector::vec3f;

pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub position: vec3f,
}

impl Entity {
    pub fn tick(&mut self) {}
}