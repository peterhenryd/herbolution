use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};
use math::num::{Float, Num, NumCast};
use math::num::traits::ConstZero;
use math::num::traits::real::Real;
use math::vector::Vec3;

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
        T: Copy + Num + NumCast
    {
        (self.min + self.max) / T::from(2).unwrap()
    }

    pub fn set_pos(&mut self, position: Vec3<T>)
    where
        T: Copy + Num,
    {
        self.max.x = position.x + self.width();
        self.max.y = position.y + self.height();
        self.max.z = position.z + self.depth();
        self.min = position;
    }

    pub fn add_x(&mut self, x: T)
    where
        T: Copy + AddAssign
    {
        self.min.x += x;
        self.max.x += x;
    }

    pub fn add_y(&mut self, y: T)
    where
        T: Copy + AddAssign
    {
        self.min.y += y;
        self.max.y += y;
    }

    pub fn add_z(&mut self, z: T)
    where
        T: Copy + AddAssign
    {
        self.min.z += z;
        self.max.z += z;
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

    pub fn clip_collision(&self, other: &Cuboid<T>, velocity: &mut Vec3<T>)
    where T: Copy + ConstZero + Real + PartialOrd + NumCast {
        velocity.x = self.clip_dx_collision(other, velocity.x);
        velocity.y = self.clip_dy_collision(other, velocity.y);
        velocity.z = self.clip_dz_collision(other, velocity.z);
    }

    pub fn clip_dx_collision(&self, other: &Cuboid<T>, mut dx: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.y >= self.max.y || other.max.y <= self.min.y {
            return dx;
        }

        if other.min.z >= self.max.z || other.max.z <= self.min.z {
            return dx;
        }

        if dx > T::ZERO && other.max.x <= self.min.x {
            let clip = self.min.x - other.max.x;
            if clip < dx {
                dx = clip;
            }
        }

        if dx < T::ZERO && other.min.x >= self.max.x {
            let clip = self.max.x - other.min.x;
            if clip > dx {
                dx = clip;
            }
        }

        dx
    }

    pub fn clip_dy_collision(&self, other: &Cuboid<T>, mut dy: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.x >= self.max.x || other.max.x <= self.min.x {
            return dy;
        }

        if other.min.z >= self.max.z || other.max.z <= self.min.z {
            return dy;
        }

        if dy > T::ZERO && other.max.y <= self.min.y {
            let clip = self.min.y - other.max.y;
            if clip < dy {
                dy = clip;
            }
        }

        if dy < T::ZERO && other.min.y >= self.max.y {
            let clip = self.max.y - other.min.y;
            if clip > dy {
                dy = clip;
            }
        }

        dy
    }

    pub fn clip_dz_collision(&self, other: &Cuboid<T>, mut dz: T) -> T
    where T: Copy + ConstZero + Num + PartialOrd {
        if other.min.x >= self.max.x || other.max.x <= self.min.x {
            return dz;
        }

        if other.min.y >= self.max.y || other.max.y <= self.min.y {
            return dz;
        }

        if dz > T::ZERO && other.max.z <= self.min.z {
            let clip = self.min.z - other.max.z;
            if clip < dz {
                dz = clip;
            }
        }

        if dz < T::ZERO && other.min.z >= self.max.z {
            let clip = self.max.z - other.min.z;
            if clip > dz {
                dz = clip;
            }
        }

        dz
    }

    pub fn x(&self) -> T
    where T: Real {
        (self.min.x + self.max.x) * T::from(0.5).unwrap()
    }

    pub fn y(&self) -> T
    where T: Real {
        (self.min.y + self.max.y) * T::from(0.5).unwrap()
    }

    pub fn z(&self) -> T
    where T: Real {
        (self.min.z + self.max.z) * T::from(0.5).unwrap()
    }
}

impl<T: ConstZero + Copy + PartialEq> Cuboid<T> {
    pub const ZERO: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };
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

impl<T: Copy + AddAssign> AddAssign<Vec3<T>> for Cuboid<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.min += rhs;
        self.max += rhs;
    }
}