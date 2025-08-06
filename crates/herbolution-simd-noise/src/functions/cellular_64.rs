use crate::simd::*;

#[inline(always)]
pub fn hash_2d<S: Simd>(seed: i64, x: S::I64, y: S::I64) -> S::I64 {
    let mut hash = x ^ S::I64::set1(seed);
    hash = y ^ hash;
    ((hash * hash) * S::I64::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_3d<S: Simd>(seed: i64, x: S::I64, y: S::I64, z: S::I64) -> S::I64 {
    let mut hash = x ^ S::I64::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    ((hash * hash) * S::I64::set1(60493)) * hash
}
