#![allow(non_camel_case_types)]

use bytemuck::{cast_ref, Pod, Zeroable};
use num::traits::{ConstOne, ConstZero, Euclid, Signed};
use num::{Float, NumCast, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use static_assertions::assert_eq_size;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU16;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub type vec2u8 = Vec2<u8>;
pub type vec2u16 = Vec2<u16>;
pub type vec2u = Vec2<u32>;
pub type vec2u64 = Vec2<u64>;
pub type vec2u128 = Vec2<u128>;
pub type vec2usize = Vec2<usize>;
pub type vec2i8 = Vec2<i8>;
pub type vec2i16 = Vec2<i16>;
pub type vec2i = Vec2<i32>;
pub type vec2i64 = Vec2<i64>;
pub type vec2i128 = Vec2<i128>;
pub type vec2isize = Vec2<isize>;
pub type vec2f = Vec2<f32>;
pub type vec2d = Vec2<f64>;

pub type vec3u8 = Vec3<u8>;
pub type vec3u16 = Vec3<u16>;
pub type vec3u = Vec3<u32>;
pub type vec3u64 = Vec3<u64>;
pub type vec3u128 = Vec3<u128>;
pub type vec3usize = Vec3<usize>;
pub type vec3i8 = Vec3<i8>;
pub type vec3i16 = Vec3<i16>;
pub type vec3i = Vec3<i32>;
pub type vec3i64 = Vec3<i64>;
pub type vec3i128 = Vec3<i128>;
pub type vec3isize = Vec3<isize>;
pub type vec3f = Vec3<f32>;
pub type vec3d = Vec3<f64>;

pub type vec4u8 = Vec4<u8>;
pub type vec4u16 = Vec4<u16>;
pub type vec4u = Vec4<u32>;
pub type vec4u64 = Vec4<u64>;
pub type vec4u128 = Vec4<u128>;
pub type vec4usize = Vec4<usize>;
pub type vec4i8 = Vec4<i8>;
pub type vec4i16 = Vec4<i16>;
pub type vec4i = Vec4<i32>;
pub type vec4i64 = Vec4<i64>;
pub type vec4i128 = Vec4<i128>;
pub type vec4isize = Vec4<isize>;
pub type vec4f = Vec4<f32>;
pub type vec4d = Vec4<f64>;

macro_rules! vector {
    (@count) => { 0 };
    (@count $first:ident $($rest:ident)*) => {
        1 + vector!(@count $($rest)*)
    };
    (
        struct $name:ident<$t:ident> {
        $(
            $field:ident($constant:ident = $($literal:literal),+): $ft:ident
        ),+ $(,)?
        }
        linearize($($order:ident),+)
    ) => {
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
        pub struct $name<$t> {
            $(pub $field: $ft),+
        }

        impl<$t> $name<$t> {
            #[inline]
            pub const fn new($($field: $t),+) -> Self {
                Self { $($field),+ }
            }

            #[inline]
            pub const fn splat(v: $t) -> Self
            where
                $t: Copy,
            {
                Self { $($field: v),+ }
            }

            #[inline]
            pub fn by_index(mut f: impl FnMut(usize) -> T) -> Self {
                let n = 0;
            $(
                let $field = f(n);
                let n = n + 1;
            )+
                let _ = n;

                Self {
                    $($field),+
                }
            }

            #[inline]
            pub fn zero() -> Self
            where
                $t: Zero
            {
                Self {
                    $($field: Zero::zero()),+
                }
            }

            #[inline]
            pub fn map<U>(self, mut f: impl FnMut($t) -> U) -> $name<U> {
                $name {
                    $($field: f(self.$field)),+
                }
            }

            #[inline]
            pub fn dot(self, other: Self) -> $t
            where
                $t: Mul<Output = $t>,
                $t: Add<Output = $t>,
                $t: Zero
            {
                $(
                    self.$field * other.$field +
                )+ Zero::zero()
            }

            #[inline]
            pub fn length_squared(self) -> $t
            where
                $t: Copy,
                $t: Mul<Output = $t>,
                $t: Add<Output = $t>,
                $t: Zero,
            {
                self.dot(self)
            }

            #[inline]
            pub fn length(self) -> T
            where
                $t: Float,
            {
                self.length_squared().sqrt()
            }

            #[inline]
            pub fn normalize(self) -> Self
            where
                $t: Float,
            {
                let length = self.length();
                if length.is_zero() {
                    return Self::zero();
                }
                Self {
                    $($field: self.$field / length),+
                }
            }

            #[inline]
            pub fn try_cast<U: NumCast>(self) -> Option<$name<U>>
            where
                $t: ToPrimitive,
            {
                Some($name {
                    $(
                        $field: U::from(self.$field)?,
                    )+
                })
            }

            #[inline]
            pub fn cast<U: NumCast>(self) -> $name<U>
            where
                $t: ToPrimitive,
            {
                self.try_cast().unwrap()
            }

            #[inline]
            pub fn to_tuple(self) -> ($($ft),+) {
                ($(self.$field),+)
            }

            #[inline]
            pub fn to_array(self) -> [$t; vector!(@count $($field)+)] {
                [$(self.$field),+]
            }

            #[inline]
            pub fn as_array(&self) -> &[$t; vector!(@count $($field)+)]
            where
                T: Pod,
            {
                cast_ref(self)
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t]
            where
                $t: Pod,
            {
                self.as_array()
            }

            #[inline]
            pub fn smallest(&self) -> $t
            where
                $t: PartialOrd + Pod,
            {
                let slice = self.as_slice();
                let mut smallest = slice[0];

                for &item in &slice[1..] {
                    if item < smallest {
                        smallest = item;
                    }
                }

                smallest
            }

            #[inline]
            pub fn largest(&self) -> $t
            where
                $t: PartialOrd + Pod,
            {
                let slice = self.as_slice();
                let mut largest = slice[0];

                for &item in &slice[1..] {
                    if item > largest {
                        largest = item;
                    }
                }

                largest
            }

            #[inline]
            pub fn take(&mut self) -> Self
            where
                $t: ConstZero,
            {
                std::mem::replace(self, Self::ZERO)
            }

            #[inline]
            pub fn linearize(self, length: $t) -> $t
            where
                $t: Copy,
                $t: MulAssign,
                $t: AddAssign,
                $t: ConstZero,
                $t: ConstOne,
            {
                let n = T::ONE;
                let x = T::ZERO;
            $(
                let x = x + self.$order * n;
                let n = n * length;
            )+
                let _ = n;
                x
            }

            #[inline]
            pub fn abs(self) -> Self
            where
                $t: Signed,
            {
                Self {
                    $($field: self.$field.abs()),+
                }
            }

            #[inline]
            pub fn signum(self) -> Self
            where
                $t: Signed,
            {
                Self {
                    $($field: self.$field.signum()),+
                }
            }

            #[inline]
            pub fn is_positive(&self) -> bool
            where
                $t: Signed,
            {
                $(self.$field.is_positive())&&+
            }

            #[inline]
            pub fn is_negative(&self) -> bool
            where
                $t: Signed,
            {
                $(self.$field.is_negative())&&+
            }

            #[inline]
            pub fn floor(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.floor()),+
                }
            }

            #[inline]
            pub fn ceil(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.ceil()),+
                }
            }

            #[inline]
            pub fn round(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.round()),+
                }
            }

            #[inline]
            pub fn trunc(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.trunc()),+
                }
            }

            #[inline]
            pub fn fract(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.fract()),+
                }
            }

            #[inline]
            pub fn recip(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.recip()),+
                }
            }

            #[inline]
            pub fn pow(self, n: Self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.powf(n.$field)),+
                }
            }

            #[inline]
            pub fn powf(self, n: $t) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.powf(n)),+
                }
            }

            #[inline]
            pub fn powi(self, n: i32) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.powi(n)),+
                }
            }

            #[inline]
            pub fn sqrt(self) -> Self
            where
                $t: Float,
            {
                Self {
                    $($field: self.$field.sqrt()),+
                }
            }

            #[inline]
            pub fn max(self, other: Self) -> Self
            where
                $t: PartialOrd
            {
                Self {
                    $($field: if self.$field > other.$field { self.$field } else { other.$field }),+
                }
            }

            #[inline]
            pub fn max_each(self, other: $t) -> Self
            where
                $t: Copy + PartialOrd
            {
                Self {
                    $($field: if self.$field > other { self.$field } else { other }),+
                }
            }

            #[inline]
            pub fn min(self, other: Self) -> Self
            where
                $t: PartialOrd
            {
                Self {
                    $($field: if self.$field < other.$field { self.$field } else { other.$field }),+
                }
            }

            #[inline]
            pub fn min_each(self, other: $t) -> Self
            where
                $t: Copy + PartialOrd
            {
                Self {
                    $($field: if self.$field < other { self.$field } else { other }),+
                }
            }

            #[inline]
            pub fn div_euclid(self, other: Self) -> Self
            where
                $t: Euclid
            {
                Self {
                    $($field: self.$field.div_euclid(&other.$field)),+
                }
            }

            #[inline]
            pub fn div_euclid_each(self, other: $t) -> Self
            where
                $t: Copy + Euclid
            {
                Self {
                    $($field: self.$field.div_euclid(&other)),+
                }
            }

            #[inline]
            pub fn rem_euclid(self, other: Self) -> Self
            where
                $t: Euclid
            {
                Self {
                    $($field: self.$field.rem_euclid(&other.$field)),+
                }
            }

            #[inline]
            pub fn rem_euclid_each(self, other: $t) -> Self
            where
                $t: Copy + Euclid
            {
                Self {
                    $($field: self.$field.rem_euclid(&other)),+
                }
            }
        }

        impl<$t: ConstZero> $name<$t> {
            pub const ZERO: Self = Self {
                $($field: T::ZERO),+
            };
        }

        impl<$t: ConstOne> $name<$t> {
            pub const ONE: Self = Self {
                $($field: T::ONE),+
            };
        }

        impl<$t: ConstZero + ConstOne> $name<$t> {
            $(
                pub const $constant: Self = Self::new($(if $literal == 1 { T::ONE } else { T::ZERO }),+);
            )+
        }

        impl<$t> From<($($ft),+)> for $name<$t> {
            #[inline]
            fn from(($($field),+): ($($ft),+)) -> Self {
                Self {
                    $($field),+
                }
            }
        }

        impl<$t> From<[$t; vector!(@count $($field)+)]> for $name<$t> {
            #[inline]
            fn from([$($field),+]: [$t; vector!(@count $($field)+)]) -> Self {
                Self {
                    $($field),+
                }
            }
        }


        impl<$t> Into<($($ft),+)> for $name<$t> {
            #[inline]
            fn into(self) -> ($($ft),+) {
                ($(self.$field),+)
            }
        }

        impl<$t> Into<[$t; vector!(@count $($field)+)]> for $name<$t> {
            #[inline]
            fn into(self) -> [$t; vector!(@count $($field)+)] {
                [$(self.$field),+]
            }
        }

        impl<$t: Add<Output = $t>> Add for $name<$t> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),+
                }
            }
        }

        impl<$t: Add<Output = $t> + Copy> Add<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn add(self, other: $t) -> Self {
                Self {
                    $($field: self.$field + other),+
                }
            }
        }

        impl<$t: AddAssign> AddAssign for $name<$t> {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field;)+
            }
        }

        impl<$t: AddAssign + Copy> AddAssign<$t> for $name<$t> {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                $(self.$field += other;)+
            }
        }

        impl<$t: Sub<Output = $t>> Sub for $name<$t> {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),+
                }
            }
        }

        impl<$t: Sub<Output = $t> + Copy> Sub<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn sub(self, other: $t) -> Self {
                Self {
                    $($field: self.$field - other),+
                }
            }
        }

        impl<$t: SubAssign> SubAssign for $name<$t> {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field;)+
            }
        }

        impl<$t: SubAssign + Copy> SubAssign<$t> for $name<$t> {
            #[inline]
            fn sub_assign(&mut self, other: $t) {
                $(self.$field -= other;)+
            }
        }

        impl<$t: Mul<Output = $t>> Mul for $name<$t> {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self {
                Self {
                    $($field: self.$field * other.$field),+
                }
            }
        }

        impl<$t: Mul<Output = $t> + Copy> Mul<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn mul(self, other: $t) -> Self {
                Self {
                    $($field: self.$field * other),+
                }
            }
        }

        impl<$t: MulAssign> MulAssign for $name<$t> {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                $(self.$field *= other.$field;)+
            }
        }

        impl<$t: MulAssign + Copy> MulAssign<$t> for $name<$t> {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                $(self.$field *= other;)+
            }
        }

        impl<$t: Div<Output = $t>> Div for $name<$t> {
            type Output = Self;

            #[inline]
            fn div(self, other: Self) -> Self {
                Self {
                    $($field: self.$field / other.$field),+
                }
            }
        }

        impl<$t: Div<Output = $t> + Copy> Div<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn div(self, other: $t) -> Self {
                Self {
                    $($field: self.$field / other),+
                }
            }
        }

        impl<$t: DivAssign> DivAssign for $name<$t> {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                $(self.$field /= other.$field;)+
            }
        }

        impl<$t: DivAssign + Copy> DivAssign<$t> for $name<$t> {
            fn div_assign(&mut self, other: $t) {
                $(self.$field /= other;)+
            }
        }

        impl<$t: Rem<Output = $t>> Rem for $name<$t> {
            type Output = Self;

            #[inline]
            fn rem(self, other: Self) -> Self {
                Self {
                    $($field: self.$field % other.$field),+
                }
            }
        }

        impl<$t: Rem<Output = $t> + Copy> Rem<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn rem(self, other: $t) -> Self {
                Self {
                    $($field: self.$field % other),+
                }
            }
        }

        impl<$t: RemAssign> RemAssign for $name<$t> {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                $(self.$field %= other.$field;)+
            }
        }

        impl<$t: RemAssign + Copy> RemAssign<$t> for $name<$t> {
            #[inline]
            fn rem_assign(&mut self, other: $t) {
                $(self.$field %= other;)+
            }
        }

        impl<$t: BitAnd<Output = $t>> BitAnd for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self {
                    $($field: self.$field & other.$field),+
                }
            }
        }

        impl<$t: BitAnd<Output = $t> + Copy> BitAnd<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitand(self, other: $t) -> Self {
                Self {
                    $($field: self.$field & other),+
                }
            }
        }

        impl<$t: BitAndAssign> BitAndAssign for $name<$t> {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                $(self.$field &= other.$field;)+
            }
        }

        impl<$t: BitAndAssign + Copy> BitAndAssign<$t> for $name<$t> {
            #[inline]
            fn bitand_assign(&mut self, other: $t) {
                $(self.$field &= other;)+
            }
        }

        impl<$t: BitOr<Output = $t>> BitOr for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field | other.$field),+
                }
            }
        }

        impl<$t: BitOr<Output = $t> + Copy> BitOr<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field | other),+
                }
            }
        }

        impl<$t: BitOrAssign> BitOrAssign for $name<$t> {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                $(self.$field |= other.$field;)+
            }
        }

        impl<$t: BitOrAssign + Copy> BitOrAssign<$t> for $name<$t> {
            #[inline]
            fn bitor_assign(&mut self, other: $t) {
                $(self.$field |= other;)+
            }
        }

        impl<$t: BitXor<Output = $t>> BitXor for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitxor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field ^ other.$field),+
                }
            }
        }

        impl<$t: BitXor<Output = $t> + Copy> BitXor<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitxor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field ^ other),+
                }
            }
        }

        impl<$t: BitXorAssign> BitXorAssign for $name<$t> {
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                $(self.$field ^= other.$field;)+
            }
        }

        impl<$t: BitXorAssign + Copy> BitXorAssign<$t> for $name<$t> {
            #[inline]
            fn bitxor_assign(&mut self, other: $t) {
                $(self.$field ^= other;)+
            }
        }

        impl<$t: Shl<Output = $t>> Shl for $name<$t> {
            type Output = Self;

            #[inline]
            fn shl(self, other: Self) -> Self {
                Self {
                    $($field: self.$field << other.$field),+
                }
            }
        }

        impl<$t: Shl<Output = $t> + Copy> Shl<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn shl(self, other: $t) -> Self {
                Self {
                    $($field: self.$field << other),+
                }
            }
        }

        impl<$t: ShlAssign> ShlAssign for $name<$t> {
            #[inline]
            fn shl_assign(&mut self, other: Self) {
                $(self.$field <<= other.$field;)+
            }
        }

        impl<$t: ShlAssign + Copy> ShlAssign<$t> for $name<$t> {
            #[inline]
            fn shl_assign(&mut self, other: $t) {
                $(self.$field <<= other;)+
            }
        }

        impl<$t: Shr<Output = $t>> Shr for $name<$t> {
            type Output = Self;

            #[inline]
            fn shr(self, other: Self) -> Self {
                Self {
                    $($field: self.$field >> other.$field),+
                }
            }
        }

        impl<$t: Shr<Output = $t> + Copy> Shr<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn shr(self, other: $t) -> Self {
                Self {
                    $($field: self.$field >> other),+
                }
            }
        }

        impl<$t: ShrAssign> ShrAssign for $name<$t> {
            #[inline]
            fn shr_assign(&mut self, other: Self) {
                $(self.$field >>= other.$field;)+
            }
        }

        impl<$t: ShrAssign + Copy> ShrAssign<$t> for $name<$t> {
            #[inline]
            fn shr_assign(&mut self, other: $t) {
                $(self.$field >>= other;)+
            }
        }

        impl<$t: Neg<Output = $t>> Neg for $name<$t> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    $($field: -self.$field),+
                }
            }
        }

        impl<$t: Not<Output = $t>> Not for $name<$t> {
            type Output = Self;

            #[inline]
            fn not(self) -> Self {
                Self {
                    $($field: !self.$field),+
                }
            }
        }

        impl<$t> Index<usize> for $name<$t> {
            type Output = $t;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                let n = 0;
                $(
                    if index == n {
                        return &self.$field;
                    }
                    let n = n + 1;
                )+
                let _ = n;
                panic!("Index out of bounds: {}", index);
            }
        }

        impl<$t> IndexMut<usize> for $name<$t> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let n = 0;
                $(
                    if index == n {
                        return &mut self.$field;
                    }
                    let n = n + 1;
                )+
                let _ = n;
                panic!("Index out of bounds: {}", index);
            }
        }

        impl<$t: Display + Pod> Display for $name<$t> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                let mut s = self.as_array().iter();
                write!(f, "(")?;
                if let Some(first) = s.next() {
                    write!(f, "{}", first)?;
                }
                for value in s {
                    write!(f, ", {}", value)?;
                }
                write!(f, ")")
            }
        }

        impl<$t> IntoIterator for $name<$t> {
            type Item = $t;
            type IntoIter = std::array::IntoIter<$t, { vector!(@count $($field)+) }>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.to_array().into_iter()
            }
        }

        impl<'a, T: Pod> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }

        unsafe impl<$t: Zeroable> Zeroable for $name<$t> {}

        unsafe impl<$t: Pod> Pod for $name<$t> {}
    };
}

