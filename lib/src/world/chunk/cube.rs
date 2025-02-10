use crate::engine::geometry::cuboid::{Faces, PerFace};
use crate::world::chunk::material::Material;
use crate::world::lighting::level::{LightLevel, LIGHT_LEVELS};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cube {
    pub(crate) material: Material,
    pub(crate) faces: Faces,
    pub(crate) light_levels: PerFace<LightLevel>,
}

impl Cube {
    pub const fn new() -> Self {
        Self {
            material: Material::Air,
            faces: Faces::empty(),
            light_levels: PerFace::splat(LIGHT_LEVELS[0]),
        }
    }
}