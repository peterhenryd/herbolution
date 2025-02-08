use Material::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Material {
    Air,
    Stone,
    Dirt,
    Grass,
}

impl Material {
    pub fn is_face_culled(self) -> bool {
        match self {
            Air => false,
            _ => true,
        }
    }

    pub fn get_texture_index(self) -> u32 {
        match self {
            Air => unreachable!(),
            Stone => 0,
            Dirt => 1,
            Grass => 2,
        }
    }

    pub fn can_collide(self) -> bool {
        match self {
            Air => false,
            Stone | Dirt | Grass => true,
        }
    }

    pub fn entries() -> impl Iterator<Item = Material> {
        [Stone, Dirt, Grass].into_iter()
    }

    pub fn id(self) -> &'static str {
        match self {
            Air => "air",
            Stone => "stone",
            Dirt => "dirt",
            Grass => "grass",
        }
    }
}