vector! {
    struct Vec2<T> {
        x(X = 1, 0): T,
        y(Y = 0, 1): T,
    }
    linearize(x, y)
}

impl<T> Vec2<T> {
    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3 { x: self.x, y: self.y, z }
    }
}

vector! {
    struct Vec3<T> {
        x(X = 1, 0, 0): T,
        y(Y = 0, 1, 0): T,
        z(Z = 0, 0, 1): T,
    }
    linearize(z, y, x)
}

impl<T> Vec3<T> {
    #[inline]
    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Copy + Sub<Output = T>,
        T: Mul<Output = T>,
    {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn xy(self) -> Vec2<T> {
        Vec2 { x: self.x, y: self.y }
    }

    #[inline]
    pub fn xz(self) -> Vec2<T> {
        Vec2 { x: self.x, y: self.z }
    }

    #[inline]
    pub fn xxx(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    pub fn yyy(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    pub fn zzz(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }
}

impl vec3d {
    #[inline]
    pub fn split_int_fract(self) -> (vec3i, vec3f) {
        (self.cast::<i32>(), self.cast::<f32>().fract())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct vec3u4(NonZeroU16);

impl vec3u4 {
    #[inline]
    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        let n = 1 | (x as u16) << 12 | (y as u16) << 8 | (z as u16) << 4;

        Self(unsafe { NonZeroU16::new_unchecked(n) })
    }

    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 16, "x out of range");
        debug_assert!(y < 16, "y out of range");
        debug_assert!(z < 16, "z out of range");

        Self::new_unchecked(x, y, z)
    }

    #[inline]
    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
            Some(Self::new_unchecked(x, y, z))
        } else {
            None
        }
    }

