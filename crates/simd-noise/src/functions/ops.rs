use crate::simd::{Simd, SimdBaseIo, SimdConsts};

#[inline(always)]
pub unsafe fn gather_32<S: Simd>(arr: &[i32], indices: S::I32) -> S::I32 {
    let width = S::I32::WIDTH;
    let mut dst = S::I32::zeroes();
    for i in 0..width {
        *dst.get_unchecked_mut(i) = *arr.get_unchecked(indices[i] as usize);
    }
    dst
}

#[inline(always)]
pub unsafe fn gather_64<S: Simd>(arr: &[i64], indices: S::I64) -> S::I64 {
    let width = S::I64::WIDTH;
    let mut dst = S::I64::zeroes();
    for i in 0..width {
        *dst.get_unchecked_mut(i) = *arr.get_unchecked(indices[i] as usize);
    }
    dst
}
