macro_rules! vector {
    (@count) => { 0 };
    (@count $first:ident $($rest:ident)*) => {
        1 + vector!(@count $($rest)*)
    };
    (
        struct $name:ident<$t:ident> {
            $($field:ident($constant:ident = $($literal:literal),+): $ft:ident),+ $(,)?
        }
        linearize($($order:ident),+)
    ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Deserialize, serde::Serialize)]
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
                let mut n = 0;
                let vec = $name {
                    $($field: {
                        let value = f(n);
                        n += 1;
                        value
                    }),+
                };
                let _ = n;
                vec
            }

            #[inline]
            pub fn zero() -> Self
            where
                $t: num::traits::Zero
            {
                Self {
                    $($field: num::traits::Zero::zero()),+
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
                $t: std::ops::Mul<Output = $t>,
                $t: std::ops::Add<Output = $t>,
                $t: num::traits::Zero
            {
                $(
                    self.$field * other.$field +
                )+ num::traits::Zero::zero()
            }

            #[inline]
            pub fn length_squared(self) -> $t
            where
                $t: Copy,
                $t: std::ops::Mul<Output = $t>,
                $t: std::ops::Add<Output = $t>,
                $t: num::traits::Zero,
            {
                self.dot(self)
            }

            #[inline]
            pub fn length(self) -> T
            where
                $t: num::traits::Float,
            {
                self.length_squared().sqrt()
            }

            #[inline]
            pub fn normalize(self) -> Self
            where
                $t: num::traits::Float,
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
            pub fn try_cast<U: num::traits::NumCast>(self) -> Option<$name<U>>
            where
                $t: num::traits::ToPrimitive,
            {
                Some($name {
                    $(
                        $field: U::from(self.$field)?,
                    )+
                })
            }

            #[inline]
            pub fn cast<U: num::traits::NumCast>(self) -> $name<U>
            where
                $t: num::traits::ToPrimitive,
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
                T: bytemuck::Pod,
            {
                bytemuck::cast_ref(self)
            }

            #[inline]
            pub fn as_slice(&self) -> &[$t]
            where
                $t: bytemuck::Pod,
            {
                self.as_array()
            }

            #[inline]
            pub fn smallest(&self) -> $t
            where
                $t: PartialOrd + bytemuck::Pod,
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
                $t: PartialOrd + bytemuck::Pod,
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
                $t: num::traits::ConstZero,
            {
                std::mem::replace(self, Self::ZERO)
            }

            #[inline]
            pub fn linearize(self, length: $t) -> $t
            where
                $t: Copy,
                $t: std::ops::MulAssign,
                $t: std::ops::AddAssign,
                $t: num::traits::ConstZero,
                $t: num::traits::ConstOne,
            {
                let mut n = T::ONE;
                let mut x = T::ZERO;
                $(
                    x += self.$order * n;
                    n *= length;
                )+
                x
            }

            #[inline]
            pub fn abs(self) -> Self
            where
                $t: num::traits::Signed,
            {
                Self {
                    $($field: self.$field.abs()),+
                }
            }

            #[inline]
            pub fn signum(self) -> Self
            where
                $t: num::traits::Signed,
            {
                Self {
                    $($field: self.$field.signum()),+
                }
            }

            #[inline]
            pub fn is_positive(&self) -> bool
            where
                $t: num::traits::Signed,
            {
                $(self.$field.is_positive())&&+
            }

            #[inline]
            pub fn is_negative(&self) -> bool
            where
                $t: num::traits::Signed,
            {
                $(self.$field.is_negative())&&+
            }

            #[inline]
            pub fn floor(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.floor()),+
                }
            }

            #[inline]
            pub fn ceil(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.ceil()),+
                }
            }

            #[inline]
            pub fn round(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.round()),+
                }
            }

            #[inline]
            pub fn trunc(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.trunc()),+
                }
            }

            #[inline]
            pub fn fract(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.fract()),+
                }
            }

            #[inline]
            pub fn recip(self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.recip()),+
                }
            }

            #[inline]
            pub fn pow(self, n: Self) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.powf(n.$field)),+
                }
            }

            pub fn powf(self, n: $t) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.powf(n)),+
                }
            }

            #[inline]
            pub fn powi(self, n: i32) -> Self
            where
                $t: num::traits::Float,
            {
                Self {
                    $($field: self.$field.powi(n)),+
                }
            }

            #[inline]
            pub fn sqrt(self) -> Self
            where
                $t: num::traits::Float,
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
            pub fn min(self, other: Self) -> Self
            where
                $t: PartialOrd
            {
                Self {
                    $($field: if self.$field < other.$field { self.$field } else { other.$field }),+
                }
            }
        }

        impl<$t: num::traits::ConstZero> $name<$t> {
            pub const ZERO: Self = Self {
                $($field: T::ZERO),+
            };
        }

        impl<$t: num::traits::ConstOne> $name<$t> {
            pub const ONE: Self = Self {
                $($field: T::ONE),+
            };
        }

        impl<$t: num::traits::ConstZero + num::traits::ConstOne> $name<$t> {
            $(
                pub const $constant: Self = Self::new($(if $literal == 1 { T::ONE } else { T::ZERO }),+);
            )+
        }

        impl<$t: Default> Default for $name<$t> {
            fn default() -> Self {
                Self {
                    $($field: Default::default()),+
                }
            }
        }

        impl<$t> From<($($ft),+)> for $name<$t> {
            fn from(($($field),+): ($($ft),+)) -> Self {
                Self {
                    $($field),+
                }
            }
        }

        impl<$t> From<[$t; vector!(@count $($field)+)]> for $name<$t> {
            fn from([$($field),+]: [$t; vector!(@count $($field)+)]) -> Self {
                Self {
                    $($field),+
                }
            }
        }


        impl<$t> Into<($($ft),+)> for $name<$t> {
            fn into(self) -> ($($ft),+) {
                ($(self.$field),+)
            }
        }

        impl<$t> Into<[$t; vector!(@count $($field)+)]> for $name<$t> {
            fn into(self) -> [$t; vector!(@count $($field)+)] {
                [$(self.$field),+]
            }
        }

        impl<$t: std::ops::Add<Output = $t>> std::ops::Add for $name<$t> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),+
                }
            }
        }

        impl<$t: std::ops::Add<Output = $t> + Copy> std::ops::Add<$t> for $name<$t> {
            type Output = Self;

            fn add(self, other: $t) -> Self {
                Self {
                    $($field: self.$field + other),+
                }
            }
        }

        impl<$t: std::ops::AddAssign> std::ops::AddAssign for $name<$t> {
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field;)+
            }
        }

        impl<$t: std::ops::AddAssign + Copy> std::ops::AddAssign<$t> for $name<$t> {
            fn add_assign(&mut self, other: $t) {
                $(self.$field += other;)+
            }
        }

        impl<$t: std::ops::Sub<Output = $t>> std::ops::Sub for $name<$t> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),+
                }
            }
        }

        impl<$t: std::ops::Sub<Output = $t> + Copy> std::ops::Sub<$t> for $name<$t> {
            type Output = Self;

            fn sub(self, other: $t) -> Self {
                Self {
                    $($field: self.$field - other),+
                }
            }
        }

        impl<$t: std::ops::SubAssign> std::ops::SubAssign for $name<$t> {
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field;)+
            }
        }

        impl<$t: std::ops::SubAssign + Copy> std::ops::SubAssign<$t> for $name<$t> {
            fn sub_assign(&mut self, other: $t) {
                $(self.$field -= other;)+
            }
        }

        impl<$t: std::ops::Mul<Output = $t>> std::ops::Mul for $name<$t> {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self {
                    $($field: self.$field * other.$field),+
                }
            }
        }

        impl<$t: std::ops::Mul<Output = $t> + Copy> std::ops::Mul<$t> for $name<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> Self {
                Self {
                    $($field: self.$field * other),+
                }
            }
        }

        impl<$t: std::ops::MulAssign> std::ops::MulAssign for $name<$t> {
            fn mul_assign(&mut self, other: Self) {
                $(self.$field *= other.$field;)+
            }
        }

        impl<$t: std::ops::MulAssign + Copy> std::ops::MulAssign<$t> for $name<$t> {
            fn mul_assign(&mut self, other: $t) {
                $(self.$field *= other;)+
            }
        }

        impl<$t: std::ops::Div<Output = $t>> std::ops::Div for $name<$t> {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self {
                    $($field: self.$field / other.$field),+
                }
            }
        }

        impl<$t: std::ops::Div<Output = $t> + Copy> std::ops::Div<$t> for $name<$t> {
            type Output = Self;

            fn div(self, other: $t) -> Self {
                Self {
                    $($field: self.$field / other),+
                }
            }
        }

        impl<$t: std::ops::DivAssign> std::ops::DivAssign for $name<$t> {
            fn div_assign(&mut self, other: Self) {
                $(self.$field /= other.$field;)+
            }
        }

        impl<$t: std::ops::DivAssign + Copy> std::ops::DivAssign<$t> for $name<$t> {
            fn div_assign(&mut self, other: $t) {
                $(self.$field /= other;)+
            }
        }

        impl<$t: std::ops::Rem<Output = $t>> std::ops::Rem for $name<$t> {
            type Output = Self;

            fn rem(self, other: Self) -> Self {
                Self {
                    $($field: self.$field % other.$field),+
                }
            }
        }

        impl<$t: std::ops::Rem<Output = $t> + Copy> std::ops::Rem<$t> for $name<$t> {
            type Output = Self;

            fn rem(self, other: $t) -> Self {
                Self {
                    $($field: self.$field % other),+
                }
            }
        }

        impl<$t: std::ops::RemAssign> std::ops::RemAssign for $name<$t> {
            fn rem_assign(&mut self, other: Self) {
                $(self.$field %= other.$field;)+
            }
        }

        impl<$t: std::ops::RemAssign + Copy> std::ops::RemAssign<$t> for $name<$t> {
            fn rem_assign(&mut self, other: $t) {
                $(self.$field %= other;)+
            }
        }

        impl<$t: std::ops::BitAnd<Output = $t>> std::ops::BitAnd for $name<$t> {
            type Output = Self;

            fn bitand(self, other: Self) -> Self {
                Self {
                    $($field: self.$field & other.$field),+
                }
            }
        }

        impl<$t: std::ops::BitAnd<Output = $t> + Copy> std::ops::BitAnd<$t> for $name<$t> {
            type Output = Self;

            fn bitand(self, other: $t) -> Self {
                Self {
                    $($field: self.$field & other),+
                }
            }
        }

        impl<$t: std::ops::BitAndAssign> std::ops::BitAndAssign for $name<$t> {
            fn bitand_assign(&mut self, other: Self) {
                $(self.$field &= other.$field;)+
            }
        }

        impl<$t: std::ops::BitAndAssign + Copy> std::ops::BitAndAssign<$t> for $name<$t> {
            fn bitand_assign(&mut self, other: $t) {
                $(self.$field &= other;)+
            }
        }

        impl<$t: std::ops::BitOr<Output = $t>> std::ops::BitOr for $name<$t> {
            type Output = Self;

            fn bitor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field | other.$field),+
                }
            }
        }

        impl<$t: std::ops::BitOr<Output = $t> + Copy> std::ops::BitOr<$t> for $name<$t> {
            type Output = Self;

            fn bitor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field | other),+
                }
            }
        }

        impl<$t: std::ops::BitOrAssign> std::ops::BitOrAssign for $name<$t> {
            fn bitor_assign(&mut self, other: Self) {
                $(self.$field |= other.$field;)+
            }
        }

        impl<$t: std::ops::BitOrAssign + Copy> std::ops::BitOrAssign<$t> for $name<$t> {
            fn bitor_assign(&mut self, other: $t) {
                $(self.$field |= other;)+
            }
        }

        impl<$t: std::ops::BitXor<Output = $t>> std::ops::BitXor for $name<$t> {
            type Output = Self;

            fn bitxor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field ^ other.$field),+
                }
            }
        }

        impl<$t: std::ops::BitXor<Output = $t> + Copy> std::ops::BitXor<$t> for $name<$t> {
            type Output = Self;

            fn bitxor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field ^ other),+
                }
            }
        }

        impl<$t: std::ops::BitXorAssign> std::ops::BitXorAssign for $name<$t> {
            fn bitxor_assign(&mut self, other: Self) {
                $(self.$field ^= other.$field;)+
            }
        }

        impl<$t: std::ops::BitXorAssign + Copy> std::ops::BitXorAssign<$t> for $name<$t> {
            fn bitxor_assign(&mut self, other: $t) {
                $(self.$field ^= other;)+
            }
        }

        impl<$t: std::ops::Shl<Output = $t>> std::ops::Shl for $name<$t> {
            type Output = Self;

            fn shl(self, other: Self) -> Self {
                Self {
                    $($field: self.$field << other.$field),+
                }
            }
        }

        impl<$t: std::ops::Shl<Output = $t> + Copy> std::ops::Shl<$t> for $name<$t> {
            type Output = Self;

            fn shl(self, other: $t) -> Self {
                Self {
                    $($field: self.$field << other),+
                }
            }
        }

        impl<$t: std::ops::ShlAssign> std::ops::ShlAssign for $name<$t> {
            fn shl_assign(&mut self, other: Self) {
                $(self.$field <<= other.$field;)+
            }
        }

        impl<$t: std::ops::ShlAssign + Copy> std::ops::ShlAssign<$t> for $name<$t> {
            fn shl_assign(&mut self, other: $t) {
                $(self.$field <<= other;)+
            }
        }

        impl<$t: std::ops::Shr<Output = $t>> std::ops::Shr for $name<$t> {
            type Output = Self;

            fn shr(self, other: Self) -> Self {
                Self {
                    $($field: self.$field >> other.$field),+
                }
            }
        }

        impl<$t: std::ops::Shr<Output = $t> + Copy> std::ops::Shr<$t> for $name<$t> {
            type Output = Self;

            fn shr(self, other: $t) -> Self {
                Self {
                    $($field: self.$field >> other),+
                }
            }
        }

        impl<$t: std::ops::ShrAssign> std::ops::ShrAssign for $name<$t> {
            fn shr_assign(&mut self, other: Self) {
                $(self.$field >>= other.$field;)+
            }
        }

        impl<$t: std::ops::ShrAssign + Copy> std::ops::ShrAssign<$t> for $name<$t> {
            fn shr_assign(&mut self, other: $t) {
                $(self.$field >>= other;)+
            }
        }

        impl<$t: std::ops::Neg<Output = $t>> std::ops::Neg for $name<$t> {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    $($field: -self.$field),+
                }
            }
        }

        impl<$t: std::ops::Not<Output = $t>> std::ops::Not for $name<$t> {
            type Output = Self;

            fn not(self) -> Self {
                Self {
                    $($field: !self.$field),+
                }
            }
        }

        impl<$t> std::ops::Index<usize> for $name<$t> {
            type Output = $t;

            #[inline(always)]
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

        impl<$t> std::ops::IndexMut<usize> for $name<$t> {
            #[inline(always)]
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

        impl<$t: std::fmt::Display + bytemuck::Pod> std::fmt::Display for $name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

            fn into_iter(self) -> Self::IntoIter {
                self.to_array().into_iter()
            }
        }

        impl<'a, T: bytemuck::Pod> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }

        unsafe impl<$t: bytemuck::Zeroable> bytemuck::Zeroable for $name<$t> {}

        unsafe impl<$t: bytemuck::Pod> bytemuck::Pod for $name<$t> {}
    };
}

pub(crate) use vector;
