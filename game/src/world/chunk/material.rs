use std::array::IntoIter;

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Material {
    Stone,
    Dirt,
    Grass,
}

impl Material {
    pub fn is_face_culled(self) -> bool {
        true
    }

    pub fn texture_index(self) -> u32 {
        match self {
            Self::Stone => 0,
            Self::Dirt => 1,
            Self::Grass => 2,
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
}

pub trait OptionMaterialExt: Copy {
    fn is_face_culled(self) -> bool;
}

impl OptionMaterialExt for Option<Material> {
    fn is_face_culled(self) -> bool {
        self.map_or(false, Material::is_face_culled)
    }
}