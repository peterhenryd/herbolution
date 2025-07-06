use crate::functions::simplex_32::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};
use crate::noise::f32::Sample32;
use crate::noise::Noise;
pub use crate::noise::NoiseTransform;
pub use crate::noise::NoiseType;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::Simd;
use crate::{get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise, get_4d_scaled_noise};

#[derive(Copy, Clone)]
pub struct GradientNoise<const D: NoiseDim> {
    dim: NoiseTransform<D>,
    freq: [f32; 4],
}

impl<const D: NoiseDim> DimNoise<D> for GradientNoise<D> {
    fn dim(&self) -> NoiseTransform<D> {
        self.dim
    }
}

impl<const D: NoiseDim> From<NoiseTransform<D>> for GradientNoise<D> {
    fn from(dim: NoiseTransform<D>) -> Self {
        GradientNoise { dim, freq: [0.02; 4] }
    }
}

impl<const D: NoiseDim> Noise<D> for GradientNoise<D> {
    fn set_seed(&mut self, seed: i64) {
        self.dim.seed = seed;
    }

    fn seed(&self) -> i64 {
        self.dim.seed
    }

    fn set_freq(&mut self, freq: [f32; D.dim()]) {
        self.freq[0..D.dim()].copy_from_slice(&freq);
    }

    fn freq(&self) -> &[f32] {
        &self.freq[0..D.dim()]
    }

    fn generate(self) -> ([f32; D.size()], f32, f32) {
        match D.dim() {
            1 => get_1d_noise(&NoiseType::Gradient(self)),
            2 => get_2d_noise(&NoiseType::Gradient(self)),
            3 => get_3d_noise(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {}

    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()] {
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match D.dim() {
            1 => get_1d_scaled_noise(&NoiseType::Gradient(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Gradient(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Gradient(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Gradient(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<const D: NoiseDim, S: Simd> Sample32<D, S> for GradientNoise<D> {
    #[inline(always)]
    fn sample_1d(&self, x: S::F32) -> S::F32 {
        simplex_1d::<S>(x, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::F32, y: S::F32) -> S::F32 {
        simplex_2d::<S>(x, y, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::F32, y: S::F32, z: S::F32) -> S::F32 {
        simplex_3d::<S>(x, y, z, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::F32, y: S::F32, z: S::F32, w: S::F32) -> S::F32 {
        simplex_4d::<S>(x, y, z, w, self.dim.seed)
    }
}

/*
impl<const D: NoiseDim, S: Simd> Sample<S, D> for GradientNoise<D> {
    #[inline(always)]
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F {
        simplex_1d_f64::<S>(x, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F {
        simplex_2d_f64::<S>(x, y, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_3d<F: SimdFloat>(&self, x: F, y: F, z: F) -> F {
        simplex_3d_f64::<S>(x, y, z, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_4d<F: SimdFloat>(&self, x: F, y: F, z: F, w: F) -> F {
        simplex_4d_f64::<S>(x, y, z, w, self.dim.seed.into())
    }
}
 */
