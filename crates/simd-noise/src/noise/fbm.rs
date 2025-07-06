use super::{Noise, OctaveNoise};
use crate::functions::fbm_32::{fbm_1d, fbm_2d, fbm_3d, fbm_4d};
use crate::noise::f32::Sample32;
pub use crate::noise::NoiseTransform;
pub use crate::noise::NoiseType;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::{Simd, SimdBaseIo};
use crate::{get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise, get_4d_scaled_noise};

#[derive(Copy, Clone)]
pub struct FbmNoise<const D: NoiseDim> {
    pub dim: NoiseTransform<D>,
    freq: [f32; 4],
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: u8,
}

impl<const D: NoiseDim> DimNoise<D> for FbmNoise<D> {
    #[inline]
    fn dim(&self) -> NoiseTransform<D> {
        self.dim
    }
}

impl<const D: NoiseDim> From<NoiseTransform<D>> for FbmNoise<D> {
    #[inline]
    fn from(dim: NoiseTransform<D>) -> Self {
        FbmNoise {
            dim,
            freq: [0.02; 4],
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }
}

impl<const D: NoiseDim> Noise<D> for FbmNoise<D> {
    #[inline]
    fn set_seed(&mut self, seed: i64) {
        self.dim.seed = seed;
    }

    #[inline]
    fn seed(&self) -> i64 {
        self.dim.seed
    }

    #[inline]
    fn set_freq(&mut self, freq: [f32; D.dim()]) {
        self.freq[0..D.dim()].copy_from_slice(&freq);
    }

    #[inline]
    fn freq(&self) -> &[f32] {
        &self.freq[0..D.dim()]
    }

    #[inline]
    fn generate(self) -> ([f32; D.size()], f32, f32) {
        match D.dim() {
            1 => get_1d_noise(&NoiseType::Fbm(self)),
            2 => get_2d_noise(&NoiseType::Fbm(self)),
            3 => get_3d_noise(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    #[inline]
    fn validate(&self) {}

    #[inline]
    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()] {
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match D.dim() {
            1 => get_1d_scaled_noise(&NoiseType::Fbm(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Fbm(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Fbm(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Fbm(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<const D: NoiseDim> OctaveNoise for FbmNoise<D> {
    #[inline]
    fn set_lacunarity(&mut self, lacunarity: f32) {
        self.lacunarity = lacunarity;
    }

    #[inline]
    fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    #[inline]
    fn set_octaves(&mut self, octaves: u8) {
        self.octaves = octaves;
    }
}

impl<const D: NoiseDim, S: Simd> Sample32<D, S> for FbmNoise<D> {
    #[inline(always)]
    fn sample_1d(&self, x: S::F32) -> S::F32 {
        fbm_1d::<S>(x, S::F32::set1(self.lacunarity), S::F32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::F32, y: S::F32) -> S::F32 {
        fbm_2d::<S>(x, y, S::F32::set1(self.lacunarity), S::F32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::F32, y: S::F32, z: S::F32) -> S::F32 {
        fbm_3d::<S>(x, y, z, S::F32::set1(self.lacunarity), S::F32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::F32, y: S::F32, z: S::F32, w: S::F32) -> S::F32 {
        fbm_4d::<S>(x, y, z, w, S::F32::set1(self.lacunarity), S::F32::set1(self.gain), self.octaves, self.dim.seed)
    }
}

/*
impl<const D: NoiseDim, S: Simd> Sample<S, D> for FbmNoise<D> {
    #[inline(always)]
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F {
        fbm_1d_f64::<S>(
            x,
            F::set1(self.lacunarity.into()),
            F::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F {
        fbm_2d_f64::<S>(
            x,
            y,
            F::set1(self.lacunarity.into()),
            F::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_3d<F: SimdFloat>(&self, x: F, y: F, z: F) -> F {
        fbm_3d_f64::<S>(
            x,
            y,
            z,
            F::set1(self.lacunarity.into()),
            F::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_4d<F: SimdFloat>(&self, x: F, y: F, z: F, w: F) -> F {
        fbm_4d_f64::<S>(
            x,
            y,
            z,
            w,
            F::set1(self.lacunarity.into()),
            F::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
}
 */
