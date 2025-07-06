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
    type I8: SimdI8<Scalar = i8> + SimdBaseIo;

    type I16: SimdI16<Scalar = i16> + SimdBaseIo;

    type I32: SimdI32<Backend = Self, Scalar = i32> + SimdBaseIo;

    type I64: SimdI64<Backend = Self, Scalar = i64> + SimdBaseIo;

    type F32: SimdF32<Backend = Self, Scalar = f32> + SimdBaseIo;

    type F64: SimdF64<Backend = Self, Scalar = f64> + SimdBaseIo;
}