    #[inline]
    pub fn try_from(vec: vec3u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        (self.0.get() >> 12 & 15) as u8
    }

    #[inline]
    pub const fn y(self) -> u8 {
        (self.0.get() >> 8 & 15) as u8
    }

    #[inline]
    pub const fn z(self) -> u8 {
        (self.0.get() >> 4 & 15) as u8
    }

    #[inline]
    pub const fn to_vec3u8(self) -> vec3u8 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Vec3<U>> {
        Some(Vec3 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
        })
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn linearize(&self) -> usize {
        self.x() as usize * 16usize.pow(2) + self.z() as usize * 16 + self.y() as usize
    }
}

assert_eq_size!(Option<vec3u4>, vec3u4);

impl<T: NumCast> From<Vec3<T>> for vec3u4 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(NumCast::from(vec.x).unwrap(), NumCast::from(vec.y).unwrap(), NumCast::from(vec.z).unwrap())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct vec3u5(NonZeroU16);

impl vec3u5 {
    pub const ZERO: Self = Self::new(0, 0, 0);

    #[inline]
    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        let n = 1 | (x as u16) << 11 | (y as u16) << 6 | (z as u16) << 1;

        Self(unsafe { NonZeroU16::new_unchecked(n) })
    }

    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 32, "x out of range");
        debug_assert!(y < 32, "y out of range");
        debug_assert!(z < 32, "z out of range");

        Self::new_unchecked(x, y, z)
    }

    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 32 && y < 32 && z < 32 {
            Some(Self::new_unchecked(x, y, z))
        } else {
            None
        }
    }

    pub fn try_from(vec: vec3u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        ((self.0.get() >> 11) & 31) as u8
    }

    #[inline]
    pub const fn set_x(&mut self, x: u8) {
        debug_assert!(x < 32, "x out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 11)) | ((x as u16) << 11)).unwrap();
    }

    #[inline]
    pub const fn y(self) -> u8 {
        ((self.0.get() >> 6) & 31) as u8
    }

    #[inline]
    pub const fn set_y(&mut self, y: u8) {
        debug_assert!(y < 32, "y out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 6)) | ((y as u16) << 6)).unwrap();
    }

    #[inline]
    pub const fn z(self) -> u8 {
        ((self.0.get() >> 1) & 31) as u8
    }

    #[inline]
    pub const fn set_z(&mut self, z: u8) {
        debug_assert!(z < 32, "z out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 1)) | ((z as u16) << 1)).unwrap();
    }

    #[inline]
    pub const fn into_u8(self) -> vec3u8 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn try_cast<U: NumCast>(self) -> Option<Vec3<U>> {
        Some(Vec3 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
        })
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Vec3<U> {
        self.try_cast().unwrap()
    }

    #[inline]
    pub const fn into_tuple(self) -> (u8, u8, u8) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn linearize(&self) -> usize {
        self.x() as usize * 32usize.pow(2) + self.z() as usize * 32 + self.y() as usize
    }
}

assert_eq_size!(Option<vec3u5>, vec3u5);

impl<T: NumCast> From<Vec3<T>> for vec3u5 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(NumCast::from(vec.x).unwrap(), NumCast::from(vec.y).unwrap(), NumCast::from(vec.z).unwrap())
    }
}

