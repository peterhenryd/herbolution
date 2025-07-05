use crate::noise::Noise;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::base::*;
use crate::simd::float::Float;
use crate::simd::{Simd, SimdBaseIo, SimdConsts};

pub trait Sample<S: Simd, const D: NoiseDim>: DimNoise<D> + Noise<D> {
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F;

    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F;

    fn sample_3d<F: SimdFloat>(&self, x: F, y: F, z: F) -> F;

    fn sample_4d<F: SimdFloat>(&self, x: F, y: F, z: F, w: F) -> F;
}

#[inline(always)]
unsafe fn get_1d_noise_helper_f64<F, S, SF, Settings, const D: NoiseDim>(settings: Settings) -> ([F; D.size()], F, F)
where
    F: Float,
    S: Simd,
    SF: SimdFloat<Backend = S, Scalar = F> + SimdBaseIo,
    Settings: Sample<S, D>,
{
    let dim = settings.dim();
    let freq_x = SF::set1(F::from(settings.freq()[0]));
    let start_x = F::from(dim.x);
    let mut min_s = SF::set1(F::MAX);
    let mut max_s = SF::set1(F::MIN);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr: Vec<F> = Vec::with_capacity(SF::WIDTH);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + F::from(i as f32));
    }
    x_arr.set_len(vector_width);
    let mut x = SF::load_from_ptr_unaligned(&x_arr[0]);
    for _ in 0..D.x_extent / vector_width {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += vector_width;
        x = x + S::Vf64::set1(vector_width as f64);
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
unsafe fn get_2d_noise_helper_f64<const D: NoiseDim, S: Simd, Settings: Sample<S, D>>(settings: Settings) -> ([f64; D.size()], f64, f64) {
    let dim = settings.dim();
    let freq_x = S::Vf64::set1(settings.freq()[0] as f64);
    let freq_y = S::Vf64::set1(settings.freq()[1] as f64);
    let start_x = dim.x as f64;
    let start_y = dim.y as f64;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = [0.0; D.size()];
    let result_ptr = result.as_mut_ptr();
    let mut y = S::Vf64::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = D.x_extent % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    for _ in 0..D.y_extent {
        let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
        for _ in 0..D.x_extent / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x = x + S::Vf64::set1(vector_width as f64);
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
        y = y + S::Vf64::set1(1.0);
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

/*
#[inline(always)]
unsafe fn get_3d_noise_helper_f64<const X: usize, const Y: usize, const Z: usize, S: Simd, Settings: Sample64<S, X, Y, Z, 1>>(noise: Settings) -> (Vec<f64>,
                                                                                                                                                  f64, f64) {
    let dim = noise.dim();
    let freq_x = S::Vf64::set1(noise.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(noise.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(noise.get_freq_z() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::<f64>::with_capacity(width * height * depth);
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = noise.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x = x + S::Vf64::set1(vector_width as f64);
            }
            if remainder != 0 {
                let f = noise.sample_3d(x * freq_x, y * freq_y, z * freq_z);
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
            y = y + S::Vf64::set1(1.0);
        }
        z = z + S::Vf64::set1(1.0);
    }
    result.set_len(width * height * depth);
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
unsafe fn get_4d_noise_helper_f64<const D: NoiseDim, S: Simd, Settings: Sample64<S, D>>(noise: Settings) -> (Vec<f64>, f64, f64) {
    let dim = noise.dim();
    let freq_x = S::Vf64::set1(noise.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(noise.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(noise.get_freq_z() as f64);
    let freq_w = S::Vf64::set1(noise.get_freq_w() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::<f64>::with_capacity(width * height * depth * time);
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf64::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf64::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf64::set1(start_y);
            for _ in 0..height {
                let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = noise.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(result_ptr.add(i));
                    i += vector_width;
                    x = x + S::Vf64::set1(vector_width as f64);
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
                y = y + S::Vf64::set1(1.0);
            }
            z = z + S::Vf64::set1(1.0);
        }
        w = w + S::Vf64::set1(1.0);
    }
    result.set_len(width * height * depth * time);
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
pub unsafe fn get_1d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f64::<D, S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_1d_noise_helper_f64::<D, S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f64::<D, S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f64::<D, S, GradientSettings>(*s),
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
pub unsafe fn get_2d_noise<const D: NoiseDim, S: Simd>(noise_type: &NoiseType<D>) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f64::<D, S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_2d_noise_helper_f64::<D, S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f64::<D, S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f64::<D, S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_2d_noise_helper_f64::<D, S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f64::<D, S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_3d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_3d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f64::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_4d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_4d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}


 */
