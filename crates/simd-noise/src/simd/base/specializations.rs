use core::ops::*;

use super::transmute::*;
use crate::simd::{Simd, SimdBaseOps};

pub trait SimdInt: SimdBaseOps + Shl<i32, Output = Self> + ShlAssign<i32> + Shr<i32, Output = Self> + ShrAssign<i32> {
    fn shl(self, rhs: i32) -> Self;

    fn shr(self, rhs: i32) -> Self;

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        SimdInt::shl(self, BY)
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        SimdInt::shr(self, BY)
    }

    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar;

    fn from_i64(value: i64) -> Self;
}

pub trait SimdInt8: SimdInt<Scalar = i8, HorizontalAddScalar = i64> + SimdTransmuteI8 {
    fn extend_to_i16(self) -> (<Self::Backend as Simd>::Vi16, <Self::Backend as Simd>::Vi16);

    fn unsigned_extend_to_i16(self) -> (<Self::Backend as Simd>::Vi16, <Self::Backend as Simd>::Vi16);

    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Backend as Simd>::Vi16 {
        let (a, b) = self.extend_to_i16();
        a + b
    }

    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Backend as Simd>::Vi16 {
        let (a, b) = self.unsigned_extend_to_i16();
        a + b
    }

    fn get_mask(self) -> u32;

    #[inline(always)]
    fn is_any_truthy(self) -> bool {
        self.get_mask() != 0
    }

    fn is_truthy(self) -> bool;

    #[inline(always)]
    fn index_of_last_truthy(self) -> Option<usize> {
        let leading = self.get_mask().leading_zeros();
        if leading >= Self::WIDTH as u32 {
            None
        } else {
            Some(leading as usize)
        }
    }

    #[inline(always)]
    fn index_of_last_falsy(self) -> Option<usize> {
        let leading = self.get_mask().leading_ones();
        if leading >= Self::WIDTH as u32 {
            None
        } else {
            Some(leading as usize)
        }
    }

    #[inline(always)]
    fn index_of_first_truthy(self) -> Option<usize> {
        let trailing = self.get_mask().trailing_zeros();
        if trailing >= Self::WIDTH as u32 {
            None
        } else {
            Some(trailing as usize)
        }
    }

    #[inline(always)]
    fn index_of_first_falsy(self) -> Option<usize> {
        let trailing = self.get_mask().trailing_ones();
        if trailing >= Self::WIDTH as u32 {
            None
        } else {
            Some(trailing as usize)
        }
    }

    #[inline(always)]
    fn index_of_first_eq(self, value: i8) -> Option<usize> {
        let value = Self::set1(value);
        let mask = self.cmp_eq(value);
        mask.index_of_first_truthy()
    }
}

pub trait SimdInt16: SimdInt<Scalar = i16, HorizontalAddScalar = i64> + SimdTransmuteI16 {
    fn extend_to_i32(self) -> (<Self::Backend as Simd>::Vi32, <Self::Backend as Simd>::Vi32);

    fn unsigned_extend_to_i32(self) -> (<Self::Backend as Simd>::Vi32, <Self::Backend as Simd>::Vi32);

    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Backend as Simd>::Vi32 {
        let (a, b) = self.extend_to_i32();
        a + b
    }

    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Backend as Simd>::Vi32 {
        let (a, b) = self.unsigned_extend_to_i32();
        a + b
    }
}

pub trait SimdInt32: SimdInt<Scalar = i32, HorizontalAddScalar = i64> + SimdTransmuteI32 {
    fn bitcast_f32(self) -> <Self::Backend as Simd>::Vf32;

    fn cast_f32(self) -> <Self::Backend as Simd>::Vf32;

    fn extend_to_i64(self) -> (<Self::Backend as Simd>::Vi64, <Self::Backend as Simd>::Vi64);

    fn unsigned_extend_to_i64(self) -> (<Self::Backend as Simd>::Vi64, <Self::Backend as Simd>::Vi64);

    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Backend as Simd>::Vi64 {
        let (a, b) = self.extend_to_i64();
        a + b
    }

    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Backend as Simd>::Vi64 {
        let (a, b) = self.unsigned_extend_to_i64();
        a + b
    }
}

pub trait SimdInt64: SimdInt<Scalar = i64, HorizontalAddScalar = i64> + SimdTransmuteI64 {
    fn bitcast_f64(self) -> <Self::Backend as Simd>::Vf64;

    fn cast_f64(self) -> <Self::Backend as Simd>::Vf64;

    fn partial_horizontal_add(self) -> i64;
}

pub trait SimdFloat: SimdBaseOps + Div<Self, Output = Self> + DivAssign<Self> + Div<Self::Scalar, Output = Self> + DivAssign<Self::Scalar>
where
    Self::Scalar:,
{
    fn div(self, rhs: Self) -> Self;

    fn ceil(self) -> Self;

    fn floor(self) -> Self;

    fn round(self) -> Self;

    fn fast_ceil(self) -> Self;

    fn fast_floor(self) -> Self;

    fn fast_round(self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;

    fn mul_sub(self, a: Self, b: Self) -> Self;

    fn neg_mul_add(self, a: Self, b: Self) -> Self;

    fn neg_mul_sub(self, a: Self, b: Self) -> Self;

    fn sqrt(self) -> Self;

    fn rsqrt(self) -> Self;

    fn from_f64(value: f64) -> Self;
}

pub trait SimdFloat32: SimdFloat<Scalar = f32, HorizontalAddScalar = f32> + SimdTransmuteF32 {
    fn bitcast_i32(self) -> <Self::Backend as Simd>::Vi32;

    fn cast_i32(self) -> <Self::Backend as Simd>::Vi32;

    fn fast_inverse(self) -> Self;
}

pub trait SimdFloat64: SimdFloat<Scalar = f64, HorizontalAddScalar = f64> + SimdTransmuteF64 {
    fn bitcast_i64(self) -> <Self::Backend as Simd>::Vi64;

    fn cast_i64(self) -> <Self::Backend as Simd>::Vi64;
}