impl Sub for vec3u5 {
    type Output = vec3u5;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3u5::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Default for vec3u5 {
    fn default() -> Self {
        Self::ZERO
    }
}

vector! {
    struct Vec4<T> {
        x(X = 1, 0, 0, 0): T,
        y(Y = 0, 1, 0, 0): T,
        z(Z = 0, 0, 1, 0): T,
        w(W = 0, 0, 0, 1): T,
    }
    linearize(x, y, z, w)
}

impl<T> Vec4<T> {
    pub fn xxxx(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    pub fn yyyy(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    pub fn zzzz(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    pub fn wwww(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Pod, Zeroable, Deserialize, Serialize)]
pub struct vec4u4(u16);

impl vec4u4 {
    pub const fn new_unchecked(x: u8, y: u8, z: u8, w: u8) -> Self {
        let n = (x as u16) << 12 | (y as u16) << 8 | (z as u16) << 4 | w as u16;

        Self(n)
    }

    pub const fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
        debug_assert!(x < 16, "x out of range");
        debug_assert!(y < 16, "y out of range");
        debug_assert!(z < 16, "z out of range");
        debug_assert!(w < 16, "w out of range");

        Self::new_unchecked(x, y, z, w)
    }

    pub const fn try_new(x: u8, y: u8, z: u8, w: u8) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 && w < 16 {
            Some(Self::new_unchecked(x, y, z, w))
        } else {
            None
        }
    }

    pub fn try_from(vec: vec4u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z, vec.w)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        ((self.0 >> 12) & 15) as u8
    }

    #[inline]
    pub const fn y(self) -> u8 {
        ((self.0 >> 8) & 15) as u8
    }

    #[inline]
    pub const fn z(self) -> u8 {
        ((self.0 >> 4) & 15) as u8
    }

    #[inline]
    pub const fn w(self) -> u8 {
        (self.0 & 15) as u8
    }

    #[inline]
    pub const fn to_vec4u8(self) -> vec4u8 {
        Vec4::new(self.x(), self.y(), self.z(), self.w())
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Vec4<U>> {
        Some(Vec4 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
            w: NumCast::from(self.w())?,
        })
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.x(), self.y(), self.z(), self.w())
    }
}

impl<T: NumCast> From<Vec4<T>> for vec4u4 {
    fn from(vec: Vec4<T>) -> Self {
        Self::new(
            NumCast::from(vec.x).unwrap(),
            NumCast::from(vec.y).unwrap(),
            NumCast::from(vec.z).unwrap(),
            NumCast::from(vec.w).unwrap(),
        )
    }
}
