use crate::angle::{Angle, Rad};
use num::traits::real::Real;
use num::traits::ConstZero;
use num::Zero;
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub struct Deg<T>(pub T);

impl<T> Deg<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: Real> Angle for Deg<T> {
    type Comp = T;

    fn into_deg(self) -> Deg<Self::Comp> {
        self
    }

    fn into_rad(self) -> Rad<Self::Comp> {
        Rad(self.0.to_radians())
    }
}

impl<T> From<T> for Deg<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Real> From<Rad<T>> for Deg<T> {
    fn from(value: Rad<T>) -> Self {
        Self(value.0.to_degrees())
    }
}

impl<T: Add<Output = T>> Add for Deg<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Add<Output = T>> Add<T> for Deg<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl<T: AddAssign> AddAssign for Deg<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T: AddAssign> AddAssign<T> for Deg<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Deg<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Sub<Output = T>> Sub<T> for Deg<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl<T: SubAssign> SubAssign for Deg<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T: SubAssign> SubAssign<T> for Deg<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

impl<T: Mul<Output = T>> Mul for Deg<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T: Mul<Output = T>> Mul<T> for Deg<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: MulAssign> MulAssign for Deg<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<T: MulAssign> MulAssign<T> for Deg<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T: Div<Output = T>> Div for Deg<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T: Div<Output = T>> Div<T> for Deg<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T: DivAssign> DivAssign for Deg<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<T: DivAssign> DivAssign<T> for Deg<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T: Rem<Output = T>> Rem for Deg<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Rem<Output = T>> Rem<T> for Deg<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % rhs)
    }
}

impl<T: RemAssign> RemAssign for Deg<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl<T: RemAssign> RemAssign<T> for Deg<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.0 %= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Deg<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T: Zero> Zero for Deg<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: ConstZero> ConstZero for Deg<T> {
    const ZERO: Self = Self(T::ZERO);
}
