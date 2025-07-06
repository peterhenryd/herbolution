#[macro_export]
macro_rules! impl_simd_base_overloads {
    ($s:ident) => {
        impl core::ops::Add<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::add(self, rhs)
            }
        }

        impl core::ops::AddAssign<Self> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::add(*self, rhs);
            }
        }

        impl core::ops::Add<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::add(self, <Self as $crate::simd::SimdBaseIo>::set1(rhs))
            }
        }

        impl core::ops::Add<$s> for <$s as $crate::simd::SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn add(self, rhs: $s) -> $s {
                $crate::simd::SimdBaseOps::add(<$s as $crate::simd::SimdBaseIo>::set1(self), rhs)
            }
        }

        impl core::ops::AddAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn add_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::add(*self, <Self as $crate::simd::SimdBaseIo>::set1(rhs));
            }
        }

        impl core::ops::Sub<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::sub(self, rhs)
            }
        }

        impl core::ops::SubAssign<Self> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::sub(*self, rhs);
            }
        }

        impl core::ops::Sub<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::sub(self, <Self as $crate::simd::SimdBaseIo>::set1(rhs))
            }
        }

        impl core::ops::Sub<$s> for <$s as $crate::simd::SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn sub(self, rhs: $s) -> $s {
                $crate::simd::SimdBaseOps::sub(<$s as $crate::simd::SimdBaseIo>::set1(self), rhs)
            }
        }

        impl core::ops::SubAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::sub(*self, <Self as $crate::simd::SimdBaseIo>::set1(rhs));
            }
        }

        impl core::ops::Mul<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::mul(self, rhs)
            }
        }

        impl core::ops::MulAssign<Self> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::mul(*self, rhs);
            }
        }

        impl core::ops::Mul<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::mul(self, <Self as $crate::simd::SimdBaseIo>::set1(rhs))
            }
        }

        impl core::ops::MulAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::mul(*self, <Self as $crate::simd::SimdBaseIo>::set1(rhs));
            }
        }

        impl core::ops::Mul<$s> for <$s as $crate::simd::SimdConsts>::Scalar {
            type Output = $s;

            #[inline(always)]
            fn mul(self, rhs: $s) -> $s {
                $crate::simd::SimdBaseOps::mul(<$s as $crate::simd::SimdBaseIo>::set1(self), rhs)
            }
        }

        impl core::ops::BitAnd<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::bit_and(self, rhs)
            }
        }

        impl core::ops::BitAndAssign<Self> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::bit_and(*self, rhs);
            }
        }

        impl core::ops::BitAnd<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitand(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::bit_and(self, $crate::simd::SimdBaseIo::set1(rhs))
            }
        }

        impl core::ops::BitAndAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::bit_and(*self, $crate::simd::SimdBaseIo::set1(rhs));
            }
        }

        impl core::ops::BitOr<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::bit_or(self, rhs)
            }
        }

        impl core::ops::BitOrAssign<Self> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::bit_or(*self, rhs);
            }
        }

        impl core::ops::BitOr<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitor(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::bit_or(self, $crate::simd::SimdBaseIo::set1(rhs))
            }
        }

        impl core::ops::BitOrAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::bit_or(*self, $crate::simd::SimdBaseIo::set1(rhs));
            }
        }

        impl core::ops::BitXor<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self {
                $crate::simd::SimdBaseOps::bit_xor(self, rhs)
            }
        }

        impl core::ops::BitXorAssign<Self> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdBaseOps::bit_xor(*self, rhs);
            }
        }

        impl core::ops::BitXor<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn bitxor(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdBaseOps::bit_xor(self, $crate::simd::SimdBaseIo::set1(rhs))
            }
        }

        impl core::ops::BitXorAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdBaseOps::bit_xor(*self, $crate::simd::SimdBaseIo::set1(rhs));
            }
        }

        impl core::ops::Not for $s {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                $crate::simd::SimdBaseOps::bit_not(self)
            }
        }

        impl core::ops::Neg for $s {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                <Self as $crate::simd::SimdBaseIo>::zeroes() - self
            }
        }

        impl core::ops::Index<usize> for $s {
            type Output = <Self as $crate::simd::SimdConsts>::Scalar;

            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                unsafe { &(*self.transmute_into_array_ref())[index] }
            }
        }

        impl core::ops::IndexMut<usize> for $s {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                unsafe { &mut (*self.transmute_into_array_mut())[index] }
            }
        }

        impl core::fmt::Debug for $s {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                unsafe {
                    let array = self.transmute_into_array_ref();
                    write!(f, "{}([{:?}])", stringify!($s), array)
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_simd_int_overloads {
    ($s:ident) => {
        impl core::ops::Shl<i32> for $s {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: i32) -> Self {
                $crate::simd::SimdInt::shl(self, rhs)
            }
        }

        impl core::ops::ShlAssign<i32> for $s {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: i32) {
                *self = $crate::simd::SimdInt::shl(*self, rhs);
            }
        }

        impl core::ops::Shr<i32> for $s {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: i32) -> Self {
                $crate::simd::SimdInt::shr(self, rhs)
            }
        }

        impl core::ops::ShrAssign<i32> for $s {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: i32) {
                *self = $crate::simd::SimdInt::shr(*self, rhs);
            }
        }
    };
}
#[macro_export]
macro_rules! impl_simd_float_overloads {
    ($s:ident) => {
        impl core::ops::Div<Self> for $s {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                $crate::simd::SimdFloat::div(self, rhs)
            }
        }

        impl core::ops::DivAssign<Self> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                *self = $crate::simd::SimdFloat::div(*self, rhs);
            }
        }

        impl core::ops::Div<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) -> Self {
                $crate::simd::SimdFloat::div(self, <Self as $crate::simd::SimdBaseIo>::set1(rhs))
            }
        }

        impl core::ops::DivAssign<<Self as $crate::simd::SimdConsts>::Scalar> for $s {
            #[inline(always)]
            fn div_assign(&mut self, rhs: <Self as $crate::simd::SimdConsts>::Scalar) {
                *self = $crate::simd::SimdFloat::div(*self, <Self as $crate::simd::SimdBaseIo>::set1(rhs));
            }
        }
    };
}
#[macro_export]
macro_rules! horizontal_add_scalar {
    (i8) => {
        i64
    };
    (i16) => {
        i64
    };
    (i32) => {
        i64
    };
    (i64) => {
        i64
    };
    (f32) => {
        f32
    };
    (f64) => {
        f64
    };
}
#[macro_export]
macro_rules! define_simd_type {
    (Scalar, $ty:ty, $width:literal, $underlying:ty) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width>](pub $underlying);

            $crate::impl_simd_base_overloads!([<$ty:upper x $width>]);

            impl $crate::simd::SimdConsts for [<$ty:upper x $width>] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = $crate::horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
                type Backend = Scalar;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width>] {
                #[inline(always)]
                fn [<try_transmute_ scalar>](&self) -> $underlying {
                    self.0
                }

                #[inline(always)]
                fn [<try_transmute_from_ scalar>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };

    ($engine:ident, $ty:ty, $width:literal, $underlying:ty) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width>]($underlying);

            $crate::impl_simd_base_overloads!([<$ty:upper x $width>]);

            impl $crate::simd::SimdConsts for [<$ty:upper x $width>] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = $crate::horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
                type Backend = $engine;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width>] {
                #[inline(always)]
                fn [<try_transmute_ $engine:lower>](&self) -> $underlying {
                    self.0
                }

                #[inline(always)]
                fn [<try_transmute_from_ $engine:lower>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };
    ($engine:ident, $ty:ty, $width:literal, $underlying:ty, $suffix:ident) => {
        paste::item! {
            #[derive(Copy, Clone)]
            pub struct [<$ty:upper x $width $suffix>]($underlying);

            $crate::impl_simd_base_overloads!([<$ty:upper x $width $suffix>]);

            impl $crate::simd::SimdConsts for [<$ty:upper x $width $suffix >] {
                const WIDTH: usize = $width;
                type Scalar = $ty;
                type HorizontalAddScalar = $crate::horizontal_add_scalar!($ty);
                type ArrayRepresentation = [$ty; $width];
                type UnderlyingType = $underlying;
                type Backend = $engine;
            }

            impl [<SimdTransmute $ty:upper>] for [<$ty:upper x $width $suffix >] {
                #[inline(always)]
                fn [<try_transmute_ $engine:lower>](&self) -> $underlying {
                    self.0
                }

                #[inline(always)]
                fn [<try_transmute_from_ $engine:lower>](val: $underlying) -> Self {
                    Self(val)
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_simd_base {
    ($engine:ident, $ty:ident, $scalar_ty:ident, |$self:ident| {
        $($hadd:tt)*
    }) => {
        impl $crate::simd::SimdBaseIo for $ty {
            #[inline(always)]
            fn zeroes() -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::zeroes()) }
            }

            #[inline(always)]
            fn set1(x: Self::Scalar) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::set1(x)) }
            }

            #[inline(always)]
            unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::load_aligned(array.as_ptr())) }
            }

            #[inline(always)]
            unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::load_unaligned(ptr)) }
            }

            #[inline(always)]
            unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
                unsafe { $crate::simd::Ops::<$engine, $scalar_ty>::store_unaligned(ptr, self.0) }
            }

            #[inline(always)]
            unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::load_aligned(ptr)) }
            }

            #[inline(always)]
            unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
                unsafe { $crate::simd::Ops::<$engine, $scalar_ty>::store_aligned(ptr, self.0) }
            }

            #[inline(always)]
            unsafe fn underlying_value(self) -> Self::UnderlyingType {
                self.0
            }

            #[inline(always)]
            unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
                &mut self.0
            }

            #[inline(always)]
            unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
                Self(value)
            }
        }

        impl $crate::simd::SimdBaseOps for $ty {
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::add(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::sub(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::mul(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn bit_and(self, rhs: Self) -> Self {
                unsafe {
                    let left = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = $crate::simd::Ops::<$engine, $crate::simd::binary>::bit_and(left, right);
                    paste::paste! {
                        Self($crate::simd::Ops::<$engine, $crate::simd::binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_or(self, rhs: Self) -> Self {
                unsafe {
                    let left = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = $crate::simd::Ops::<$engine, $crate::simd::binary>::bit_or(left, right);
                    paste::paste! {
                        Self($crate::simd::Ops::<$engine, $crate::simd::binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_xor(self, rhs: Self) -> Self {
                unsafe {
                    let left = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = $crate::simd::Ops::<$engine, $crate::simd::binary>::bit_xor(left, right);
                    paste::paste! {
                        Self($crate::simd::Ops::<$engine, $crate::simd::binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn bit_not(self) -> Self {
                unsafe {
                    let val = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let result = $crate::simd::Ops::<$engine, $crate::simd::binary>::bit_not(val);
                    paste::paste! {
                        Self($crate::simd::Ops::<$engine, $crate::simd::binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn abs(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::abs(self.0)) }
            }

            #[inline(always)]
            fn and_not(self, rhs: Self) -> Self {
                unsafe {
                    let left = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(self.0);
                    let right = $crate::simd::Ops::<$engine, $scalar_ty>::bitcast_binary(rhs.0);
                    let result = $crate::simd::Ops::<$engine, $crate::simd::binary>::bit_andnot(right, left);
                    paste::paste! {
                        Self($crate::simd::Ops::<$engine, $crate::simd::binary>::[<bitcast_ $scalar_ty>](result))
                    }
                }
            }

            #[inline(always)]
            fn blendv(self, a: Self, b: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::blendv(a.0, b.0, self.0)) }
            }

            #[inline(always)]
            fn cmp_eq(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::eq(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_neq(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::neq(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_lt(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::lt(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_lte(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::lte(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_gt(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::gt(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn cmp_gte(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::gte(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn max(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::max(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn min(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::min(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn horizontal_add($self) -> Self::HorizontalAddScalar {
                $($hadd)*
            }
        }
    };
}
#[macro_export]
macro_rules! impl_simd_int {
    ($engine:ident, $ty:ident, $scalar_ty:ident, |$self:ident| {
        $($hadd:tt)*
    }) => {
        impl $crate::simd::SimdInt for $ty {
            #[inline(always)]
            fn shl(self, rhs: i32) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::shl(self.0, rhs)) }
            }

            #[inline(always)]
            fn shr(self, rhs: i32) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::shr(self.0, rhs)) }
            }

            #[inline(always)]
            fn shl_const<const BY: i32>(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::shl_const::<BY>(self.0)) }
            }

            #[inline(always)]
            fn shr_const<const BY: i32>(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::shr_const::<BY>(self.0)) }
            }

            #[inline(always)]
            fn horizontal_unsigned_add($self) -> Self::HorizontalAddScalar {
                $($hadd)*
            }

            #[inline(always)]
            fn from_i64(value: i64) -> Self {
                Self::set1(value as $scalar_ty)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_simd_float {
    ($engine:ident, $ty:ident, $scalar_ty:ident) => {
        impl $crate::simd::SimdFloat for $ty {
            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::div(self.0, rhs.0)) }
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::ceil(self.0)) }
            }

            #[inline(always)]
            fn floor(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::floor(self.0)) }
            }

            #[inline(always)]
            fn round(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::round(self.0)) }
            }

            #[inline(always)]
            fn fast_ceil(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::fast_ceil(self.0)) }
            }

            #[inline(always)]
            fn fast_floor(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::fast_floor(self.0)) }
            }

            #[inline(always)]
            fn fast_round(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::fast_round(self.0)) }
            }

            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::mul_add(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn mul_sub(self, a: Self, b: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::mul_sub(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn neg_mul_add(self, a: Self, b: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::neg_mul_add(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn neg_mul_sub(self, a: Self, b: Self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::neg_mul_sub(self.0, a.0, b.0)) }
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::sqrt(self.0)) }
            }

            #[inline(always)]
            fn rsqrt(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, $scalar_ty>::rsqrt(self.0)) }
            }

            #[inline(always)]
            fn from_f64(value: f64) -> Self {
                Self::set1(value as $scalar_ty)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_i8_simd_type {
    ($engine:ident, $i8_ty:ident, $i16_ty:ident) => {
        $crate::impl_simd_base!($engine, $i8_ty, i8, |self| {
            self.partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
        });

        $crate::impl_simd_int!($engine, $i8_ty, i8, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl $crate::simd::SimdI8 for $i8_ty {
            #[inline(always)]
            fn extend_to_i16(self) -> (<Self::Backend as Simd>::I16, <Self::Backend as Simd>::I16) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i8>::extend_i16(self.0) };
                ($i16_ty(a), $i16_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i16(self) -> (<Self::Backend as Simd>::I16, <Self::Backend as Simd>::I16) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i8>::unsigned_extend_i16(self.0) };
                ($i16_ty(a), $i16_ty(b))
            }

            #[inline(always)]
            fn get_mask(self) -> u32 {
                unsafe { $crate::simd::Ops::<$engine, i8>::get_mask(self.0) }
            }

            #[inline(always)]
            fn is_truthy(self) -> bool {
                unsafe { $crate::simd::Ops::<$engine, i8>::is_truthy(self.0) }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_i16_simd_type {
    ($engine:ident, $i16_ty:ident, $i32_ty:ident) => {
        $crate::impl_simd_base!($engine, $i16_ty, i16, |self| {
            self.partial_horizontal_add()
                .partial_horizontal_add()
                .partial_horizontal_add()
        });
        $crate::impl_simd_int!($engine, $i16_ty, i16, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl $crate::simd::SimdI16 for $i16_ty {
            #[inline(always)]
            fn extend_to_i32(self) -> (<Self::Backend as Simd>::I32, <Self::Backend as Simd>::I32) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i16>::extend_i32(self.0) };
                ($i32_ty(a), $i32_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i32(self) -> (<Self::Backend as Simd>::I32, <Self::Backend as Simd>::I32) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i16>::unsigned_extend_i32(self.0) };
                ($i32_ty(a), $i32_ty(b))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_i32_simd_type {
    ($engine:ident, $i32_ty:ident, $f32_ty:ident, $i64_ty:ident) => {
        $crate::impl_simd_base!($engine, $i32_ty, i32, |self| {
            self.partial_horizontal_add()
                .partial_horizontal_add()
        });

        $crate::impl_simd_int!($engine, $i32_ty, i32, |self| {
            self.partial_horizontal_unsigned_add()
                .partial_horizontal_add()
        });

        impl $crate::simd::SimdI32 for $i32_ty {
            #[inline(always)]
            fn bitcast_f32(self) -> <Self::Backend as Simd>::F32 {
                unsafe { $f32_ty($crate::simd::Ops::<$engine, i32>::bitcast_f32(self.0)) }
            }

            #[inline(always)]
            fn cast_f32(self) -> <Self::Backend as Simd>::F32 {
                unsafe { $f32_ty($crate::simd::Ops::<$engine, i32>::cast_f32(self.0)) }
            }

            #[inline(always)]
            fn extend_to_i64(self) -> (<Self::Backend as Simd>::I64, <Self::Backend as Simd>::I64) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i32>::extend_i64(self.0) };
                ($i64_ty(a), $i64_ty(b))
            }

            #[inline(always)]
            fn unsigned_extend_to_i64(self) -> (<Self::Backend as Simd>::I64, <Self::Backend as Simd>::I64) {
                let (a, b) = unsafe { $crate::simd::Ops::<$engine, i32>::unsigned_extend_i64(self.0) };
                ($i64_ty(a), $i64_ty(b))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_i64_simd_type {
    ($engine:ident, $i64_ty:ident, $f64_ty:ident) => {
        $crate::impl_simd_base!($engine, $i64_ty, i64, |self| { self.partial_horizontal_add() });

        $crate::impl_simd_int!($engine, $i64_ty, i64, |self| { self.partial_horizontal_add() });

        impl $crate::simd::SimdI64 for $i64_ty {
            #[inline(always)]
            fn bitcast_f64(self) -> <Self::Backend as Simd>::F64 {
                unsafe { $f64_ty($crate::simd::Ops::<$engine, i64>::bitcast_f64(self.0)) }
            }

            #[inline(always)]
            fn cast_f64(self) -> <Self::Backend as Simd>::F64 {
                unsafe { $f64_ty($crate::simd::Ops::<$engine, i64>::cast_f64(self.0)) }
            }

            #[inline(always)]
            fn partial_horizontal_add(self) -> i64 {
                unsafe { $crate::simd::Ops::<$engine, i64>::horizontal_add(self.0) }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_f32_simd_type {
    ($engine:ident, $f32_ty:ident, $i32_ty:ident) => {
        $crate::impl_simd_base!($engine, $f32_ty, f32, |self| {
            unsafe { $crate::simd::Ops::<$engine, f32>::horizontal_add(self.0) }
        });

        $crate::impl_simd_float!($engine, $f32_ty, f32);

        impl $crate::simd::SimdF32 for $f32_ty {
            #[inline(always)]
            fn bitcast_i32(self) -> <Self::Backend as Simd>::I32 {
                unsafe { $i32_ty($crate::simd::Ops::<$engine, f32>::bitcast_i32(self.0)) }
            }

            #[inline(always)]
            fn cast_i32(self) -> <Self::Backend as Simd>::I32 {
                unsafe { $i32_ty($crate::simd::Ops::<$engine, f32>::cast_i32(self.0)) }
            }

            #[inline(always)]
            fn fast_inverse(self) -> Self {
                unsafe { Self($crate::simd::Ops::<$engine, f32>::recip(self.0)) }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_f64_simd_type {
    ($engine:ident, $f64_ty:ident, $i64_ty:ident) => {
        $crate::impl_simd_base!($engine, $f64_ty, f64, |self| {
            unsafe { $crate::simd::Ops::<$engine, f64>::horizontal_add(self.0) }
        });

        $crate::impl_simd_float!($engine, $f64_ty, f64);

        impl $crate::simd::SimdF64 for $f64_ty {
            #[inline(always)]
            fn bitcast_i64(self) -> <Self::Backend as Simd>::I64 {
                unsafe { $i64_ty($crate::simd::Ops::<$engine, f64>::bitcast_i64(self.0)) }
            }

            #[inline(always)]
            fn cast_i64(self) -> <Self::Backend as Simd>::I64 {
                unsafe { $i64_ty($crate::simd::Ops::<$engine, f64>::cast_i64(self.0)) }
            }
        }
    };
}
