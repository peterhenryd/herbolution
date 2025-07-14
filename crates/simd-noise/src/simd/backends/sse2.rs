pub struct Sse2;
impl Simd for Sse2 {
    type I8 = I8x16;
    type I16 = I16x8;
    type I32 = I32x4;
    type I64 = I64x2;
    type F32 = F32x4;
    type F64 = F64x2;
}

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
  use crate::{define_simd_type, impl_f32_simd_type, impl_f64_simd_type, impl_i16_simd_type, impl_i32_simd_type, impl_i64_simd_type, impl_i8_simd_type, impl_simd_float_overloads, impl_simd_int_overloads};
use crate::simd::*;

define_simd_type!(Sse2, i8, 16, __m128i);
impl_simd_int_overloads!(I8x16);
impl_i8_simd_type!(Sse2, I8x16, I16x8);

define_simd_type!(Sse2, i16, 8, __m128i);
impl_simd_int_overloads!(I16x8);
impl_i16_simd_type!(Sse2, I16x8, I32x4);

define_simd_type!(Sse2, i32, 4, __m128i);
impl_simd_int_overloads!(I32x4);
impl_i32_simd_type!(Sse2, I32x4, F32x4, I64x2);

define_simd_type!(Sse2, i64, 2, __m128i);
impl_simd_int_overloads!(I64x2);
impl_i64_simd_type!(Sse2, I64x2, F64x2);

define_simd_type!(Sse2, f32, 4, __m128);
impl_simd_float_overloads!(F32x4);
impl_f32_simd_type!(Sse2, F32x4, I32x4);

define_simd_type!(Sse2, f64, 2, __m128d);
impl_simd_float_overloads!(F64x2);
impl_f64_simd_type!(Sse2, F64x2, I64x2);
