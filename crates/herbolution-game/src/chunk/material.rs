#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Material {
    Air = 0,
    Dirt = 1,
    Grass = 2,
    Stone = 3,
}

impl TryFrom<u16> for Material {
    type Error = ();

    fn try_from(i: u16) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Self::Air),
            1 => Ok(Self::Dirt),
            2 => Ok(Self::Grass),
            3 => Ok(Self::Stone),
            _ => Err(()),
        }
    }
}

impl Material {
    pub fn into_texture_index(self) -> u32 {
        self as u32 + 1
    }
}
