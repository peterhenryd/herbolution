use super::{Noise, OctaveNoise};
use crate::functions::turbulence_32::{turbulence_1d, turbulence_2d, turbulence_3d, turbulence_4d};
use crate::noise::f32::Sample32;
pub use crate::noise::NoiseTransform;
pub use crate::noise::NoiseType;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::{Simd, SimdBaseIo};
use crate::{get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise, get_4d_scaled_noise};

#[derive(Copy, Clone)]
pub struct TurbulenceNoise<const D: NoiseDim> {
    dim: NoiseTransform<D>,
    freq: [f32; 4],
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: u8,
}

impl<const D: NoiseDim> DimNoise<D> for TurbulenceNoise<D> {
    fn dim(&self) -> NoiseTransform<D> {
        self.dim
    }
}

impl<const D: NoiseDim> From<NoiseTransform<D>> for TurbulenceNoise<D> {
    fn from(dim: NoiseTransform<D>) -> Self {
        TurbulenceNoise {
            dim,
            freq: [0.02; 4],
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3,
        }
    }
}

impl<const D: NoiseDim> Noise<D> for TurbulenceNoise<D> {
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

    fn validate(&self) {}

    fn generate(self) -> ([f32; D.size()], f32, f32) {
        match D.dim() {
            1 => get_1d_noise(&NoiseType::Turbulence(self)),
            2 => get_2d_noise(&NoiseType::Turbulence(self)),
            3 => get_3d_noise(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()] {
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match D.dim() {
            1 => get_1d_scaled_noise(&NoiseType::Turbulence(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Turbulence(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Turbulence(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Turbulence(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<const D: NoiseDim> OctaveNoise for TurbulenceNoise<D> {
    fn set_lacunarity(&mut self, lacunarity: f32) {
        self.lacunarity = lacunarity;
    }

    fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    fn set_octaves(&mut self, octaves: u8) {
        self.octaves = octaves;
    }
}

impl<const D: NoiseDim, S: Simd> Sample32<D, S> for TurbulenceNoise<D> {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        turbulence_1d::<S>(x, S::Vf32::set1(self.lacunarity), S::Vf32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        turbulence_2d::<S>(x, y, S::Vf32::set1(self.lacunarity), S::Vf32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        turbulence_3d::<S>(x, y, z, S::Vf32::set1(self.lacunarity), S::Vf32::set1(self.gain), self.octaves, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        turbulence_4d::<S>(
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }
}

/*
impl<const D: NoiseDim, S: Simd> Sample<S, D> for TurbulenceNoise<D> {
    #[inline(always)]
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F {
        turbulence_1d_f64::<S>(
            x,
            F::set1(self.lacunarity.into()),
            F::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F {
        turbulence_2d_f64::<S>(
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
        turbulence_3d_f64::<S>(
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
        turbulence_4d_f64::<S>(
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
