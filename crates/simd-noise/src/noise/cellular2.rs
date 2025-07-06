use super::Noise;
use crate::functions::cell2_32::{cellular2_2d, cellular2_3d};
pub use crate::functions::cell2_return_type::Cell2ReturnType;
pub use crate::functions::cell_distance_function::CellDistanceFunction;
use crate::noise::f32::Sample32;
pub use crate::noise::NoiseTransform;
pub use crate::noise::NoiseType;
use crate::noise::{DimNoise, NoiseDim};
use crate::simd::{Simd, SimdBaseIo};
use crate::{get_2d_noise, get_2d_scaled_noise, get_3d_noise, get_3d_scaled_noise};

#[derive(Copy, Clone)]
pub struct Cellular2Noise<const D: NoiseDim> {
    dim: NoiseTransform<D>,
    freq: [f32; 3],
    pub distance_function: CellDistanceFunction,
    pub return_type: Cell2ReturnType,
    pub jitter: f32,
    pub index0: usize,
    pub index1: usize,
}

impl<const D: NoiseDim> From<NoiseTransform<D>> for Cellular2Noise<D> {
    fn from(dim: NoiseTransform<D>) -> Self {
        Cellular2Noise {
            dim,
            freq: [0.02; 3],
            distance_function: CellDistanceFunction::Euclidean,
            return_type: Cell2ReturnType::Distance2,
            jitter: 0.25,
            index0: 0,
            index1: 1,
        }
    }
}

impl<const D: NoiseDim> DimNoise<D> for Cellular2Noise<D> {
    fn dim(&self) -> NoiseTransform<D> {
        return self.dim;
    }
}

impl<const D: NoiseDim> Noise<D> for Cellular2Noise<D> {
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
        match D.dim {
            2 => get_2d_noise(&NoiseType::Cellular2(self)),
            3 => get_3d_noise(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        if self.index0 > 2 || self.index1 > 3 || self.index0 >= self.index1 {
            panic!("invalid index settings in cellular2 noise");
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()] {
        self.validate();
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match D.dim() {
            2 => get_2d_scaled_noise(&NoiseType::Cellular2(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Cellular2(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl<const D: NoiseDim, S: Simd> Sample32<D, S> for Cellular2Noise<D> {
    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_1d(&self, x: S::F32) -> S::F32 {
        unimplemented!()
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::F32, y: S::F32) -> S::F32 {
        cellular2_2d::<S>(
            x,
            y,
            self.distance_function,
            self.return_type,
            S::F32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::F32, y: S::F32, z: S::F32) -> S::F32 {
        cellular2_3d::<S>(
            x,
            y,
            z,
            self.distance_function,
            self.return_type,
            S::F32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed,
        )
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_4d(&self, x: S::F32, y: S::F32, z: S::F32, w: S::F32) -> S::F32 {
        unimplemented!()
    }
}

/*
impl<const D: NoiseDim, S: Simd> Sample<S, D> for Cellular2Noise<D> {
    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_1d<F: SimdFloat>(&self, x: F) -> F {
        unimplemented!()
    }

    #[inline(always)]
    fn sample_2d<F: SimdFloat>(&self, x: F, y: F) -> F {
        cellular2_2d_f64::<S, F>(
            x,
            y,
            self.distance_function,
            self.return_type,
            F::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_3d<F: SimdFloat>(&self, x: F, y: F, z: F) -> F {
        cellular2_3d_f64::<S>(
            x,
            y,
            z,
            self.distance_function,
            self.return_type,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    #[allow(unused_variables)]
    fn sample_4d<F: SimdFloat>(&self, x: F, y: F, z: F, w: F) -> F {
        unimplemented!()
    }
}
 */

impl<const D: NoiseDim> Cellular2Noise<D> {
    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut Cellular2Noise<D> {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: Cell2ReturnType) -> &mut Cellular2Noise<D> {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut Cellular2Noise<D> {
        self.jitter = jitter;
        self
    }

    pub fn with_index0(&mut self, i: usize) -> &mut Cellular2Noise<D> {
        self.index0 = i;
        self
    }

    pub fn with_index1(&mut self, i: usize) -> &mut Cellular2Noise<D> {
        self.index1 = i;
        self
    }
}
