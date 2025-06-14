use crate::vector::{vec_type, Vec3};

vec_type! {
    struct Vec2<T> {
        x(X = 1, 0): T,
        y(Y = 0, 1): T,
    }
    linearize(x, y)
}

impl<T> Vec2<T> {
    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

pub type vec2u8 = Vec2<u8>;
pub type vec2u16 = Vec2<u16>;
pub type vec2u = Vec2<u32>;
pub type vec2u64 = Vec2<u64>;
pub type vec2u128 = Vec2<u128>;
pub type vec2usize = Vec2<usize>;
pub type vec2i8 = Vec2<i8>;
pub type vec2i16 = Vec2<i16>;
pub type vec2i = Vec2<i32>;
pub type vec2i64 = Vec2<i64>;
pub type vec2i128 = Vec2<i128>;
pub type vec2isize = Vec2<isize>;
pub type vec2f = Vec2<f32>;
pub type vec2d = Vec2<f64>;