use crate::functions::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};
use crate::simd::{Simd, SimdBaseIo, SimdBaseOps};

#[inline(always)]
pub fn turbulence_1d<S: Simd>(mut x: S::F64, lacunarity: S::F64, gain: S::F64, octaves: u8, seed: i64) -> S::F64 {
    let mut amp = S::F64::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        amp = amp * gain;
        result = result + simplex_1d::<S>(x, seed).abs();
    }

    result
}

#[inline(always)]
pub fn turbulence_2d<S: Simd>(mut x: S::F64, mut y: S::F64, lac: S::F64, gain: S::F64, octaves: u8, seed: i64) -> S::F64 {
    let mut result = simplex_2d::<S>(x, y, seed).abs();

    let mut amp = S::F64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        amp = amp * gain;
        result = result + (simplex_2d::<S>(x, y, seed) * amp).abs();
    }

    result
}

#[inline(always)]
pub fn turbulence_3d<S: Simd>(mut x: S::F64, mut y: S::F64, mut z: S::F64, lac: S::F64, gain: S::F64, octaves: u8, seed: i64) -> S::F64 {
    let mut result = simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::F64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        amp = amp * gain;
        result = result + (simplex_3d::<S>(x, y, z, seed) * amp).abs();
    }

    result
}

#[inline(always)]
pub fn turbulence_4d<S: Simd>(mut x: S::F64, mut y: S::F64, mut z: S::F64, mut w: S::F64, lac: S::F64, gain: S::F64, octaves: u8, seed: i64) -> S::F64 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed).abs();
    let mut amp = S::F64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        w = w * lac;
        amp = amp * gain;
        result = result + (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    result
}
