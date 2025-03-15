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
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v, v)
    }

    #[inline(always)]
    pub const fn at_index(index: usize, v: T) -> Option<Self>
    where
        T: Copy + ConstZero,
    {
        match index {
            0 => Some(Self { x: v, ..Self::ZERO }),
            1 => Some(Self { y: v, ..Self::ZERO }),
            2 => Some(Self { z: v, ..Self::ZERO }),
            3 => Some(Self { w: v, ..Self::ZERO }),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vec4<U> {
        Vec4 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
            w: f(self.w),
        }
    }

    #[inline(always)]
    pub fn zip<U>(self, other: Vec4<U>) -> Vec4<(T, U)> {
        Vec4 {
            x: (self.x, other.x),
            y: (self.y, other.y),
            z: (self.z, other.z),
            w: (self.w, other.w),
        }
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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
            z: self.z / length,
            w: self.w / length,
        }
    }

    #[inline(always)]
    pub fn cast<U: NumCast>(self) -> Option<Vec4<U>>
    where
        T: ToPrimitive,
    {
        Some(Vec4 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
            z: NumCast::from(self.z)?,
            w: NumCast::from(self.w)?,
        })
    }

    #[inline(always)]
    pub fn linearize(&self, n: T) -> T
    where
        T: Copy,
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        self.w * n * n * n + self.z * n * n + self.y * n + self.x
    }

    #[inline(always)]
    pub fn into_tuple(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }

    #[inline(always)]
    pub fn into_array(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
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

impl<T: Copy + Zero> Zero for Vec4<T> {
    #[inline(always)]
    fn zero() -> Self {
        Self::splat(T::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}

impl<T: Copy + ConstZero> ConstZero for Vec4<T> {
    const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: Copy + PartialEq + One> One for Vec4<T> {
    #[inline(always)]
    fn one() -> Self {
        Self::splat(T::one())
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one()
    }
}

impl<T: Copy + PartialEq + ConstOne> ConstOne for Vec4<T> {
    const ONE: Self = Self::splat(T::ONE);
}

impl<T: Zero + One> Vec4<T> {
    #[inline(always)]
    pub fn x() -> Self {
        Self::new(T::one(), T::zero(), T::zero(), T::zero())
    }

    #[inline(always)]
    pub fn y() -> Self {
        Self::new(T::zero(), T::one(), T::zero(), T::zero())
    }

    #[inline(always)]
    pub fn z() -> Self {
        Self::new(T::zero(), T::zero(), T::one(), T::zero())
    }

    #[inline(always)]
    pub fn w() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T: ConstZero + ConstOne> Vec4<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

// False implementation of num::traits::sign::Signed

impl<T: Signed> Vec4<T> {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
            w: self.w.signum(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.x.is_positive() && self.y.is_positive() && self.z.is_positive() && self.w.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.x.is_negative() && self.y.is_negative() && self.z.is_negative() && self.w.is_negative()
    }
}

// False implementation of num::traits::real::Real
impl<T: Real> Vec4<T> {
    #[inline(always)]
    pub fn min_value() -> Self {
        Self {
            x: T::min_value(),
            y: T::min_value(),
            z: T::min_value(),
            w: T::min_value(),
        }
    }

    pub fn min_positive_value() -> Self {
        Self {
            x: T::min_positive_value(),
            y: T::min_positive_value(),
            z: T::min_positive_value(),
            w: T::min_positive_value(),
        }
    }

    pub fn epsilon() -> Self {
        Self {
            x: T::epsilon(),
            y: T::epsilon(),
            z: T::epsilon(),
            w: T::epsilon(),
        }
    }

    pub fn max_value() -> Self {
        Self {
            x: T::max_value(),
            y: T::max_value(),
            z: T::max_value(),
            w: T::max_value(),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil(),
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            w: self.w.round(),
        }
    }

    pub fn trunc(self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
            z: self.z.trunc(),
            w: self.w.trunc(),
        }
    }

    pub fn fract(self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
            w: self.w.fract(),
        }
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            x: self.x.mul_add(a.x, b.x),
            y: self.y.mul_add(a.y, b.y),
            z: self.z.mul_add(a.z, b.z),
            w: self.w.mul_add(a.w, b.w),
        }
    }

    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
            w: self.w.recip(),
        }
    }

    pub fn pow(self, n: Self) -> Self {
        Self {
            x: self.x.powf(n.x),
            y: self.y.powf(n.y),
            z: self.z.powf(n.z),
            w: self.w.powf(n.w),
        }
    }

    pub fn powi(self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
            z: self.z.powi(n),
            w: self.w.powi(n),
        }
    }

    pub fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
            w: self.w.powf(n),
        }
    }

    pub fn sqrt(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
            w: self.w.sqrt(),
        }
    }

    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
            w: self.w.exp(),
        }
    }

    pub fn exp2(self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
            z: self.z.exp2(),
            w: self.w.exp2(),
        }
    }

    pub fn ln(self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
            z: self.z.ln(),
            w: self.w.ln(),
        }
    }

    pub fn log(self, base: Self) -> Self {
        Self {
            x: self.x.log(base.x),
            y: self.y.log(base.y),
            z: self.z.log(base.z),
            w: self.w.log(base.w),
        }
    }

    pub fn logf(self, base: T) -> Self {
        Self {
            x: self.x.log(base),
            y: self.y.log(base),
            z: self.z.log(base),
            w: self.w.log(base),
        }
    }

    pub fn log2(self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
            z: self.z.log2(),
            w: self.w.log2(),
        }
    }

    pub fn log10(self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
            z: self.z.log10(),
            w: self.w.log10(),
        }
    }

    pub fn to_degrees(self) -> Self {
        Self {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
            z: self.z.to_degrees(),
            w: self.w.to_degrees(),
        }
    }

    pub fn to_radians(self) -> Self {
        Self {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians(),
            w: self.w.to_radians(),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    pub fn maxf(self, other: T) -> Self {
        Self {
            x: self.x.max(other),
            y: self.y.max(other),
            z: self.z.max(other),
            w: self.w.max(other),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    pub fn minf(self, other: T) -> Self {
        Self {
            x: self.x.min(other),
            y: self.y.min(other),
            z: self.z.min(other),
            w: self.w.min(other),
        }
    }

    pub fn cbrt(self) -> Self {
        Self {
            x: self.x.cbrt(),
            y: self.y.cbrt(),
            z: self.z.cbrt(),
            w: self.w.cbrt(),
        }
    }

    pub fn hypot(self, other: Self) -> Self {
        Self {
            x: self.x.hypot(other.x),
            y: self.y.hypot(other.y),
            z: self.z.hypot(other.z),
            w: self.w.hypot(other.w),
        }
    }

    pub fn hypotf(self, other: T) -> Self {
        Self {
            x: self.x.hypot(other),
            y: self.y.hypot(other),
            z: self.z.hypot(other),
            w: self.w.hypot(other),
        }
    }

    pub fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
            w: self.w.sin(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
            w: self.w.cos(),
        }
    }

    pub fn tan(self) -> Self {
        Self {
            x: self.x.tan(),
            y: self.y.tan(),
            z: self.z.tan(),
            w: self.w.tan(),
        }
    }

    pub fn asin(self) -> Self {
        Self {
            x: self.x.asin(),
            y: self.y.asin(),
            z: self.z.asin(),
            w: self.w.asin(),
        }
    }

    pub fn acos(self) -> Self {
        Self {
            x: self.x.acos(),
            y: self.y.acos(),
            z: self.z.acos(),
            w: self.w.acos(),
        }
    }

    pub fn atan(self) -> Self {
        Self {
            x: self.x.atan(),
            y: self.y.atan(),
            z: self.z.atan(),
            w: self.w.atan(),
        }
    }

    pub fn atan2(self, other: Self) -> Self {
        Self {
            x: self.x.atan2(other.x),
            y: self.y.atan2(other.y),
            z: self.z.atan2(other.z),
            w: self.w.atan2(other.w),
        }
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (x_sin, x_cos) = self.x.sin_cos();
        let (y_sin, y_cos) = self.y.sin_cos();
        let (z_sin, z_cos) = self.z.sin_cos();
        let (w_sin, w_cos) = self.w.sin_cos();
        (
            Self {
                x: x_sin,
                y: y_sin,
                z: z_sin,
                w: w_sin,
            },
            Self {
                x: x_cos,
                y: y_cos,
                z: z_cos,
                w: w_cos,
            },
        )
    }

    pub fn exp_m1(self) -> Self {
        Self {
            x: self.x.exp_m1(),
            y: self.y.exp_m1(),
            z: self.z.exp_m1(),
            w: self.w.exp_m1(),
        }
    }

    pub fn ln_1p(self) -> Self {
        Self {
            x: self.x.ln_1p(),
            y: self.y.ln_1p(),
            z: self.z.ln_1p(),
            w: self.w.ln_1p(),
        }
    }

    pub fn sinh(self) -> Self {
        Self {
            x: self.x.sinh(),
            y: self.y.sinh(),
            z: self.z.sinh(),
            w: self.w.sinh(),
        }
    }

    pub fn cosh(self) -> Self {
        Self {
            x: self.x.cosh(),
            y: self.y.cosh(),
            z: self.z.cosh(),
            w: self.w.cosh(),
        }
    }

    pub fn tanh(self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
            z: self.z.tanh(),
            w: self.w.tanh(),
        }
    }

    pub fn asinh(self) -> Self {
        Self {
            x: self.x.asinh(),
            y: self.y.asinh(),
            z: self.z.asinh(),
            w: self.w.asinh(),
        }
    }

    pub fn acosh(self) -> Self {
        Self {
            x: self.x.acosh(),
            y: self.y.acosh(),
            z: self.z.acosh(),
            w: self.w.acosh(),
        }
    }

    pub fn atanh(self) -> Self {
        Self {
            x: self.x.atanh(),
            y: self.y.atanh(),
            z: self.z.atanh(),
            w: self.w.atanh(),
        }
    }
}

// Operator-overloading implementations

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Copy + Add<Output = T>> Add<T> for Vec4<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec4<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Vec4<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
        self.w += other;
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub<T> for Vec4<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for Vec4<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self.w -= other;
    }
}

