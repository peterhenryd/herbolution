use bytemuck::{Pod, Zeroable};
use num::traits::ConstZero;
use num::{NumCast, ToPrimitive, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::vector::Vec2;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Size2<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2<T> {
    #[inline]
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self { width: value, height: value }
    }

    #[inline]
    pub fn try_cast<U: NumCast>(self) -> Option<Size2<U>>
    where
        T: ToPrimitive,
    {
        Some(Size2 {
            width: NumCast::from(self.width)?,
            height: NumCast::from(self.height)?,
        })
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Size2<U>
    where
        T: ToPrimitive,
    {
        self.try_cast().unwrap()
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

impl<T: ConstZero> Size2<T> {
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T: Zero> Zero for Size2<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl<T: Add<Output = T>> Add for Size2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T: AddAssign> AddAssign for Size2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T: Copy + Add<Output = T>> Add<T> for Size2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.width + rhs, self.height + rhs)
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Size2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.width += rhs;
        self.height += rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Size2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T: SubAssign> SubAssign for Size2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Size2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.width - rhs, self.height - rhs)
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for Size2<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.width -= rhs;
        self.height -= rhs;
    }
}

impl<T: Mul<Output = T>> Mul for Size2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.width * rhs.width, self.height * rhs.height)
    }
}

impl<T: MulAssign> MulAssign for Size2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Size2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Size2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T: Div<Output = T>> Div for Size2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.width / rhs.width, self.height / rhs.height)
    }
}

impl<T: DivAssign> DivAssign for Size2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.width /= rhs.width;
        self.height /= rhs.height;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Size2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Size2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl<T> From<(T, T)> for Size2<T> {
    fn from((width, height): (T, T)) -> Self {
        Self::new(width, height)
    }
}

unsafe impl<T: Zeroable> Zeroable for Size2<T> {}

unsafe impl<T: Pod> Pod for Size2<T> {}
