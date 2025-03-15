use std::fmt::Debug;
use std::ops::{Add, Neg};
use math::num::{Float, Num, NumCast};
use math::num::traits::ConstZero;
use math::num::traits::real::Real;
use math::vector::Vec3;
use crate::geometry::cuboid::face::{Face, Faces};

pub mod face;

/// A cuboid in 3D space, also used as an axis-aligned bounding box.
pub struct Cuboid<T> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T> Cuboid<T> {
    pub const fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }

    pub fn from_half(half: Vec3<T>) -> Self
    where
        T: Copy + Num + Neg<Output = T>,
    {
        Self {
            min: -half,
            max: half,
        }
    }

    pub fn union(slice: &[Self]) -> Self
    where
        T: Copy + Real + ConstZero,
    {
        slice.iter()
            .copied()
            .reduce(|lhs, rhs| Self {
                min: lhs.min.min(rhs.min),
                max: lhs.max.max(rhs.max),
            })
            .unwrap_or(Cuboid::ZERO)
    }

    #[inline]
    pub fn width(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn depth(&self) -> T
    where
        T: Copy + Num,
    {
        self.max.z - self.min.z
    }

    #[inline]
    pub fn length_squared(&self) -> T
    where
        T: Copy + Float,
    {
        self.width().powi(2) + self.height().powi(2) + self.depth().powi(2)
    }

    #[inline]
    pub fn length(&self) -> T
    where
        T: Copy + Float,
    {
        self.length_squared().sqrt()
    }

    pub fn center(&self) -> Vec3<T>
    where
        T: Copy + Num + NumCast,
    {
        (self.min + self.max) / T::from(2).unwrap()
    }

    pub fn set_position(&mut self, position: Vec3<T>)
    where
        T: Copy + Num,
    {
        self.max.x = position.x + self.width();
        self.max.y = position.y + self.height();
        self.max.z = position.z + self.depth();
        self.min = position;
    }

    pub fn intersect(&self, other: &Cuboid<T>) -> Self
    where
        T: Copy + Num,
    {
        Self {
            min: self.min - other.min,
            max: self.max - other.max,
        }
    }

    pub fn intersects(&self, other: &Cuboid<T>) -> bool
    where
        T: Copy + PartialOrd,
    {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn is_touching(&self, other: &Cuboid<T>) -> bool
    where
        T: Copy + Num,
    {
        let intersection = self.intersect(other);
        intersection.width() == T::zero() || intersection.height() == T::zero() || intersection.depth() == T::zero()
    }
}

impl<T: ConstZero + Copy + PartialEq> Cuboid<T> {
    pub const ZERO: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };
}

impl<T: Copy> Copy for Cuboid<T> {}

impl<T: Clone> Clone for Cuboid<T> {
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Debug> Debug for Cuboid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cuboid")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl From<Faces> for Cuboid<f32> {
    fn from(faces: Faces) -> Self {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for corner in faces.map(Face::into_corners).flatten() {
            min = min.min(corner);
            max = max.max(corner);
        }

        Self { min, max }
    }
}

impl<T: Copy + Add<Output=T>> Add<Vec3<T>> for Cuboid<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}