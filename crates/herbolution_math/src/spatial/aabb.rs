use crate::vector::Vec3;
use bytemuck::Pod;
use num::traits::real::Real;
use num::traits::{ConstOne, ConstZero};
use num::{Float, Num, NumCast, ToPrimitive};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Aabb<T> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T> Aabb<T> {
    pub const fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }

    pub fn cube(position: Vec3<T>) -> Self
    where
        T: Copy + ConstOne + Add<Output = T>,
    {
        Self {
            min: position,
            max: position + Vec3::ONE,
        }
    }

    pub fn union(slice: &[Self]) -> Self
    where
        T: Copy + Real + ConstZero,
    {
        slice
            .iter()
            .copied()
            .reduce(|lhs, rhs| Self {
                min: lhs.min.min(rhs.min),
                max: lhs.max.max(rhs.max),
            })
            .unwrap_or(Aabb::ZERO)
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

    pub fn add_x(&mut self, x: T)
    where
        T: Copy + AddAssign,
    {
        self.min.x += x;
        self.max.x += x;
    }

    pub fn add_y(&mut self, y: T)
    where
        T: Copy + AddAssign,
    {
        self.min.y += y;
        self.max.y += y;
    }

    pub fn add_z(&mut self, z: T)
    where
        T: Copy + AddAssign,
    {
        self.min.z += z;
        self.max.z += z;
    }

    pub fn intersect(&self, other: &Aabb<T>) -> Self
    where
        T: Copy + Num,
    {
        Self {
            min: self.min - other.min,
            max: self.max - other.max,
        }
    }

    pub fn intersects(&self, other: &Aabb<T>) -> bool
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

    pub fn is_touching(&self, other: &Aabb<T>) -> bool
    where
        T: Copy + Num,
    {
        let intersection = self.intersect(other);
        intersection.width() == T::zero() || intersection.height() == T::zero() || intersection.depth() == T::zero()
    }

    pub fn clip_collision(&self, other: &Aabb<T>, velocity: &mut Vec3<T>)
    where
        T: Copy + ConstZero + Real + PartialOrd + NumCast,
    {
        velocity.x = self.clip_dx_collision(other, velocity.x);
        velocity.y = self.clip_dy_collision(other, velocity.y);
        velocity.z = self.clip_dz_collision(other, velocity.z);
    }

    pub fn clip_dx_collision(&self, other: &Aabb<T>, mut dx: T) -> T
    where
        T: Copy + ConstZero + Num + PartialOrd,
    {
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

    pub fn clip_dy_collision(&self, other: &Aabb<T>, mut dy: T) -> T
    where
        T: Copy + ConstZero + Num + PartialOrd,
    {
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

    pub fn clip_dz_collision(&self, other: &Aabb<T>, mut dz: T) -> T
    where
        T: Copy + ConstZero + Num + PartialOrd,
    {
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
    where
        T: Real,
    {
        (self.min.x + self.max.x) * T::from(0.5).unwrap()
    }

    pub fn y(&self) -> T
    where
        T: Real,
    {
        (self.min.y + self.max.y) * T::from(0.5).unwrap()
    }

    pub fn z(&self) -> T
    where
        T: Real,
    {
        (self.min.z + self.max.z) * T::from(0.5).unwrap()
    }

    pub fn try_cast<U: NumCast>(self) -> Option<Aabb<U>>
    where
        T: ToPrimitive,
    {
        Some(Aabb {
            min: Vec3::new(NumCast::from(self.min.x)?, NumCast::from(self.min.y)?, NumCast::from(self.min.z)?),
            max: Vec3::new(NumCast::from(self.max.x)?, NumCast::from(self.max.y)?, NumCast::from(self.max.z)?),
        })
    }

    pub fn cast<U: NumCast>(self) -> Aabb<U>
    where
        T: ToPrimitive,
    {
        self.try_cast().unwrap()
    }

    pub fn intersect_ray(&self, origin: Vec3<T>, dir: Vec3<T>) -> Vec3<T>
    where
        T: Pod + Float + NumCast,
    {
        let dir = dir.cast();
        let recip_dir = dir.cast().recip();

        let t0 = (self.min - origin) * recip_dir;
        let t1 = (self.max - origin) * recip_dir;

        let t_min_vec = t0.min(t1);
        let t_enter = t_min_vec.largest();

        origin + dir * t_enter
    }
}

impl<T: ConstZero + Copy + PartialEq> Aabb<T> {
    pub const ZERO: Self = Self {
        min: Vec3::ZERO,
        max: Vec3::ZERO,
    };
}

impl<T: Copy + Add<Output = T>> Add<Vec3<T>> for Aabb<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl<T: Copy + AddAssign> AddAssign<Vec3<T>> for Aabb<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.min += rhs;
        self.max += rhs;
    }
}
