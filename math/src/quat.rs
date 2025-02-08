use crate::angle::Rad;
use crate::matrix::{mat4, mat4f};
use crate::vector::{vec4, vec4f};

#[derive(Copy, Clone)]
pub struct Quat(vec4f);

impl Quat {
    pub fn from_euler(x: impl Into<Rad<f32>>, y: impl Into<Rad<f32>>, z: impl Into<Rad<f32>>) -> Self {
        let (sx, cx) = (x.into() / 2.0).0.sin_cos();
        let (sy, cy) = (y.into() / 2.0).0.sin_cos();
        let (sz, cz) = (z.into() / 2.0).0.sin_cos();

        Self(vec4f::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ))
    }

    pub fn into_mat4f(self) -> mat4f {
        let vec4 { x, y, z, w } = self.0;
        mat4::new(
            vec4::new(1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + z * w), 2.0 * (x * z - y * w), 0.0),
            vec4::new(2.0 * (x * y - z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + x * w), 0.0),
            vec4::new(2.0 * (x * z + y * w), 2.0 * (y * z - x * w), 1.0 - 2.0 * (x * x + y * y), 0.0),
            vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl From<Quat> for mat4f {
    fn from(value: Quat) -> Self {
        value.into_mat4f()
    }
}