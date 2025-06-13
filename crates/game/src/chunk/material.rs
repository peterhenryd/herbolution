use std::array::IntoIter;
use serde::{Deserialize, Serialize};
use lib::geo::face::Face;

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Material {
    Stone = 1,
    Dirt = 2,
    Grass = 3,
}

impl Material {
    pub fn is_face_culled(self) -> bool {
        true
    }

    pub fn texture_index(self, face: Face) -> usize {
        match self {
            Self::Stone => 0,
            Self::Dirt => 1,
            Self::Grass => match face {
                Face::Up => 2,
                Face::Down => 1,
                _ => 3,
            },
        }
    }

    pub fn can_collide(self) -> bool {
        true
    }

    pub fn entries() -> IntoIter<Material, 3> {
        [Self::Stone, Self::Dirt, Self::Grass].into_iter()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Stone => "stone",
            Self::Dirt => "dirt",
            Self::Grass => "grass",
        }
    }

    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            1 => Some(Self::Stone),
            2 => Some(Self::Dirt),
            3 => Some(Self::Grass),
            _ => None,
        }
    }
}

pub trait OptionMaterialExt: Copy {
    fn is_face_culled(self) -> bool;
}

impl OptionMaterialExt for Option<Material> {
    fn is_face_culled(self) -> bool {
        self.map_or(false, Material::is_face_culled)
    }
}