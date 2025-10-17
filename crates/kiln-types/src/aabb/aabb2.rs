use crate::vec::Vec2;
use crate::size::{Size2};
use num::traits::{ConstZero};
use num::{Float, Num, Signed};
use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Aabb2<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T> Aabb2<T> {
    pub fn sized(position: Vec2<T>, size: Size2<T>) -> Self
    where
        T: Copy + Add<Output = T>,
    {
        Self {
            min: position,
            max: position + size.to_vec2(),
        }
    }

    #[inline]
    pub fn contains(&self, point: Vec2<T>) -> bool
    where
        T: Copy + PartialOrd,
    {
        point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y
    }

    #[inline]
    pub fn contains_rounded(&self, point: Vec2<T>, border_radius: T) -> bool
    where
        T: ConstZero + Signed + Float,
    {
        let size = self.max - self.min;
        let extents = size * T::from(0.5).unwrap();

        let centered_point = point - (self.min + extents);
        let radius = border_radius.min(extents.x.min(extents.y));

        let q = centered_point.abs() - extents + radius;
        let signed_distance = q.max(Vec2::ZERO).length() - radius;

        signed_distance <= T::ZERO
    }

    #[inline]
    pub fn size(self) -> Size2<T>
    where
        T: Sub<Output = T>,
    {
        Size2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec2<T>)
    where
        T: Copy + Num,
    {
        self.max = position + self.size().to_vec2();
        self.min = position;
    }
}