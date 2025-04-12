use crate::vector::Vec3;
use bytemuck::{Pod, Zeroable};
use num::traits::real::Real;
use num::traits::{ConstOne, ConstZero};
use num::{NumCast, One, Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub,
    SubAssign,
};

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    #[inline(always)]
    pub const fn at_index(index: usize, v: T) -> Option<Self>
    where
        T: Copy + PartialEq + ConstZero,
    {
        match index {
            0 => Some(Self { x: v, ..Self::ZERO }),
            1 => Some(Self { y: v, ..Self::ZERO }),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn extend(self, z: T) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline(always)]
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vec2<U> {
        Vec2 {
            x: f(self.x),
            y: f(self.y),
        }
    }

    #[inline(always)]
    pub fn zip<U>(self, other: Vec2<U>) -> Vec2<(T, U)> {
        Vec2 {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn length_squared(self) -> T
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> T
    where
        T: Copy,
        T: Real,
    {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Self
    where
        T: Copy,
        T: Real,
    {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    #[inline(always)]
    pub fn cast<U: NumCast>(self) -> Option<Vec2<U>>
    where
        T: ToPrimitive,
    {
        Some(Vec2 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
        })
    }

    #[inline(always)]
    pub fn linearize(self, n: T) -> T
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.y * n + self.x
    }

    #[inline(always)]
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    pub fn take(&mut self) -> Self
    where
        T: Copy + PartialEq + ConstZero,
    {
        std::mem::replace(self, Self::ZERO)
    }
}

// Number trait implementations

impl<T: Copy + PartialEq + Zero> Zero for Vec2<T> {
    #[inline(always)]
    fn zero() -> Self {
        Self::splat(T::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Copy + PartialEq + ConstZero> ConstZero for Vec2<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + PartialEq + One> One for Vec2<T> {
    #[inline(always)]
    fn one() -> Self {
        Self::splat(T::one())
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one()
    }
}

impl<T: Copy + PartialEq + ConstOne> ConstOne for Vec2<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Zero + One> Vec2<T> {
    #[inline(always)]
    pub fn x() -> Self {
        Self::new(T::one(), T::zero())
    }

    #[inline(always)]
    pub fn y() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: ConstZero + ConstOne> Vec2<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE);
}

// False implementation of num::traits::sign::Signed

impl<T: Signed> Vec2<T> {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.x.is_positive() && self.y.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.x.is_negative() && self.y.is_negative()
    }
}

// False implementation of num::traits::real::Real
impl<T: Real> Vec2<T> {
    #[inline(always)]
    pub fn min_value() -> Self {
        Self {
            x: T::min_value(),
            y: T::min_value(),
        }
    }

    pub fn min_positive_value() -> Self {
        Self {
            x: T::min_positive_value(),
            y: T::min_positive_value(),
        }
    }

    pub fn epsilon() -> Self {
        Self {
            x: T::epsilon(),
            y: T::epsilon(),
        }
    }

    pub fn max_value() -> Self {
        Self {
            x: T::max_value(),
            y: T::max_value(),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    pub fn trunc(self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
        }
    }

    pub fn fract(self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
        }
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a.x, b.x),
            y: self.y.mul_add(a.y, b.y),
        }
    }

    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
    }

    pub fn pow(self, n: Self) -> Self {
        Self {
            x: self.x.powf(n.x),
            y: self.y.powf(n.y),
        }
    }

    pub fn powi(self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
        }
    }

    pub fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
        }
    }

    pub fn sqrt(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }

    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
        }
    }

    pub fn exp2(self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
        }
    }

    pub fn ln(self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
        }
    }

    pub fn log(self, base: Self) -> Self {
        Self {
            x: self.x.log(base.x),
            y: self.y.log(base.y),
        }
    }

    pub fn logf(self, base: T) -> Self {
        Self {
            x: self.x.log(base),
            y: self.y.log(base),
        }
    }

    pub fn log2(self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
        }
    }

    pub fn log10(self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
        }
    }

    pub fn to_degrees(self) -> Self {
        Self {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
        }
    }

    pub fn to_radians(self) -> Self {
        Self {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn maxf(self, other: T) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn minf(self, other: T) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
        }
    }

    pub fn cbrt(self) -> Self {
        Self {
            x: self.x.cbrt(),
            y: self.y.cbrt(),
        }
    }

    pub fn hypot(self, other: Self) -> Self {
        Self {
            x: self.x.hypot(other.x),
            y: self.y.hypot(other.y),
        }
    }

    pub fn hypotf(self, other: T) -> Self {
        Self {
            x: self.x.hypot(other),
            y: self.y.hypot(other),
        }
    }

    pub fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
        }
    }

    pub fn tan(self) -> Self {
        Self {
            x: self.x.tan(),
            y: self.y.tan(),
        }
    }

    pub fn asin(self) -> Self {
        Self {
            x: self.x.asin(),
            y: self.y.asin(),
        }
    }

    pub fn acos(self) -> Self {
        Self {
            x: self.x.acos(),
            y: self.y.acos(),
        }
    }

    pub fn atan(self) -> Self {
        Self {
            x: self.x.atan(),
            y: self.y.atan(),
        }
    }

    pub fn atan2(self, other: Self) -> Self {
        Self {
            x: self.x.atan2(other.x),
            y: self.y.atan2(other.y),
        }
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (x_sin, x_cos) = self.x.sin_cos();
        let (y_sin, y_cos) = self.y.sin_cos();
        (Self { x: x_sin, y: y_sin }, Self { x: x_cos, y: y_cos })
    }

    pub fn exp_m1(self) -> Self {
        Self {
            x: self.x.exp_m1(),
            y: self.y.exp_m1(),
        }
    }

    pub fn ln_1p(self) -> Self {
        Self {
            x: self.x.ln_1p(),
            y: self.y.ln_1p(),
        }
    }

    pub fn sinh(self) -> Self {
        Self {
            x: self.x.sinh(),
            y: self.y.sinh(),
        }
    }

    pub fn cosh(self) -> Self {
        Self {
            x: self.x.cosh(),
            y: self.y.cosh(),
        }
    }

    pub fn tanh(self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
        }
    }

    pub fn asinh(self) -> Self {
        Self {
            x: self.x.asinh(),
            y: self.y.asinh(),
        }
    }

    pub fn acosh(self) -> Self {
        Self {
            x: self.x.acosh(),
            y: self.y.acosh(),
        }
    }

    pub fn atanh(self) -> Self {
        Self {
            x: self.x.atanh(),
            y: self.y.atanh(),
        }
    }
}

// Operator-overloading implementations

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Vec2<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for Vec2<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec2<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T: Rem<Output = T>> Rem for Vec2<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl<T: Copy + Rem<Output = T>> Rem<T> for Vec2<T> {
    type Output = Self;

    fn rem(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec2<T> {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
    }
}

impl<T: Copy + RemAssign> RemAssign<T> for Vec2<T> {
    fn rem_assign(&mut self, other: T) {
        self.x %= other;
        self.y %= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Not<Output = T>> Not for Vec2<T> {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<'a, T: Copy> IntoIterator for &'a Vec2<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

unsafe impl<T: Zeroable> Zeroable for Vec2<T> {}

unsafe impl<T: Pod> Pod for Vec2<T> {}

// Swizzle functions, added as needed

impl<T> Vec2<T> {}