impl<T: Mul<Output = T>> Mul for Vec4<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec4<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl<T: Div<Output = T>> Div for Vec4<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec4<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec4<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl<T: Rem<Output = T>> Rem for Vec4<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        }
    }
}

impl<T: Copy + Rem<Output = T>> Rem<T> for Vec4<T> {
    type Output = Self;

    fn rem(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
            w: self.w % other,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec4<T> {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
        self.z %= other.z;
        self.w %= other.w;
    }
}

impl<T: Copy + RemAssign> RemAssign<T> for Vec4<T> {
    fn rem_assign(&mut self, other: T) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
        self.w %= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Not<Output = T>> Not for Vec4<T> {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl<T: Display> Display for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<'a, T: Copy> IntoIterator for &'a Vec4<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

unsafe impl<T: Zeroable> Zeroable for Vec4<T> {}

unsafe impl<T: Pod> Pod for Vec4<T> {}

// Swizzle functions, added as needed

impl<T> Vec4<T> {
    pub fn xxxx(&self) -> Self
    where
        T: Copy,
    {
        Self::new(self.x, self.x, self.x, self.x)
    }

    pub fn yyyy(&self) -> Self
    where
        T: Copy,
    {
        Self::new(self.y, self.y, self.y, self.y)
    }

    pub fn zzzz(&self) -> Self
    where
        T: Copy,
    {
        Self::new(self.z, self.z, self.z, self.z)
    }

    pub fn wwww(&self) -> Self
    where
        T: Copy,
    {
        Self::new(self.w, self.w, self.w, self.w)
    }
}
