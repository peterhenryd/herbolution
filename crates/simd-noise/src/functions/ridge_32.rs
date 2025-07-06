use crate::functions::simplex_32::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};
use crate::simd::{Simd, SimdBaseIo, SimdBaseOps, SimdFloat};

#[inline(always)]
pub fn ridge_1d<S: Simd>(mut x: S::F32, lacunarity: S::F32, gain: S::F32, octaves: u8, seed: i64) -> S::F32 {
    let mut amp = S::F32::set1(1.0);
    let mut result = S::F32::set1(1.0) - simplex_1d::<S>(x, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        amp = amp * gain;
        result = result + S::F32::set1(1.0) - simplex_1d::<S>(x, seed).abs();
    }

    result
}

#[inline(always)]
pub fn ridge_2d<S: Simd>(mut x: S::F32, mut y: S::F32, lac: S::F32, gain: S::F32, octaves: u8, seed: i64) -> S::F32 {
    let mut result = S::F32::set1(1.0) - simplex_2d::<S>(x, y, seed).abs();
    let mut amp = S::F32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        amp = amp * gain;
        result = result + S::F32::neg_mul_add(simplex_2d::<S>(x, y, seed).abs(), amp, S::F32::set1(1.0));
    }

    result
}

#[inline(always)]
pub fn ridge_3d<S: Simd>(mut x: S::F32, mut y: S::F32, mut z: S::F32, lac: S::F32, gain: S::F32, octaves: u8, seed: i64) -> S::F32 {
    let mut result = S::F32::set1(1.0) - simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::F32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        amp = amp * gain;
        result = result + S::F32::neg_mul_add(simplex_3d::<S>(x, y, z, seed).abs(), amp, S::F32::set1(1.0));
    }

    result
}

#[inline(always)]
pub fn ridge_4d<S: Simd>(mut x: S::F32, mut y: S::F32, mut z: S::F32, mut w: S::F32, lac: S::F32, gain: S::F32, octaves: u8, seed: i64) -> S::F32 {
    let mut result = S::F32::set1(1.0) - simplex_4d::<S>(x, y, z, w, seed).abs();
    let mut amp = S::F32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        w = w * lac;
        amp = amp * gain;
        result = result + S::F32::set1(1.0) - (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    result
}
