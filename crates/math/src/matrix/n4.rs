use std::ops::{Add, Mul};

use bytemuck::{Pod, Zeroable};
use num::Float;
use num::traits::{ConstOne, ConstZero};
use serde::{Deserialize, Serialize};

use crate::rotation::Euler;
use crate::vector::{Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T> Mat4<T> {
    pub const fn new(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Self { x, y, z, w }
    }

    pub fn look_to(eye: Vec3<T>, dir: Vec3<T>, up: Vec3<T>) -> Self
    where
        T: Float + ConstZero + ConstOne,
    {
        let f = -dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Mat4::new(
            Vec4::new(s.x, u.x, -f.x, T::ZERO),
            Vec4::new(s.y, u.y, -f.y, T::ZERO),
            Vec4::new(s.z, u.z, -f.z, T::ZERO),
            Vec4::new(-eye.dot(s), -eye.dot(u), eye.dot(f), T::ONE),
        )
    }

    pub fn view_origin(rot: Euler<T>) -> Self
    where
        T: Float + ConstZero + ConstOne,
    {
        let f = -rot.into_view_center().cast().unwrap();
        let s = f.cross(Vec3::Y).normalize();
        let u = s.cross(f);

        Mat4::new(
            Vec4::new(s.x, u.x, -f.x, T::ZERO),
            Vec4::new(s.y, u.y, -f.y, T::ZERO),
            Vec4::new(s.z, u.z, -f.z, T::ZERO),
            Vec4::W,
        )
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        self.x * rhs.xxxx() + self.y * rhs.yyyy() + self.z * rhs.zzzz() + self.w * rhs.wwww()
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul for Mat4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl<T: ConstZero + ConstOne> Mat4<T> {
    pub const IDENTITY: Self = Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);
}

unsafe impl<T: Zeroable> Zeroable for Mat4<T> {}

unsafe impl<T: Pod> Pod for Mat4<T> {}
