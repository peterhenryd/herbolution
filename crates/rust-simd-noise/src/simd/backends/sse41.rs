use crate::simd::{Simd, SimdFloat32, SimdFloat64, SimdInt16, SimdInt32, SimdInt64, SimdInt8};
use crate::{
    define_simd_type, impl_f32_simd_type, impl_f64_simd_type, impl_i16_simd_type, impl_i32_simd_type, impl_i64_simd_type, impl_i8_simd_type,
    impl_simd_float_overloads, impl_simd_int_overloads,
};
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub struct Sse41;
impl Simd for Sse41 {
    type Vi8 = I8x16_41;
    type Vi16 = I16x8_41;
    type Vi32 = I32x4_41;
    type Vf32 = F32x4_41;
    type Vf64 = F64x2_41;
    type Vi64 = I64x2_41;
}

define_simd_type!(Sse41, i8, 16, __m128i, _41);
impl_simd_int_overloads!(I8x16_41);
impl_i8_simd_type!(Sse41, I8x16_41, I16x8_41);

define_simd_type!(Sse41, i16, 8, __m128i, _41);
impl_simd_int_overloads!(I16x8_41);
impl_i16_simd_type!(Sse41, I16x8_41, I32x4_41);

define_simd_type!(Sse41, i32, 4, __m128i, _41);
impl_simd_int_overloads!(I32x4_41);
impl_i32_simd_type!(Sse41, I32x4_41, F32x4_41, I64x2_41);

define_simd_type!(Sse41, i64, 2, __m128i, _41);
impl_simd_int_overloads!(I64x2_41);
impl_i64_simd_type!(Sse41, I64x2_41, F64x2_41);

define_simd_type!(Sse41, f32, 4, __m128, _41);
impl_simd_float_overloads!(F32x4_41);
impl_f32_simd_type!(Sse41, F32x4_41, I32x4_41);

define_simd_type!(Sse41, f64, 2, __m128d, _41);
impl_simd_float_overloads!(F64x2_41);
impl_f64_simd_type!(Sse41, F64x2_41, I64x2_41);
