use std::arch::x86_64::{__m256, __m256d, __m256i};

use crate::simd::*;
use crate::{
    define_simd_type, impl_f32_simd_type, impl_f64_simd_type, impl_i16_simd_type, impl_i32_simd_type, impl_i64_simd_type, impl_i8_simd_type,
    impl_simd_float_overloads, impl_simd_int_overloads,
};

pub struct Avx2;

impl Simd for Avx2 {
    type I8 = I8x32;
    type I16 = I16x16;
    type I32 = I32x8;
    type I64 = I64x4;
    type F32 = F32x8;
    type F64 = F64x4;
}

define_simd_type!(Avx2, i8, 32, __m256i);
impl_simd_int_overloads!(I8x32);
impl_i8_simd_type!(Avx2, I8x32, I16x16);

define_simd_type!(Avx2, i16, 16, __m256i);
impl_simd_int_overloads!(I16x16);
impl_i16_simd_type!(Avx2, I16x16, I32x8);

define_simd_type!(Avx2, i32, 8, __m256i);
impl_simd_int_overloads!(I32x8);
impl_i32_simd_type!(Avx2, I32x8, F32x8, I64x4);

define_simd_type!(Avx2, i64, 4, __m256i);
impl_simd_int_overloads!(I64x4);
impl_i64_simd_type!(Avx2, I64x4, F64x4);

define_simd_type!(Avx2, f32, 8, __m256);
impl_simd_float_overloads!(F32x8);
impl_f32_simd_type!(Avx2, F32x8, I32x8);

define_simd_type!(Avx2, f64, 4, __m256d);
impl_simd_float_overloads!(F64x4);
impl_f64_simd_type!(Avx2, F64x4, I64x4);
