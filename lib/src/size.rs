#![allow(non_camel_case_types)]

use crate::macros::impl_ops;
use crate::vector::{Vec2, Vec3};
use bytemuck::{Pod, Zeroable};
use num::traits::ConstZero;
use num::{NumCast, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub type size2u8 = Size2<u8>;
pub type size2u16 = Size2<u16>;
pub type size2u = Size2<u32>;
pub type size2u64 = Size2<u64>;
pub type size2u128 = Size2<u128>;
pub type size2usize = Size2<usize>;
pub type size2i8 = Size2<i8>;
pub type size2i16 = Size2<i16>;
pub type size2i = Size2<i32>;
pub type size2i64 = Size2<i64>;
pub type size2i128 = Size2<i128>;
pub type size2isize = Size2<isize>;
pub type size2f = Size2<f32>;
pub type size2d = Size2<f64>;

pub type size3u8 = Size3<u8>;
pub type size3u16 = Size3<u16>;
pub type size3u = Size3<u32>;
pub type size3u64 = Size3<u64>;
pub type size3u128 = Size3<u128>;
pub type size3usize = Size3<usize>;
pub type size3i8 = Size3<i8>;
pub type size3i16 = Size3<i16>;
pub type size3i = Size3<i32>;
pub type size3i64 = Size3<i64>;
pub type size3i128 = Size3<i128>;
pub type size3isize = Size3<isize>;
pub type size3f = Size3<f32>;
pub type size3d = Size3<f64>;

macro_rules! size {
    (@count) => { 0 };
    (@count $first:ident $($rest:ident)*) => {
        1 + vector!(@count $($rest)*)
    };
    (
        struct $name:ident<$t:ident> {
        $(
            $field:ident: $ft:ident
        ),+ $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        pub struct $name<$t> {
            $(pub $field: $ft),+
        }

        impl<$t> $name<$t> {
            #[inline]
            pub const fn new($($field: $ft),+) -> Self {
                Self {
                    $($field),+
                }
            }

            #[inline]
            pub const fn splat(value: $t) -> Self
            where
                $t: Copy,
            {
                Self {
                    $($field: value),+
                }
            }

            #[inline]
            pub fn try_cast<U: NumCast>(self) -> Option<$name<U>>
            where
                $t: ToPrimitive,
            {
                Some($name {
                    $($field: NumCast::from(self.$field)?),+
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
            pub fn to_tuple(self) -> ($($ft,)+) {
                ($(self.$field,)+)
            }
        }

        impl<$t: ConstZero> $name<$t> {
            pub const ZERO: Self = Self {
                $($field: $t::ZERO),+
            };
        }

        impl<$t> From<($($ft,)+)> for $name<$t> {
            #[inline]
            fn from(($($field),+): ($($ft,)+)) -> Self {
                Self {
                    $($field),+
                }
            }
        }

        impl_ops! {
            struct $name<$t> {
                $($field: $ft),+
            }
        }

        unsafe impl<$t: Zeroable> Zeroable for $name<$t> {}

        unsafe impl<$t: Pod> Pod for $name<$t> {}
    };
}

size! {
    struct Size2<T> {
        width: T,
        height: T,
    }
}

impl<T> Size2<T> {
    #[inline]
    pub fn to_vec2(self) -> Vec2<T>
    where
        T: Copy,
    {
        Vec2::new(self.width, self.height)
    }

    #[inline]
    pub fn area(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height
    }

    #[inline]
    pub fn aspect(self) -> T
    where
        T: Div<Output = T>,
    {
        self.width / self.height
    }
}

size! {
    struct Size3<T> {
        width: T,
        height: T,
        depth: T,
    }
}

impl<T> Size3<T> {
    #[inline]
    pub fn to_vec3(self) -> Vec3<T>
    where
        T: Copy,
    {
        Vec3::new(self.width, self.height, self.depth)
    }

    #[inline]
    pub fn volume(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height * self.depth
    }
}
