use std::fmt::Debug;
use std::ops::{Add, Neg};
use math::num::{Float, Num, NumCast};
use math::num::traits::ConstZero;
use math::num::traits::real::Real;
use math::vector::Vec3;
use crate::geometry::cuboid::face::{Face, Faces};

pub mod face;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    /*

    public float clipXCollide(Cuboid other, float v) {
        if (!(other.y1 > this.y0) || !(other.y0 < this.y1)) {
            return v;
        }
        if (!(other.z1 > this.z0) || !(other.z0 < this.z1)) {
            return v;
        }
        float max;
        if (v > 0.0F && other.x1 <= this.x0) {
            max = this.x0 - other.x1;
            if (max < v) {
                v = max;
            }
        }

        if (v < 0.0F && other.x0 >= this.x1) {
            max = this.x1 - other.x0;
            if (max > v) {
                v = max;
            }
        }

        return v;
    }

    public float clipYCollide(Cuboid other, float v) {
        if (!(other.x1 > this.x0) || !(other.x0 < this.x1)) {
            return v;
        }
        if (!(other.z1 > this.z0) || !(other.z0 < this.z1)) {
            return v;
        }
        float max;
        if (v > 0.0F && other.y1 <= this.y0) {
            max = this.y0 - other.y1;
            if (max < v) {
                v = max;
            }
        }

        if (v < 0.0F && other.y0 >= this.y1) {
            max = this.y1 - other.y0;
            if (max > v) {
                v = max;
            }
        }

        return v;
    }

    public float clipZCollide(Cuboid other, float v) {
        if (!(other.x1 > this.x0) || !(other.x0 < this.x1)) {
            return v;
        }
        if (!(other.y1 > this.y0) || !(other.y0 < this.y1)) {
            return v;
        }
        float max;

        if (v > 0.0F && other.z1 <= this.z0) {
            max = this.z0 - other.z1;
            if (max < v) {
                v = max;
            }
        }

        if (v < 0.0F && other.z0 >= this.z1) {
            max = this.z1 - other.z0;
            if (max > v) {
                v = max;
            }
        }

        return v;
    }
     */

    pub fn clamp_collision_velocity(&self, other: &Cuboid<T>, velocity: &mut Vec3<T>)
    where T: Copy + ConstZero + Num + PartialOrd {
        velocity.x = self.clamp_collision_dx(other, velocity.x);
        velocity.y = self.clamp_collision_dy(other, velocity.y);
        velocity.z = self.clamp_collision_dz(other, velocity.z);
    }

    pub fn clamp_collision_dx(&self, other: &Cuboid<T>, mut dx: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.y >= self.max.y || other.max.y <= self.min.y {
            return dx;
        }

        if other.min.z >= self.max.z || other.max.z <= self.min.z {
            return dx;
        }

        if dx > T::ZERO && other.max.x <= self.min.x {
            let max = self.min.x - other.max.x;
            if max < dx {
                dx = max;
            }
        }

        if dx < T::ZERO && other.min.x >= self.max.x {
            let max = self.max.x - other.min.x;
            if max > dx {
                dx = max;
            }
        }

        dx
    }

    pub fn clamp_collision_dy(&self, other: &Cuboid<T>, mut dy: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.x >= self.max.x || other.max.x <= self.min.x {
            return dy;
        }

        if other.min.z >= self.max.z || other.max.z <= self.min.z {
            return dy;
        }

        if dy > T::ZERO && other.max.y <= self.min.y {
            let max = self.min.y - other.max.y;
            if max < dy {
                dy = max;
            }
        }

        if dy < T::ZERO && other.min.y >= self.max.y {
            let max = self.max.y - other.min.y;
            if max > dy {
                dy = max;
            }
        }

        dy
    }

    pub fn clamp_collision_dz(&self, other: &Cuboid<T>, mut dz: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.x >= self.max.x || other.max.x <= self.min.x {
            return dz;
        }

        if other.min.y >= self.max.y || other.max.y <= self.min.y {
            return dz;
        }

        if dz > T::ZERO && other.max.z <= self.min.z {
            let max = self.min.z - other.max.z;
            if max < dz {
                dz = max;
            }
        }

        if dz < T::ZERO && other.min.z >= self.max.z {
            let max = self.max.z - other.min.z;
            if max > dz {
                dz = max;
            }
        }

        dz
    }
}

impl<T: ConstZero + Copy + PartialEq> Cuboid<T> {
    pub const ZERO: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };
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