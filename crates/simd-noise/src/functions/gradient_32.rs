use crate::functions::hash3d_32::hash3d;
use crate::simd::{Simd, SimdBaseIo, SimdBaseOps, SimdI32};

#[inline(always)]
pub fn grad1<S: Simd>(seed: i64, hash: S::I32) -> S::F32 {
    let h = (S::I32::set1(seed as i32) ^ hash) & S::I32::set1(15);
    let v = (h & S::I32::set1(7)).cast_f32();

    let h_and_8 = ((h & S::I32::set1(8)).cmp_eq(S::I32::zeroes())).bitcast_f32();
    h_and_8.blendv(S::F32::zeroes() - v, v)
}

#[inline(always)]
pub fn grad2<S: Simd>(seed: i64, hash: S::I32) -> [S::F32; 2] {
    let h = (hash ^ S::I32::set1(seed as i32)) & S::I32::set1(7);
    let mask = (S::I32::set1(4).cmp_gt(h)).bitcast_f32();
    let x_magnitude = mask.blendv(S::F32::set1(2.0), S::F32::set1(1.0));
    let y_magnitude = mask.blendv(S::F32::set1(1.0), S::F32::set1(2.0));

    let h_and_1 = ((h & S::I32::set1(1)).cmp_eq(S::I32::zeroes())).bitcast_f32();
    let h_and_2 = ((h & S::I32::set1(2)).cmp_eq(S::I32::zeroes())).bitcast_f32();

    let gx = mask
        .blendv(h_and_2, h_and_1)
        .blendv(S::F32::zeroes() - x_magnitude, x_magnitude);
    let gy = mask
        .blendv(h_and_1, h_and_2)
        .blendv(S::F32::zeroes() - y_magnitude, y_magnitude);
    [gx, gy]
}

#[inline(always)]
pub fn grad3d_dot<S: Simd>(seed: i64, i: S::I32, j: S::I32, k: S::I32, x: S::F32, y: S::F32, z: S::F32) -> S::F32 {
    let h = hash3d::<S>(seed, i, j, k);
    let u = h.l8.blendv(y, x);
    let v = h.l4.blendv(h.h12_or_14.blendv(z, x), y);
    let result = (u ^ h.h1) + (v ^ h.h2);
    debug_assert_eq!(
        result[0],
        {
            let [gx, gy, gz] = grad3d::<S>(seed, i, j, k);
            gx * x + gy * y + gz * z
        }[0],
        "results match"
    );
    result
}

pub fn grad3d<S: Simd>(seed: i64, i: S::I32, j: S::I32, k: S::I32) -> [S::F32; 3] {
    let h = hash3d::<S>(seed, i, j, k);

    let first = S::F32::set1(1.0) | h.h1;
    let mut gx = h.l8 & first;
    let mut gy = first.and_not(h.l8);

    let second = S::F32::set1(1.0) | h.h2;
    gy = h.l4.blendv(gy, second);
    gx = h.h12_or_14.and_not(h.l4).blendv(gx, second);
    let gz = second.and_not(h.h12_or_14 | h.l4);
    debug_assert_eq!(gx[0].abs() + gy[0].abs() + gz[0].abs(), 2.0, "exactly two axes are chosen");
    [gx, gy, gz]
}

#[inline(always)]
pub fn grad4<S: Simd>(seed: i64, hash: S::I32, x: S::F32, y: S::F32, z: S::F32, t: S::F32) -> S::F32 {
    let h = (S::I32::set1(seed as i32) ^ hash) & S::I32::set1(31);
    let mut mask = (S::I32::set1(24).cmp_gt(h)).bitcast_f32();
    let u = mask.blendv(y, x);
    mask = (S::I32::set1(16).cmp_gt(h)).bitcast_f32();
    let v = mask.blendv(z, y);
    mask = (S::I32::set1(8).cmp_gt(h)).bitcast_f32();
    let w = mask.blendv(t, z);

    let h_and_1 = ((h & S::I32::set1(1)).cmp_eq(S::I32::zeroes())).bitcast_f32();
    let h_and_2 = ((h & S::I32::set1(2)).cmp_eq(S::I32::zeroes())).bitcast_f32();
    let h_and_4 = ((h & S::I32::set1(4)).cmp_eq(S::I32::zeroes())).bitcast_f32();

    h_and_1.blendv(S::F32::zeroes() - u, u) + h_and_2.blendv(S::F32::zeroes() - v, v) + h_and_4.blendv(S::F32::zeroes() - w, w)
}
