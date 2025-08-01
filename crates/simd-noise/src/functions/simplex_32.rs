use crate::functions::cellular_32::{X_PRIME_32, Y_PRIME_32, Z_PRIME_32};
use crate::functions::gradient_32::{grad1, grad2, grad3d, grad3d_dot, grad4};
use crate::functions::ops::gather_32;
use crate::simd::{Simd, SimdBaseIo, SimdBaseOps, SimdF32, SimdFloat, SimdI32, SimdIter};

const F2_32: f32 = 0.36602540378;
pub const F2_64: f64 = 0.36602540378;
const F3_32: f32 = 1.0 / 3.0;
pub const F3_64: f64 = 1.0 / 3.0;
const F4_32: f32 = 0.309016994;
pub const F4_64: f64 = 0.309016994;
const G2_32: f32 = 0.2113248654;
pub const G2_64: f64 = 0.2113248654;
const G22_32: f32 = G2_32 * 2.0;
pub const G22_64: f64 = G2_64 * 2.0;
const G3_32: f32 = 1.0 / 6.0;
pub const G3_64: f64 = 1.0 / 6.0;
const G33_32: f32 = 3.0 / 6.0 - 1.0;
pub const G33_64: f64 = 3.0 / 6.0 - 1.0;
const G4_32: f32 = 0.138196601;
pub const G4_64: f64 = 0.138196601;
const G24_32: f32 = 2.0 * G4_32;
pub const G24_64: f64 = 2.0 * G4_64;
const G34_32: f32 = 3.0 * G4_32;
pub const G34_64: f64 = 3.0 * G4_64;
const G44_32: f32 = 4.0 * G4_32;
pub const G44_64: f64 = 4.0 * G4_64;

static PERM: [i32; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48,
    27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73,
    209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
    147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153,
    101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144,
    12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
    138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233,
    7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177,
    33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133,
    230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116,
    188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58,
    17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79,
    113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
    192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195,
    78, 66, 215, 61, 156, 180,
];

#[inline(always)]
fn assert_in_perm_range<S: Simd>(values: S::I32) {
    debug_assert!(values
        .cmp_lt(S::I32::set1(PERM.len() as i32))
        .iter()
        .all(|is_less_than| is_less_than != 0));
}

#[inline(always)]
pub fn simplex_1d_deriv<S: Simd>(x: S::F32, seed: i64) -> (S::F32, S::F32) {
    let ips = x.fast_floor();
    let mut i0 = ips.cast_i32();
    let i1 = (i0 + S::I32::set1(1)) & S::I32::set1(0xff);

    let x0 = x - ips;
    let x1 = x0 - S::F32::set1(1.0);

    i0 = i0 & S::I32::set1(0xff);
    let (gi0, gi1) = unsafe {
        let gi0 = gather_32::<S>(&PERM, i0);
        let gi1 = gather_32::<S>(&PERM, i1);
        (gi0, gi1)
    };

    let x20 = x0 * x0;
    let t0 = S::F32::set1(1.0) - x20;
    let t20 = t0 * t0;
    let t40 = t20 * t20;
    let gx0 = grad1::<S>(seed, gi0);
    let n0 = t40 * gx0 * x0;

    let x21 = x1 * x1;
    let t1 = S::F32::set1(1.0) - x21;
    let t21 = t1 * t1;
    let t41 = t21 * t21;
    let gx1 = grad1::<S>(seed, gi1);
    let n1 = t41 * gx1 * x1;

    const SCALE: f32 = 256.0 / (81.0 * 7.0);

    let value = (n0 + n1) * S::F32::set1(SCALE);
    let derivative = ((t20 * t0 * gx0 * x20 + t21 * t1 * gx1 * x21) * S::F32::set1(-8.0) + t40 * gx0 + t41 * gx1) * S::F32::set1(SCALE);
    (value, derivative)
}

#[inline(always)]
pub fn simplex_1d<S: Simd>(x: S::F32, seed: i64) -> S::F32 {
    simplex_1d_deriv::<S>(x, seed).0
}

#[inline(always)]
pub fn simplex_2d<S: Simd>(x: S::F32, y: S::F32, seed: i64) -> S::F32 {
    simplex_2d_deriv::<S>(x, y, seed).0
}

