#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub mod noise;
pub mod intrinsics;
pub mod functions;
pub mod simd;

pub use functions::cell2_return_type::Cell2ReturnType;
pub use functions::cell_distance_function::CellDistanceFunction;
pub use functions::cell_return_type::CellReturnType;
pub use noise::NoiseTransform;
use paste::item as simd_paste_item;
use crate::noise::{NoiseDim};
use crate::simd::{__SimdRunner, __run_simd_generic, __run_simd_invoke_scalar, __run_simd_runtime_decide};

simd_runtime_generate!(
    pub fn get_1d_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> ([f32; D.size()], f32, f32) {
        noise::f32::get_1d_noise::<D, S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_2d_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> ([f32; D.size()], f32, f32) {
        noise::f32::get_2d_noise::<D, S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_3d_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> ([f32; D.size()], f32, f32) {
        noise::f32::get_3d_noise::<D, S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_4d_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> ([f32; D.size()], f32, f32) {
        noise::f32::get_4d_noise::<D, S>(noise_type)
    }
);


simd_runtime_generate!(
    pub fn get_1d_scaled_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> [f32; D.size()] {
        unsafe { functions::get_scaled_noise::<D, S, _>(noise_type, get_1d_noise) }
    }
);

simd_runtime_generate!(
    pub fn get_2d_scaled_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> [f32; D.size()] {
        unsafe { functions::get_scaled_noise::<D, S, _>(noise_type, get_2d_noise) }
    }
);

simd_runtime_generate!(
    pub fn get_3d_scaled_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> [f32; D.size()] {
        unsafe { functions::get_scaled_noise::<D, S, _>(noise_type, get_3d_noise) }
    }
);

simd_runtime_generate!(
    pub fn get_4d_scaled_noise<const D: NoiseDim>(noise_type: &noise::NoiseType<D>) -> [f32; D.size()] {
        unsafe { functions::get_scaled_noise::<D, S, _>(noise_type, get_4d_noise) }
    }
);
