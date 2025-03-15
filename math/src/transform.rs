use crate::angle::Rad;
use crate::matrix::{mat4f, Mat4};
use crate::rotation::Euler;
use crate::vector::{vec3f, Vec3, Vec4};
use num::traits::ConstOne;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transform {
    pub position: vec3f,
    pub rotation: Euler<Rad<f32>>,
    pub scale: vec3f,
}

impl Transform {
    pub const fn new(position: vec3f) -> Self {
        Self {
            position,
            rotation: Euler::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn to_view_matrix(&self) -> mat4f {
        let f = -self.rotation.into_view_center().cast().unwrap();
        let s = f.cross(Vec3::Y).normalize();
        let u = s.cross(f);

        Mat4::new(
            Vec4::new(s.x, u.x, -f.x, 0.0),
            Vec4::new(s.y, u.y, -f.y, 0.0),
            Vec4::new(s.z, u.z, -f.z, 0.0),
            Vec4::new(
                -self.position.dot(s),
                -self.position.dot(u),
                self.position.dot(f),
                1.0,
            ),
        )
    }
}
