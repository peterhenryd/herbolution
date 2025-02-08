use crate::engine::geometry::cube::Faces;
use crate::world::chunk::material::Material;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cube {
    pub(crate) material: Material,
    pub(crate) faces: Faces,
}

impl Cube {
    pub const fn new() -> Self {
        Self {
            material: Material::Air,
            faces: Faces::empty(),
        }
    }
}