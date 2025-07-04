use std::ops::{Add, Mul};

use num::Float;
use num::traits::ConstZero;
use serde::{Deserialize, Serialize};

use crate::matrix::Mat4;
use crate::vector::{Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3<T> {
    pub const fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self { x, y, z }
    }

    pub const fn from_array(array: &[T; 9]) -> Self
    where
        T: Copy,
    {
        Self {
            x: Vec3::new(array[0], array[1], array[2]),
            y: Vec3::new(array[3], array[4], array[5]),
            z: Vec3::new(array[6], array[7], array[8]),
        }
    }

    pub fn extend(self, each: Vec3<T>, w: Vec4<T>) -> Mat4<T> {
        Mat4 {
            x: self.x.extend(each.x),
            y: self.y.extend(each.y),
            z: self.z.extend(each.z),
            w,
        }
    }

    pub fn look_to_and_fsu(dir: Vec3<T>, up: Vec3<T>) -> (Mat3<T>, Mat3<T>)
    where
        T: Float,
    {
        let f = dir;
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        let dir = Self::from_array(&[s.x, u.x, -f.x, s.y, u.y, -f.y, s.z, u.z, -f.z]);

        (dir, Mat3::new(f, s, u))
    }

    pub fn look_to(dir: Vec3<T>, up: Vec3<T>) -> Mat3<T>
    where
        T: Float,
    {
        Self::look_to_and_fsu(dir, up).0
    }
}

impl<T> From<Vec3<T>> for Mat3<T>
where
    T: ConstZero + Copy,
{
    fn from(vec: Vec3<T>) -> Self {
        Self {
            x: Vec3::new(vec.x, T::ZERO, T::ZERO),
            y: Vec3::new(T::ZERO, vec.y, T::ZERO),
            z: Vec3::new(T::ZERO, T::ZERO, vec.z),
        }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Vec3<T>> for Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.x * rhs.xxx() + self.y * rhs.yyy() + self.z * rhs.zzz()
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul for Mat3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
