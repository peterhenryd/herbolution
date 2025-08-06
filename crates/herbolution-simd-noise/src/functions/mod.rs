use crate::noise::{DimNoise, NoiseDim, NoiseType};
use crate::simd::{Simd, SimdBaseIo, SimdConsts};

pub mod cell2_32;
pub mod cell2_return_type;
pub mod cell_32;
pub mod cell_distance_function;
pub mod cell_return_type;
pub mod cellular_32;
pub mod cellular_64;
pub mod fbm_32;
pub mod fbm_64;
pub mod gradient_32;
pub mod gradient_64;
pub mod hash3d_32;
pub mod hash3d_64;
pub mod ops;
pub mod ridge_32;
pub mod ridge_64;
pub mod simplex_32;
pub mod simplex_64;
pub mod turbulence_32;
pub mod turbulence_64;

#[inline(always)]
pub unsafe fn scale_noise<S: Simd>(scale_min: f32, scale_max: f32, min: f32, max: f32, data: &mut [f32]) {
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    let vector_width = S::F32::WIDTH;
    let mut i = 0;
    if data.len() >= vector_width {
        while i <= data.len() - vector_width {
            let value = (S::F32::set1(multiplier) * S::F32::load_from_ptr_unaligned(&data[i])) + S::F32::set1(offset);
            value.copy_to_ptr_unaligned(data.get_unchecked_mut(i));
            i += vector_width;
        }
    }
    i = data.len() - (data.len() % vector_width);
    while i < data.len() {
        *data.get_unchecked_mut(i) = data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}

pub(crate) unsafe fn get_scaled_noise<const D: NoiseDim, S: Simd, F: Fn(&NoiseType<D>) -> ([f32; D.size()], f32, f32)>(
    noise_type: &NoiseType<D>,
    noise_fn: F,
) -> [f32; D.size()] {
    let (mut functions, min, max) = noise_fn(noise_type);
    let dim = noise_type.dim();
    scale_noise::<S>(dim.min, dim.max, min, max, &mut functions);
    functions
}
