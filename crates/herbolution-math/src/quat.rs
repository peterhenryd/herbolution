use crate::vector::vec4f;

pub struct Quat(pub vec4f);

impl Quat {
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let (sx, cx) = (x / 2.0).sin_cos();
        let (sy, cy) = (y / 2.0).sin_cos();
        let (sz, cz) = (z / 2.0).sin_cos();

        Self(vec4f::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ))
    }
}