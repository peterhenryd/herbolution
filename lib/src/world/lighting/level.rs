/// Describes a distinct increment of light that can be applied to a surface. Ranges from 0 to 25.
///
/// The light level is used to determine the alpha by which to multiply the object's color. The 0
/// to 25 range corresponds to the 0 to 255 range of alpha values with increments of roughly 10.
#[derive(Debug, Copy, Clone)]
pub struct LightLevel(u8);

impl LightLevel {
    pub fn as_alpha(&self) -> u8 {
        self.0 * 10 + self.0 / 5
    }
}

pub const LIGHT_LEVELS: [LightLevel; 26] = [
    LightLevel(0),
    LightLevel(1),
    LightLevel(2),
    LightLevel(3),
    LightLevel(4),
    LightLevel(5),
    LightLevel(6),
    LightLevel(7),
    LightLevel(8),
    LightLevel(9),
    LightLevel(10),
    LightLevel(11),
    LightLevel(12),
    LightLevel(13),
    LightLevel(14),
    LightLevel(15),
    LightLevel(16),
    LightLevel(17),
    LightLevel(18),
    LightLevel(19),
    LightLevel(20),
    LightLevel(21),
    LightLevel(22),
    LightLevel(23),
    LightLevel(24),
    LightLevel(25),
];