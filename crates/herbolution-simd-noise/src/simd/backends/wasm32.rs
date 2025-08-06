#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;

use crate::simd::Simd;
use crate::{
    define_simd_type, impl_f32_simd_type, impl_f64_simd_type, impl_i16_simd_type, impl_i32_simd_type, impl_i64_simd_type, impl_i8_simd_type,
    impl_simd_float_overloads, impl_simd_int_overloads,
};

pub struct Wasm;

impl Simd for Wasm {
    type I8 = I8x16Wasm;
    type I16 = I16x8Wasm;
    type I32 = I32x4Wasm;
    type F32 = F32x4Wasm;
    type F64 = F64x2Wasm;
    type I64 = I64x2Wasm;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "simd128")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }
}

define_simd_type!(Wasm, i8, 16, v128, Wasm);
impl_simd_int_overloads!(I8x16Wasm);
impl_i8_simd_type!(Wasm, I8x16Wasm, I16x8Wasm);

define_simd_type!(Wasm, i16, 8, v128, Wasm);
impl_simd_int_overloads!(I16x8Wasm);
impl_i16_simd_type!(Wasm, I16x8Wasm, I32x4Wasm);

define_simd_type!(Wasm, i32, 4, v128, Wasm);
impl_simd_int_overloads!(I32x4Wasm);
impl_i32_simd_type!(Wasm, I32x4Wasm, F32x4Wasm, I64x2Wasm);

define_simd_type!(Wasm, i64, 2, v128, Wasm);
impl_simd_int_overloads!(I64x2Wasm);
impl_i64_simd_type!(Wasm, I64x2Wasm, F64x2Wasm);

define_simd_type!(Wasm, f32, 4, v128, Wasm);
impl_simd_float_overloads!(F32x4Wasm);
impl_f32_simd_type!(Wasm, F32x4Wasm, I32x4Wasm);

define_simd_type!(Wasm, f64, 2, v128, Wasm);
impl_simd_float_overloads!(F64x2Wasm);
impl_f64_simd_type!(Wasm, F64x2Wasm, I64x2Wasm);
