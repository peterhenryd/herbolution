use std::arch::aarch64::{float32x4_t, float64x2_t, int16x8_t, int32x4_t, int64x2_t, int8x16_t};

use crate::simd::{Simd, SimdBaseIo, SimdI16, SimdI32, SimdI64, SimdI8};
use crate::{
    define_simd_type, impl_f32_simd_type, impl_f64_simd_type, impl_i16_simd_type, impl_i32_simd_type, impl_i64_simd_type, impl_i8_simd_type,
    impl_simd_float_overloads, impl_simd_int_overloads,
};

pub struct Neon;
impl Simd for Neon {
    type I8 = I8x16Neon;
    type I16 = I16x8Neon;
    type I32 = I32x4Neon;
    type I64 = I64x2Neon;
    type F32 = F32x4Neon;
    type F64 = F64x2Neon;
}

use crate::simd::{SimdTransmuteF32, SimdTransmuteF64, SimdTransmuteI16, SimdTransmuteI32, SimdTransmuteI64, SimdTransmuteI8};

define_simd_type!(Neon, i8, 16, int8x16_t, Neon);
impl_simd_int_overloads!(I8x16Neon);
impl_i8_simd_type!(Neon, I8x16Neon, I16x8Neon);

define_simd_type!(Neon, i16, 8, int16x8_t, Neon);
impl_simd_int_overloads!(I16x8Neon);
impl_i16_simd_type!(Neon, I16x8Neon, I32x4Neon);

define_simd_type!(Neon, i32, 4, int32x4_t, Neon);
impl_simd_int_overloads!(I32x4Neon);
impl_i32_simd_type!(Neon, I32x4Neon, F32x4Neon, I64x2Neon);

define_simd_type!(Neon, i64, 2, int64x2_t, Neon);
impl_simd_int_overloads!(I64x2Neon);
impl_i64_simd_type!(Neon, I64x2Neon, F64x2Neon);

define_simd_type!(Neon, f32, 4, float32x4_t, Neon);
impl_simd_float_overloads!(F32x4Neon);
impl_f32_simd_type!(Neon, F32x4Neon, I32x4Neon);

define_simd_type!(Neon, f64, 2, float64x2_t, Neon);
impl_simd_float_overloads!(F64x2Neon);
impl_f64_simd_type!(Neon, F64x2Neon, I64x2Neon);
