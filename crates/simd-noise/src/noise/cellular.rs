use super::Noise;
use crate::functions::cell_32::{cellular_2d, cellular_3d};
pub use crate::functions::cell_distance_function::CellDistanceFunction;
pub use crate::functions::cell_return_type::CellReturnType;
use crate::noise::f32::Sample32;
pub use crate::noise::NoiseTransform;
pub use crate::noise::NoiseType;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::{Simd, SimdBaseIo};
use crate::{get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise};

#[derive(Copy, Clone)]
pub struct CellularNoise<const D: NoiseDim> {
    dim: NoiseTransform<D>,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub distance_function: CellDistanceFunction,
    pub return_type: CellReturnType,
    pub jitter: f32,
}

impl<const D: NoiseDim> DimNoise<D> for CellularNoise<D> {
    fn dim(&self) -> NoiseTransform<D> {
        self.dim
    }
}

impl<const D: NoiseDim> Default for CellularNoise<D> {
    fn default() -> Self {
        CellularNoise {
            dim: NoiseTransform::new(),
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }
}

impl<const D: NoiseDim> From<NoiseTransform<D>> for CellularNoise<D> {
    fn from(dim: NoiseTransform<D>) -> Self {
        CellularNoise {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }
}

impl<const D: NoiseDim> Noise<D> for CellularNoise<D> {
    fn set_seed(&mut self, seed: i64) {
        self.dim.seed = seed;
    }

    fn seed(&self) -> i64 {
        self.dim.seed
    }

    fn set_freq(&mut self, freq: [f32; D.dim()]) {
        match D.dim() {
            1 => {
                self.freq_x = freq[0];
                self.freq_y = 0.0;
                self.freq_z = 0.0;
            }
            2 => {
                self.freq_x = freq[0];
                self.freq_y = freq[1];
                self.freq_z = 0.0;
            }
            3 => {
                self.freq_x = freq[0];
                self.freq_y = freq[1];
                self.freq_z = freq[2];
            }
            _ => panic!("Frequency setting not implemented for dimension {}", D.dim()),
        }
    }

    fn freq(&self) -> &[f32] {
        todo!()
    }

    fn generate(self) -> ([f32; D.size()], f32, f32) {
        match D.dim() {
            2 => get_2d_noise(&NoiseType::Cellular(self)),
            3 => get_3d_noise(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {}

    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()] {
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match D.dim() {
            2 => get_2d_scaled_noise(&NoiseType::Cellular(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Cellular(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<const D: NoiseDim, S: Simd> Sample32<D, S> for CellularNoise<D> {
    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_1d(&self, x: S::F32) -> S::F32 {
        unimplemented!()
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::F32, y: S::F32) -> S::F32 {
        cellular_2d::<S>(x, y, self.distance_function, self.return_type, S::F32::set1(self.jitter), self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::F32, y: S::F32, z: S::F32) -> S::F32 {
        cellular_3d::<S>(x, y, z, self.distance_function, self.return_type, S::F32::set1(self.jitter), self.dim.seed)
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_4d(&self, x: S::F32, y: S::F32, z: S::F32, w: S::F32) -> S::F32 {
        unimplemented!()
    }
}

/*
impl<const D: NoiseDim, S: Simd> Sample<S, D> for CellularNoise<D> {
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F {
        todo!()
    }

    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F {
        cellular_2d_f64::<S>(
            x,
            y,
            self.distance_function,
            self.return_type,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into(),
        )
    }

    fn sample_3d<F: SimdFloat>(&self, x: F, y: F, z: F) -> F {
        cellular_3d_f64::<S>(
            x,
            y,
            z,
            self.distance_function,
            self.return_type,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into(),
        )
    }

    fn sample_4d<F: SimdFloat>(&self, x: F, y: F, z: F, w: F) -> F {
        todo!()
    }
}
 */

impl<const D: NoiseDim> CellularNoise<D> {
    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut Self {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: CellReturnType) -> &mut Self {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut Self {
        self.jitter = jitter;
        self
    }
}
