use crate::vector::{Vec2, Vec4};
use bytemuck::{Pod, Zeroable};
use num::traits::real::Real;
use num::traits::{ConstOne, ConstZero};
use num::{NumCast, One, Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v)
    }

    #[inline(always)]
    pub const fn at_index(index: usize, v: T) -> Option<Self>
    where
        T: Copy + PartialEq + ConstZero,
    {
        match index {
            0 => Some(Self { x: v, ..Self::ZERO }),
            1 => Some(Self { y: v, ..Self::ZERO }),
            2 => Some(Self { z: v, ..Self::ZERO }),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn extend(self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline(always)]
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vec3<U> {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    #[inline(always)]
    pub fn zip<U>(self, other: Vec3<U>) -> Vec3<(T, U)> {
        Vec3 {
            x: (self.x, other.x),
            y: (self.y, other.y),
            z: (self.z, other.z),
        }
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(self, other: Self) -> Self
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Sub<Output = T>,
    {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
        T: Real,
    {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Self
    where
        T: Real,
        T: ConstZero
    {
        let length = self.length();
        if length == T::ZERO {
            return Self::ZERO;
        }

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    #[inline(always)]
    pub fn cast<U: NumCast>(self) -> Option<Vec3<U>>
    where
        T: ToPrimitive,
    {
        Some(Vec3 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
            z: NumCast::from(self.z)?,
        })
    }

    #[inline(always)]
    pub fn linearize(&self, n: T) -> T
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.z * n * n + self.x * n + self.y
    }

    #[inline(always)]
    pub fn into_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    pub fn take(&mut self) -> Self
    where
        T: Copy + PartialEq + ConstZero,
    {
        std::mem::replace(self, Self::ZERO)
    }

    pub fn zero_inequalities(&mut self, other: &Vec3<T>)
    where T: PartialEq + ConstZero {
        if self.x != other.x {
            self.x = T::ZERO;
        }
        if self.y != other.y {
            self.y = T::ZERO;
        }
        if self.z != other.z {
            self.z = T::ZERO;
        }
    }
}

impl<A, B> Vec3<(A, B)> {
    pub fn zip_2<C>(self, other: Vec3<C>) -> Vec3<(A, B, C)> {
        Vec3 {
            x: (self.x.0, self.x.1, other.x),
            y: (self.y.0, self.y.1, other.y),
            z: (self.z.0, self.z.1, other.z),
        }
    }
}

// Number trait implementations

impl<T: Copy + PartialEq + Zero> Zero for Vec3<T> {
    #[inline(always)]
    fn zero() -> Self {
        Self::splat(T::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T: Copy + PartialEq + ConstZero> ConstZero for Vec3<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + PartialEq + One> One for Vec3<T> {
    #[inline(always)]
    fn one() -> Self {
        Self::splat(T::one())
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one()
    }
}

impl<T: Copy + PartialEq + ConstOne> ConstOne for Vec3<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: ConstZero + ConstOne> Vec3<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

// False implementation of num::traits::sign::Signed

impl<T: Signed> Vec3<T> {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.x.is_positive() && self.y.is_positive() && self.z.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.x.is_negative() && self.y.is_negative() && self.z.is_negative()
    }
}

// False implementation of num::traits::real::Real
impl<T: Real> Vec3<T> {
    #[inline(always)]
    pub fn min_value() -> Self {
        Self {
            x: T::min_value(),
            y: T::min_value(),
            z: T::min_value(),
        }
    }

    pub fn min_positive_value() -> Self {
        Self {
            x: T::min_positive_value(),
            y: T::min_positive_value(),
            z: T::min_positive_value(),
        }
    }

    pub fn epsilon() -> Self {
        Self {
            x: T::epsilon(),
            y: T::epsilon(),
            z: T::epsilon(),
        }
    }

    pub fn max_value() -> Self {
        Self {
            x: T::max_value(),
            y: T::max_value(),
            z: T::max_value(),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    pub fn trunc(self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
            z: self.z.trunc(),
        }
    }

    pub fn fract(self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
        }
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a.x, b.x),
            y: self.y.mul_add(a.y, b.y),
            z: self.z.mul_add(a.z, b.z),
        }
    }

    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
        }
    }

    pub fn pow(self, n: Self) -> Self {
        Self {
            x: self.x.powf(n.x),
            y: self.y.powf(n.y),
            z: self.z.powf(n.z),
        }
    }

    pub fn powi(self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
            z: self.z.powi(n),
        }
    }

    pub fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
        }
    }

    pub fn sqrt(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
        }
    }

    pub fn exp2(self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
            z: self.z.exp2(),
        }
    }

    pub fn ln(self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
            z: self.z.ln(),
        }
    }

    pub fn log(self, base: Self) -> Self {
        Self {
            x: self.x.log(base.x),
            y: self.y.log(base.y),
            z: self.z.log(base.z),
        }
    }

    pub fn logf(self, base: T) -> Self {
        Self {
            x: self.x.log(base),
            y: self.y.log(base),
            z: self.z.log(base),
        }
    }

    pub fn log2(self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
            z: self.z.log2(),
        }
    }

    pub fn log10(self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
            z: self.z.log10(),
        }
    }

    pub fn to_degrees(self) -> Self {
        Self {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
            z: self.z.to_degrees(),
        }
    }

    pub fn to_radians(self) -> Self {
        Self {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians(),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn maxf(self, other: T) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
            z: self.z.max(other),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn minf(self, other: T) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
            z: self.z.min(other),
        }
    }

    pub fn cbrt(self) -> Self {
        Self {
            x: self.x.cbrt(),
            y: self.y.cbrt(),
            z: self.z.cbrt(),
        }
    }

    pub fn hypot(self, other: Self) -> Self {
        Self {
            x: self.x.hypot(other.x),
            y: self.y.hypot(other.y),
            z: self.z.hypot(other.z),
        }
    }

    pub fn hypotf(self, other: T) -> Self {
        Self {
            x: self.x.hypot(other),
            y: self.y.hypot(other),
            z: self.z.hypot(other),
        }
    }

    pub fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        }
    }

    pub fn tan(self) -> Self {
        Self {
            x: self.x.tan(),
            y: self.y.tan(),
            z: self.z.tan(),
        }
    }

    pub fn asin(self) -> Self {
        Self {
            x: self.x.asin(),
            y: self.y.asin(),
            z: self.z.asin(),
        }
    }

    pub fn acos(self) -> Self {
        Self {
            x: self.x.acos(),
            y: self.y.acos(),
            z: self.z.acos(),
        }
    }

    pub fn atan(self) -> Self {
        Self {
            x: self.x.atan(),
            y: self.y.atan(),
            z: self.z.atan(),
        }
    }

    pub fn atan2(self, other: Self) -> Self {
        Self {
            x: self.x.atan2(other.x),
            y: self.y.atan2(other.y),
            z: self.z.atan2(other.z),
        }
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (x_sin, x_cos) = self.x.sin_cos();
        let (y_sin, y_cos) = self.y.sin_cos();
        let (z_sin, z_cos) = self.z.sin_cos();
        (
            Self {
                x: x_sin,
                y: y_sin,
                z: z_sin,
            },
            Self {
                x: x_cos,
                y: y_cos,
                z: z_cos,
            },
        )
    }

    pub fn exp_m1(self) -> Self {
        Self {
            x: self.x.exp_m1(),
            y: self.y.exp_m1(),
            z: self.z.exp_m1(),
        }
    }

    pub fn ln_1p(self) -> Self {
        Self {
            x: self.x.ln_1p(),
            y: self.y.ln_1p(),
            z: self.z.ln_1p(),
        }
    }

    pub fn sinh(self) -> Self {
        Self {
            x: self.x.sinh(),
            y: self.y.sinh(),
            z: self.z.sinh(),
        }
    }

    pub fn cosh(self) -> Self {
        Self {
            x: self.x.cosh(),
            y: self.y.cosh(),
            z: self.z.cosh(),
        }
    }

    pub fn tanh(self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
            z: self.z.tanh(),
        }
    }

    pub fn asinh(self) -> Self {
        Self {
            x: self.x.asinh(),
            y: self.y.asinh(),
            z: self.z.asinh(),
        }
    }

    pub fn acosh(self) -> Self {
        Self {
            x: self.x.acosh(),
            y: self.y.acosh(),
            z: self.z.acosh(),
        }
    }

    pub fn atanh(self) -> Self {
        Self {
            x: self.x.atanh(),
            y: self.y.atanh(),
            z: self.z.atanh(),
        }
    }
}