#[inline(always)]
pub fn simplex_2d_deriv<S: Simd>(x: S::F32, y: S::F32, seed: i64) -> (S::F32, [S::F32; 2]) {
    let s = S::F32::set1(F2_32) * (x + y);
    let ips = (x + s).floor();
    let jps = (y + s).floor();

    let i = ips.cast_i32();
    let j = jps.cast_i32();

    let t = (i + j).cast_f32() * S::F32::set1(G2_32);

    let x0 = x - (ips - t);
    let y0 = y - (jps - t);

    let i1 = (x0.cmp_gte(y0)).bitcast_i32();

    let j1 = (y0.cmp_gt(x0)).bitcast_i32();

    let x1 = (x0 + i1.cast_f32()) + S::F32::set1(G2_32);
    let y1 = (y0 + j1.cast_f32()) + S::F32::set1(G2_32);
    let x2 = (x0 + S::F32::set1(-1.0)) + S::F32::set1(G22_32);
    let y2 = (y0 + S::F32::set1(-1.0)) + S::F32::set1(G22_32);

    let ii = i & S::I32::set1(0xff);
    let jj = j & S::I32::set1(0xff);

    let (gi0, gi1, gi2) = unsafe {
        assert_in_perm_range::<S>(ii);
        assert_in_perm_range::<S>(jj);
        assert_in_perm_range::<S>(ii - i1);
        assert_in_perm_range::<S>(jj - j1);
        assert_in_perm_range::<S>(ii + 1);
        assert_in_perm_range::<S>(jj + 1);

        let gi0 = gather_32::<S>(&PERM, ii + gather_32::<S>(&PERM, jj));
        let gi1 = gather_32::<S>(&PERM, (ii - i1) + gather_32::<S>(&PERM, jj - j1));
        let gi2 = gather_32::<S>(&PERM, (ii - S::I32::set1(-1)) + gather_32::<S>(&PERM, jj - S::I32::set1(-1)));

        (gi0, gi1, gi2)
    };

    let mut t0 = S::F32::neg_mul_add(y0, y0, S::F32::neg_mul_add(x0, x0, S::F32::set1(0.5)));
    let mut t1 = S::F32::neg_mul_add(y1, y1, S::F32::neg_mul_add(x1, x1, S::F32::set1(0.5)));
    let mut t2 = S::F32::neg_mul_add(y2, y2, S::F32::neg_mul_add(x2, x2, S::F32::set1(0.5)));

    t0 &= t0.cmp_gte(S::F32::zeroes());
    t1 &= t1.cmp_gte(S::F32::zeroes());
    t2 &= t2.cmp_gte(S::F32::zeroes());

    let t20 = t0 * t0;
    let t40 = t20 * t20;
    let t21 = t1 * t1;
    let t41 = t21 * t21;
    let t22 = t2 * t2;
    let t42 = t22 * t22;

    let [gx0, gy0] = grad2::<S>(seed, gi0);
    let g0 = gx0 * x0 + gy0 * y0;
    let n0 = t40 * g0;
    let [gx1, gy1] = grad2::<S>(seed, gi1);
    let g1 = gx1 * x1 + gy1 * y1;
    let n1 = t41 * g1;
    let [gx2, gy2] = grad2::<S>(seed, gi2);
    let g2 = gx2 * x2 + gy2 * y2;
    let n2 = t42 * g2;

    let scale = S::F32::set1(45.26450774985561631259);
    let value = (n0 + (n1 + n2)) * scale;
    let derivative = {
        let temp0 = t20 * t0 * g0;
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let temp1 = t21 * t1 * g1;
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        let temp2 = t22 * t2 * g2;
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dx *= S::F32::set1(-8.0);
        dnoise_dy *= S::F32::set1(-8.0);
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2;
        dnoise_dx *= scale;
        dnoise_dy *= scale;
        [dnoise_dx, dnoise_dy]
    };
    (value, derivative)
}

#[inline(always)]
pub fn simplex_3d<S: Simd>(x: S::F32, y: S::F32, z: S::F32, seed: i64) -> S::F32 {
    simplex_3d_deriv::<S>(x, y, z, seed).0
}

