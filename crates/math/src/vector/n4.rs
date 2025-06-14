use crate::vector::vec_type;

vec_type! {
    struct Vec4<T> {
        x(X = 1, 0, 0, 0): T,
        y(Y = 0, 1, 0, 0): T,
        z(Z = 0, 0, 1, 0): T,
        w(W = 0, 0, 0, 1): T,
    }
    linearize(x, y, z, w)
}

impl<T> Vec4<T> {

    pub fn xxxx(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    pub fn yyyy(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    pub fn zzzz(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    pub fn wwww(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w,
        }
    }
}

pub type vec4u8 = Vec4<u8>;
pub type vec4u16 = Vec4<u16>;
pub type vec4u = Vec4<u32>;
pub type vec4u64 = Vec4<u64>;
pub type vec4u128 = Vec4<u128>;
pub type vec4usize = Vec4<usize>;
pub type vec4i8 = Vec4<i8>;
pub type vec4i16 = Vec4<i16>;
pub type vec4i = Vec4<i32>;
pub type vec4i64 = Vec4<i64>;
pub type vec4i128 = Vec4<i128>;
pub type vec4isize = Vec4<isize>;
pub type vec4f = Vec4<f32>;
pub type vec4d = Vec4<f64>;