// Operator-overloading implementations

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add<T> for Vec3<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Vec3<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for Vec3<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec3<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T: Rem<Output = T>> Rem for Vec3<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }
}

impl<T: Copy + Rem<Output = T>> Rem<T> for Vec3<T> {
    type Output = Self;

    fn rem(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec3<T> {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
        self.z %= other.z;
    }
}

impl<T: Copy + RemAssign> RemAssign<T> for Vec3<T> {
    fn rem_assign(&mut self, other: T) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Not<Output = T>> Not for Vec3<T> {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl<T: Shl<usize, Output = T>> Shl<usize> for Vec3<T> {
    type Output = Self;

    fn shl(self, other: usize) -> Self {
        Self {
            x: self.x << other,
            y: self.y << other,
            z: self.z << other,
        }
    }
}

impl<T: ShlAssign<usize>> ShlAssign<usize> for Vec3<T> {
    fn shl_assign(&mut self, other: usize) {
        self.x <<= other;
        self.y <<= other;
        self.z <<= other;
    }
}

impl<T: Shr<usize, Output = T>> Shr<usize> for Vec3<T> {
    type Output = Self;

    fn shr(self, other: usize) -> Self {
        Self {
            x: self.x >> other,
            y: self.y >> other,
            z: self.z >> other,
        }
    }
}

impl<T: ShrAssign<usize>> ShrAssign<usize> for Vec3<T> {
    fn shr_assign(&mut self, other: usize) {
        self.x >>= other;
        self.y >>= other;
        self.z >>= other;
    }
}

impl<T: Copy + BitAnd<Output = T>> BitAnd<T> for Vec3<T> {
    type Output = Self;

    fn bitand(self, other: T) -> Self {
        Self {
            x: self.x & other,
            y: self.y & other,
            z: self.z & other,
        }
    }
}

impl<T: Copy + BitAndAssign> BitAndAssign<T> for Vec3<T> {
    fn bitand_assign(&mut self, other: T) {
        self.x &= other;
        self.y &= other;
        self.z &= other;
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<'a, T: Copy> IntoIterator for &'a Vec3<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

unsafe impl<T: Zeroable> Zeroable for Vec3<T> {}

unsafe impl<T: Pod> Pod for Vec3<T> {}

// Swizzle functions, added as needed

impl<T> Vec3<T> {
    pub fn xz(&self) -> Vec2<T>
    where
        T: Copy,
    {
        Vec2::new(self.x, self.z)
    }
}