#[inline(always)]
pub fn simplex_3d_deriv<S: Simd>(x: S::F32, y: S::F32, z: S::F32, seed: i64) -> (S::F32, [S::F32; 3]) {
    let f = S::F32::set1(F3_32) * ((x + y) + z);
    let mut x0 = (x + f).fast_floor();
    let mut y0 = (y + f).fast_floor();
    let mut z0 = (z + f).fast_floor();

    let i = x0.cast_i32() * S::I32::set1(X_PRIME_32);
    let j = y0.cast_i32() * S::I32::set1(Y_PRIME_32);
    let k = z0.cast_i32() * S::I32::set1(Z_PRIME_32);

    let g = S::F32::set1(G3_32) * ((x0 + y0) + z0);
    x0 = x - (x0 - g);
    y0 = y - (y0 - g);
    z0 = z - (z0 - g);

    let x0_ge_y0 = x0.cmp_gte(y0);
    let y0_ge_z0 = y0.cmp_gte(z0);
    let x0_ge_z0 = x0.cmp_gte(z0);

    let i1 = x0_ge_y0 & x0_ge_z0;
    let j1 = y0_ge_z0.and_not(x0_ge_y0);
    let k1 = (!y0_ge_z0).and_not(x0_ge_z0);

    let i2 = x0_ge_y0 | x0_ge_z0;
    let j2 = (!x0_ge_y0) | y0_ge_z0;
    let k2 = !(x0_ge_z0 & y0_ge_z0);

    let x1 = x0 - (i1 & S::F32::set1(1.0)) + S::F32::set1(G3_32);
    let y1 = y0 - (j1 & S::F32::set1(1.0)) + S::F32::set1(G3_32);
    let z1 = z0 - (k1 & S::F32::set1(1.0)) + S::F32::set1(G3_32);

    let x2 = x0 - (i2 & S::F32::set1(1.0)) + S::F32::set1(F3_32);
    let y2 = y0 - (j2 & S::F32::set1(1.0)) + S::F32::set1(F3_32);
    let z2 = z0 - (k2 & S::F32::set1(1.0)) + S::F32::set1(F3_32);

    let x3 = x0 + S::F32::set1(G33_32);
    let y3 = y0 + S::F32::set1(G33_32);
    let z3 = z0 + S::F32::set1(G33_32);

    let mut t0 = S::F32::set1(0.6) - (x0 * x0) - (y0 * y0) - (z0 * z0);
    let mut t1 = S::F32::set1(0.6) - (x1 * x1) - (y1 * y1) - (z1 * z1);
    let mut t2 = S::F32::set1(0.6) - (x2 * x2) - (y2 * y2) - (z2 * z2);
    let mut t3 = S::F32::set1(0.6) - (x3 * x3) - (y3 * y3) - (z3 * z3);

    t0 &= t0.cmp_gte(S::F32::zeroes());
    t1 &= t1.cmp_gte(S::F32::zeroes());
    t2 &= t2.cmp_gte(S::F32::zeroes());
    t3 &= t3.cmp_gte(S::F32::zeroes());

    let t20 = t0 * t0;
    let t21 = t1 * t1;
    let t22 = t2 * t2;
    let t23 = t3 * t3;

    let t40 = t20 * t20;
    let t41 = t21 * t21;
    let t42 = t22 * t22;
    let t43 = t23 * t23;

    let g0 = grad3d_dot::<S>(seed, i, j, k, x0, y0, z0);
    let v0 = t40 * g0;

    let v1x = i + (i1.bitcast_i32() & S::I32::set1(X_PRIME_32));
    let v1y = j + (j1.bitcast_i32() & S::I32::set1(Y_PRIME_32));
    let v1z = k + (k1.bitcast_i32() & S::I32::set1(Z_PRIME_32));
    let g1 = grad3d_dot::<S>(seed, v1x, v1y, v1z, x1, y1, z1);
    let v1 = t41 * g1;

    let v2x = i + (i2.bitcast_i32() & S::I32::set1(X_PRIME_32));
    let v2y = j + (j2.bitcast_i32() & S::I32::set1(Y_PRIME_32));
    let v2z = k + (k2.bitcast_i32() & S::I32::set1(Z_PRIME_32));
    let g2 = grad3d_dot::<S>(seed, v2x, v2y, v2z, x2, y2, z2);
    let v2 = t42 * g2;

    let v3x = i + S::I32::set1(X_PRIME_32);
    let v3y = j + S::I32::set1(Y_PRIME_32);
    let v3z = k + S::I32::set1(Z_PRIME_32);
    let g3 = grad3d_dot::<S>(seed, v3x, v3y, v3z, x3, y3, z3);
    let v3 = t43 * g3;

    let p1 = v3 + v2;
    let p2 = p1 + v1;

    let scale = S::F32::set1(32.69587493801679);
    let result = (p2 + v0) * scale;
    let derivative = {
        let temp0 = t20 * t0 * g0;
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let mut dnoise_dz = temp0 * z0;
        let temp1 = t21 * t1 * g1;
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        dnoise_dz += temp1 * z1;
        let temp2 = t22 * t2 * g2;
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dz += temp2 * z2;
        let temp3 = t23 * t3 * g3;
        dnoise_dx += temp3 * x3;
        dnoise_dy += temp3 * y3;
        dnoise_dz += temp3 * z3;
        dnoise_dx *= S::F32::set1(-8.0);
        dnoise_dy *= S::F32::set1(-8.0);
        dnoise_dz *= S::F32::set1(-8.0);
        let [gx0, gy0, gz0] = grad3d::<S>(seed, i, j, k);
        let [gx1, gy1, gz1] = grad3d::<S>(seed, v1x, v1y, v1z);
        let [gx2, gy2, gz2] = grad3d::<S>(seed, v2x, v2y, v2z);
        let [gx3, gy3, gz3] = grad3d::<S>(seed, v3x, v3y, v3z);
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2 + t43 * gx3;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2 + t43 * gy3;
        dnoise_dz += t40 * gz0 + t41 * gz1 + t42 * gz2 + t43 * gz3;
        dnoise_dx *= scale;
        dnoise_dy *= scale;
        dnoise_dz *= scale;
        [dnoise_dx, dnoise_dy, dnoise_dz]
    };
    (result, derivative)
}

