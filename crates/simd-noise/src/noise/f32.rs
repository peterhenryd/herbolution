use crate::noise::NoiseType;
use crate::noise::{Cellular2Noise, CellularNoise, FbmNoise, GradientNoise, Noise, RidgeNoise, TurbulenceNoise};
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts};

pub trait Sample32<const D: NoiseDim, S: Simd>: DimNoise<D> + Noise<D> {
    fn sample_1d(&self, x: S::F32) -> S::F32;
    fn sample_2d(&self, x: S::F32, y: S::F32) -> S::F32;
    fn sample_3d(&self, x: S::F32, y: S::F32, z: S::F32) -> S::F32;
    fn sample_4d(&self, x: S::F32, y: S::F32, z: S::F32, w: S::F32) -> S::F32;
}

#[inline(always)]
unsafe fn get_1d_noise_helper_f32<const D: NoiseDim, S: Simd, Settings: Sample32<D, S>>(settings: Settings) -> ([f32; D.size()], f32, f32) {
    let dim = settings.dim();
    let freq_x = S::F32::set1(settings.freq()[0]);
    let start_x = dim.x;
    let mut min_s = S::F32::set1(f32::MAX);
    let mut max_s = S::F32::set1(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::F32::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut x = S::F32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..D.x_extent / vector_width {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += vector_width;
        x = x + S::F32::set1(vector_width as f32);
    }
    if remainder != 0 {
        let f = settings.sample_1d(x * freq_x);
        for j in 0..remainder {
            let n = f[j];
            result_ptr.add(i).write(n);
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            i += 1;
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_helper_f32<const D: NoiseDim, S: Simd, Settings: Sample32<D, S>>(settings: Settings) -> ([f32; D.size()], f32, f32) {
    let dim = settings.dim();
    let freq_x = S::F32::set1(settings.freq()[0]);
    let freq_y = S::F32::set1(settings.freq()[1]);
    let start_x = dim.x;
    let start_y = dim.y;

    let mut min_s = S::F32::set1(f32::MAX);
    let mut max_s = S::F32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut y = S::F32::set1(start_y);
    let mut i = 0;
    let vector_width = S::F32::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    for _ in 0..D.y_extent {
        let mut x = S::F32::load_from_ptr_unaligned(x_ptr);
        for _ in 0..D.x_extent / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x = x + S::F32::set1(vector_width as f32);
        }
        if remainder != 0 {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            for j in 0..remainder {
                let n = f[j];
                result_ptr.add(i).write(n);
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
        y = y + S::F32::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_3d_noise_helper_f32<const D: NoiseDim, S: Simd, Settings: Sample32<D, S>>(settings: Settings) -> ([f32; D.size()], f32, f32) {
    let dim = settings.dim();
    let freq_x = S::F32::set1(settings.freq()[0]);
    let freq_y = S::F32::set1(settings.freq()[1]);
    let freq_z = S::F32::set1(settings.freq()[2]);
    let start_x = dim.x;
    let start_y = dim.y;
    let start_z = dim.z;

    let mut min_s = S::F32::set1(f32::MAX);
    let mut max_s = S::F32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::F32::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut z = S::F32::set1(start_z);
    for _ in 0..D.z_extent {
        let mut y = S::F32::set1(start_y);
        for _ in 0..D.y_extent {
            let mut x = S::F32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..D.x_extent / vector_width {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x = x + S::F32::set1(vector_width as f32);
            }
            if remainder != 0 {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                for j in 0..remainder {
                    let n = f[j];
                    result_ptr.add(i).write(n);
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
            y = y + S::F32::set1(1.0);
        }
        z = z + S::F32::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_4d_noise_helper_f32<const D: NoiseDim, S: Simd, Settings: Sample32<D, S>>(noise: Settings) -> ([f32; D.size()], f32, f32) {
    let dim = noise.dim();
    let freq_x = S::F32::set1(noise.freq()[0]);
    let freq_y = S::F32::set1(noise.freq()[1]);
    let freq_z = S::F32::set1(noise.freq()[2]);
    let freq_w = S::F32::set1(noise.freq()[3]);
    let start_x = dim.x;
    let start_y = dim.y;
    let start_z = dim.z;
    let start_w = dim.w;

    let mut min_s = S::F32::set1(f32::MAX);
    let mut max_s = S::F32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::F32::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut w = S::F32::set1(start_w);
    for _ in 0..D.w_extent {
        let mut z = S::F32::set1(start_z);
        for _ in 0..D.z_extent {
            let mut y = S::F32::set1(start_y);
            for _ in 0..D.y_extent {
                let mut x = S::F32::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..D.w_extent / vector_width {
                    let f = noise.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(result_ptr.add(i));
                    i += vector_width;
                    x = x + S::F32::set1(vector_width as f32);
                }
                if remainder != 0 {
                    let f = noise.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    for j in 0..remainder {
                        let n = f[j];
                        result_ptr.add(i).write(n);
                        if n < min {
                            min = n;
                        }
                        if n > max {
                            max = n;
                        }
                        i += 1;
                    }
                }
                y = y + S::F32::set1(1.0);
            }
            z = z + S::F32::set1(1.0);
        }
        w = w + S::F32::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> ([f32; D.size()], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f32::<D, S, FbmNoise<D>>(*s),
        NoiseType::Ridge(s) => get_1d_noise_helper_f32::<D, S, RidgeNoise<D>>(*s),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f32::<D, S, TurbulenceNoise<D>>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f32::<D, S, GradientNoise<D>>(*s),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> ([f32; D.size()], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f32::<D, S, FbmNoise<D>>(*s),
        NoiseType::Ridge(s) => get_2d_noise_helper_f32::<D, S, RidgeNoise<D>>(*s),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f32::<D, S, TurbulenceNoise<D>>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f32::<D, S, GradientNoise<D>>(*s),
        NoiseType::Cellular(s) => get_2d_noise_helper_f32::<D, S, CellularNoise<D>>(*s),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f32::<D, S, Cellular2Noise<D>>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> ([f32; D.size()], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f32::<D, S, FbmNoise<D>>(*s),
        NoiseType::Ridge(s) => get_3d_noise_helper_f32::<D, S, RidgeNoise<D>>(*s),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f32::<D, S, TurbulenceNoise<D>>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f32::<D, S, GradientNoise<D>>(*s),
        NoiseType::Cellular(s) => get_3d_noise_helper_f32::<D, S, CellularNoise<D>>(*s),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f32::<D, S, Cellular2Noise<D>>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> ([f32; D.size()], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f32::<D, S, FbmNoise<D>>(*s),
        NoiseType::Ridge(s) => get_4d_noise_helper_f32::<D, S, RidgeNoise<D>>(*s),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f32::<D, S, TurbulenceNoise<D>>(*s),
        NoiseType::Gradient(s) => get_4d_noise_helper_f32::<D, S, GradientNoise<D>>(*s),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}
