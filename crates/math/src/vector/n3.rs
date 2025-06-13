use crate::vector::{count, vec_type, Vec4};

vec_type! {
    struct Vec3<T> {
        x(X = 1, 0, 0): T,
        y(Y = 0, 1, 0): T,
        z(Z = 0, 0, 1): T,
    }
    linearize(y, z, x)
}

impl<T> Vec3<T> {
    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
    
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Copy + std::ops::Sub<Output = T>,
        T: std::ops::Mul<Output = T>,
    {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

pub type vec3u8 = Vec3<u8>;
pub type vec3u16 = Vec3<u16>;
pub type vec3u = Vec3<u32>;
pub type vec3u64 = Vec3<u64>;
pub type vec3u128 = Vec3<u128>;
pub type vec3usize = Vec3<usize>;
pub type vec3i8 = Vec3<i8>;
pub type vec3i16 = Vec3<i16>;
pub type vec3i = Vec3<i32>;
pub type vec3i64 = Vec3<i64>;
pub type vec3i128 = Vec3<i128>;
pub type vec3isize = Vec3<isize>;
pub type vec3f = Vec3<f32>;
pub type vec3d = Vec3<f64>;