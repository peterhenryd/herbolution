pub use backends::*;
pub use base::*;
pub use float::*;
pub use invoking::*;
pub use ops::*;

mod backends;
mod base;
mod float;
mod invoking;
mod ops;
mod overloads;

pub trait Simd: 'static + Sync + Send {
    type Vi8: SimdInt8<Scalar = i8> + SimdBaseIo;

    type Vi16: SimdInt16<Scalar = i16> + SimdBaseIo;

    type Vi32: SimdInt32<Backend = Self, Scalar = i32> + SimdBaseIo;

    type Vi64: SimdInt64<Backend = Self, Scalar = i64> + SimdBaseIo;

    type Vf32: SimdFloat32<Backend = Self, Scalar = f32> + SimdBaseIo;

    type Vf64: SimdFloat64<Backend = Self, Scalar = f64> + SimdBaseIo;
}