#[inline(always)]
pub fn simplex_4d<S: Simd>(x: S::F32, y: S::F32, z: S::F32, w: S::F32, seed: i64) -> S::F32 {
    let s = S::F32::set1(F4_32) * (x + y + z + w);

    let ips = (x + s).floor();
    let jps = (y + s).floor();
    let kps = (z + s).floor();
    let lps = (w + s).floor();

    let i = ips.cast_i32();
    let j = jps.cast_i32();
    let k = kps.cast_i32();
    let l = lps.cast_i32();

    let t = (i + j + k + l).cast_f32() * S::F32::set1(G4_32);
    let x0 = x - (ips - t);
    let y0 = y - (jps - t);
    let z0 = z - (kps - t);
    let w0 = w - (lps - t);

    let mut rank_x = S::I32::zeroes();
    let mut rank_y = S::I32::zeroes();
    let mut rank_z = S::I32::zeroes();
    let mut rank_w = S::I32::zeroes();

    let cond = (x0.cmp_gt(y0)).bitcast_i32();
    rank_x = rank_x + (cond & S::I32::set1(1));
    rank_y = rank_y + S::I32::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(z0)).bitcast_i32();
    rank_x = rank_x + (cond & S::I32::set1(1));
    rank_z = rank_z + S::I32::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(w0)).bitcast_i32();
    rank_x = rank_x + (cond & S::I32::set1(1));
    rank_w = rank_w + S::I32::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(z0)).bitcast_i32();
    rank_y = rank_y + (cond & S::I32::set1(1));
    rank_z = rank_z + S::I32::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(w0)).bitcast_i32();
    rank_y = rank_y + (cond & S::I32::set1(1));
    rank_w = rank_w + S::I32::set1(1).and_not(cond);
    let cond = (z0.cmp_gt(w0)).bitcast_i32();
    rank_z = rank_z + (cond & S::I32::set1(1));
    rank_w = rank_w + S::I32::set1(1).and_not(cond);

    let cond = rank_x.cmp_gt(S::I32::set1(2));
    let i1 = S::I32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::I32::set1(2));
    let j1 = S::I32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::I32::set1(2));
    let k1 = S::I32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::I32::set1(2));
    let l1 = S::I32::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::I32::set1(1));
    let i2 = S::I32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::I32::set1(1));
    let j2 = S::I32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::I32::set1(1));
    let k2 = S::I32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::I32::set1(1));
    let l2 = S::I32::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::I32::zeroes());
    let i3 = S::I32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::I32::zeroes());
    let j3 = S::I32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::I32::zeroes());
    let k3 = S::I32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::I32::zeroes());
    let l3 = S::I32::set1(1) & cond;

    let x1 = x0 - i1.cast_f32() + S::F32::set1(G4_32);
    let y1 = y0 - j1.cast_f32() + S::F32::set1(G4_32);
    let z1 = z0 - k1.cast_f32() + S::F32::set1(G4_32);
    let w1 = w0 - l1.cast_f32() + S::F32::set1(G4_32);
    let x2 = x0 - i2.cast_f32() + S::F32::set1(G24_32);
    let y2 = y0 - j2.cast_f32() + S::F32::set1(G24_32);
    let z2 = z0 - k2.cast_f32() + S::F32::set1(G24_32);
    let w2 = w0 - l2.cast_f32() + S::F32::set1(G24_32);
    let x3 = x0 - i3.cast_f32() + S::F32::set1(G34_32);
    let y3 = y0 - j3.cast_f32() + S::F32::set1(G34_32);
    let z3 = z0 - k3.cast_f32() + S::F32::set1(G34_32);
    let w3 = w0 - l3.cast_f32() + S::F32::set1(G34_32);
    let x4 = x0 - S::F32::set1(1.0) + S::F32::set1(G44_32);
    let y4 = y0 - S::F32::set1(1.0) + S::F32::set1(G44_32);
    let z4 = z0 - S::F32::set1(1.0) + S::F32::set1(G44_32);
    let w4 = w0 - S::F32::set1(1.0) + S::F32::set1(G44_32);

    let ii = i & S::I32::set1(0xff);
    let jj = j & S::I32::set1(0xff);
    let kk = k & S::I32::set1(0xff);
    let ll = l & S::I32::set1(0xff);

    let (gi0, gi1, gi2, gi3, gi4) = unsafe {
        let lp = gather_32::<S>(&PERM, ll);
        let kp = gather_32::<S>(&PERM, kk + lp);
        let jp = gather_32::<S>(&PERM, jj + kp);
        let gi0 = gather_32::<S>(&PERM, ii + jp);

        let lp = gather_32::<S>(&PERM, ll + l1);
        let kp = gather_32::<S>(&PERM, kk + k1 + lp);
        let jp = gather_32::<S>(&PERM, jj + j1 + kp);
        let gi1 = gather_32::<S>(&PERM, ii + i1 + jp);

        let lp = gather_32::<S>(&PERM, ll + l2);
        let kp = gather_32::<S>(&PERM, kk + k2 + lp);
        let jp = gather_32::<S>(&PERM, jj + j2 + kp);
        let gi2 = gather_32::<S>(&PERM, ii + i2 + jp);

        let lp = gather_32::<S>(&PERM, ll + l3);
        let kp = gather_32::<S>(&PERM, kk + k3 + lp);
        let jp = gather_32::<S>(&PERM, jj + j3 + kp);
        let gi3 = gather_32::<S>(&PERM, ii + i3 + jp);

        let lp = gather_32::<S>(&PERM, ll + S::I32::set1(1));
        let kp = gather_32::<S>(&PERM, kk + S::I32::set1(1) + lp);
        let jp = gather_32::<S>(&PERM, jj + S::I32::set1(1) + kp);
        let gi4 = gather_32::<S>(&PERM, ii + S::I32::set1(1) + jp);
        (gi0, gi1, gi2, gi3, gi4)
    };

    let t0 = S::F32::set1(0.5) - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0);
    let t1 = S::F32::set1(0.5) - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1);
    let t2 = S::F32::set1(0.5) - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2);
    let t3 = S::F32::set1(0.5) - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3);
    let t4 = S::F32::set1(0.5) - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4);
    let mut t0q = t0 * t0;
    t0q = t0q * t0q;
    let mut t1q = t1 * t1;
    t1q = t1q * t1q;
    let mut t2q = t2 * t2;
    t2q = t2q * t2q;
    let mut t3q = t3 * t3;
    t3q = t3q * t3q;
    let mut t4q = t4 * t4;
    t4q = t4q * t4q;

    let mut n0 = t0q * grad4::<S>(seed, gi0, x0, y0, z0, w0);
    let mut n1 = t1q * grad4::<S>(seed, gi1, x1, y1, z1, w1);
    let mut n2 = t2q * grad4::<S>(seed, gi2, x2, y2, z2, w2);
    let mut n3 = t3q * grad4::<S>(seed, gi3, x3, y3, z3, w3);
    let mut n4 = t4q * grad4::<S>(seed, gi4, x4, y4, z4, w4);

    let mut cond = t0.cmp_lt(S::F32::zeroes());
    n0 = n0.and_not(cond);
    cond = t1.cmp_lt(S::F32::zeroes());
    n1 = n1.and_not(cond);
    cond = t2.cmp_lt(S::F32::zeroes());
    n2 = n2.and_not(cond);
    cond = t3.cmp_lt(S::F32::zeroes());
    n3 = n3.and_not(cond);
    cond = t4.cmp_lt(S::F32::zeroes());
    n4 = n4.and_not(cond);

    (n0 + n1 + n2 + n3 + n4) * S::F32::set1(62.77772078955791)
}
