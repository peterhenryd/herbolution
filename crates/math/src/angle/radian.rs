use crate::angle::{Angle, Deg};
use num::traits::real::Real;
use num::traits::ConstZero;
use num::Zero;
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Rad<T>(pub T);

impl<T> Rad<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: Real> Angle for Rad<T> {
    type Comp = T;

    fn into_deg(self) -> Deg<Self::Comp> {
        Deg(self.0.to_degrees())
    }

    fn into_rad(self) -> Rad<Self::Comp> {
        self
    }
}

impl<T> From<T> for Rad<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Real> From<Deg<T>> for Rad<T> {
    fn from(value: Deg<T>) -> Self {
        Self(value.0.to_radians())
    }
}

impl<T: Add<Output = T>> Add for Rad<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Add<Output = T>> Add<T> for Rad<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<T: AddAssign> AddAssign for Rad<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T: AddAssign> AddAssign<T> for Rad<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Rad<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Sub<Output = T>> Sub<T> for Rad<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<T: SubAssign> SubAssign for Rad<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T: SubAssign> SubAssign<T> for Rad<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

impl<T: Mul<Output = T>> Mul for Rad<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T: Mul<Output = T>> Mul<T> for Rad<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: MulAssign> MulAssign for Rad<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<T: MulAssign> MulAssign<T> for Rad<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T: Div<Output = T>> Div for Rad<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T: Div<Output = T>> Div<T> for Rad<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T: DivAssign> DivAssign for Rad<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<T: DivAssign> DivAssign<T> for Rad<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T: Rem<Output = T>> Rem for Rad<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Rem<Output = T>> Rem<T> for Rad<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % rhs)
    }
}

impl<T: RemAssign> RemAssign for Rad<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl<T: RemAssign> RemAssign<T> for Rad<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.0 %= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Rad<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T: Zero> Zero for Rad<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: ConstZero> ConstZero for Rad<T> {
    const ZERO: Self = Self(T::ZERO);
}

unsafe impl<T: Zeroable> Zeroable for Rad<T> {}

unsafe impl<T: Pod> Pod for Rad<T> {}