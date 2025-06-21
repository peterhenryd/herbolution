use std::ops::Add;

use num::traits::ConstZero;
use num::{NumCast, ToPrimitive, Zero};

use crate::vec::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ext2<T> {
    pub width: T,
    pub height: T,
}

impl<T> Ext2<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Ext2<U>>
    where
        T: ToPrimitive,
    {
        Some(Ext2 {
            width: NumCast::from(self.width)?,
            height: NumCast::from(self.height)?,
        })
    }

    #[inline]
    pub fn aspect(&self) -> f32
    where
        T: ToPrimitive,
    {
        self.width.to_f32().unwrap() / self.height.to_f32().unwrap()
    }

    pub fn to_vec2(self) -> Vec2<T> {
        Vec2::new(self.width, self.height)
    }
}

impl<T: Zero> Zero for Ext2<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl<T: Add<Output = T>> Add for Ext2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T: ConstZero> ConstZero for Ext2<T> {
    const ZERO: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T> From<(T, T)> for Ext2<T> {
    fn from((width, height): (T, T)) -> Self {
        Self::new(width, height)
    }
}
