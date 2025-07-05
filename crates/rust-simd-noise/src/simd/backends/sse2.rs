use core::mem;

use crate::Simd;

pub struct Sse2;
impl Simd for Sse2 {
    type Vi8 = I8x16;
    type Vi16 = I16x8;
    type Vi32 = I32x4;
    type Vf32 = F32x4;
    type Vf64 = F64x2;
    type Vi64 = I64x2;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "sse2")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }

    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x2(_mm_castps_pd(a.0))
    }

    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x4(_mm_castpd_ps(a.0))
    }

    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        let index_as_arr = mem::transmute::<I32x4, [i32; 4]>(index);
        I32x4(_mm_set_epi32(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    #[inline(always)]
    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        let index_as_arr = mem::transmute::<I64x2, [i64; 2]>(index);
        I64x2(_mm_set_epi64x(arr[index_as_arr[1] as usize], arr[index_as_arr[0] as usize]))
    }

    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<I32x4, [i32; 4]>(index);
        F32x4(_mm_set_ps(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let mut result = I32x4(_mm_setzero_si128());
        let ptr = mem_addr as *const i32;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let mut result = I64x2(_mm_setzero_si128());
        let ptr = mem_addr as *const i64;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let mut result = F32x4(_mm_setzero_ps());
        let ptr = mem_addr as *const f32;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0.0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0.0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let mut result = F64x2(_mm_setzero_pd());
        let ptr = mem_addr as *const f64;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result
    }

    #[inline(always)]
    unsafe fn shuffle_epi32<const IMM8: i32>(a: Self::Vi32) -> Self::Vi32 {
        I32x4(_mm_shuffle_epi32(a.0, IMM8))
    }
}

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::ops::*;

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
