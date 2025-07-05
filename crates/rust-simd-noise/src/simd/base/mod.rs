use core::fmt::Debug;
use core::ops::*;

mod io;

pub(crate) mod iters;
pub use iters::*;

mod transmute;
pub use transmute::*;

pub(crate) mod specializations;
pub use io::SimdBaseIo;
pub use specializations::*;

use crate::simd::Simd;

pub trait SimdConsts: 'static + Copy + Sync + Send + Debug {
    type Scalar: Copy + Debug + Sync + Send;
    type HorizontalAddScalar: Copy + Debug + Sync + Send;
    const WIDTH: usize;

    type ArrayRepresentation: Index<usize, Output = Self::Scalar> + IndexMut<usize> + Clone;

    type UnderlyingType: Copy + Debug + Sync + Send;

    type Backend: Simd;
}

pub trait SimdBaseOps:
    SimdConsts
    + SimdBaseIo
    + IndexMut<usize>
    + Index<usize, Output = <Self as SimdConsts>::Scalar>
    + Add<Self, Output = Self>
    + Add<<Self as SimdConsts>::Scalar, Output = Self>
    + AddAssign<Self>
    + AddAssign<<Self as SimdConsts>::Scalar>
    + Neg<Output = Self>
    + Sub<Self, Output = Self>
    + Sub<<Self as SimdConsts>::Scalar, Output = Self>
    + SubAssign<Self>
    + SubAssign<<Self as SimdConsts>::Scalar>
    + Mul<Self, Output = Self>
    + Mul<<Self as SimdConsts>::Scalar, Output = Self>
    + MulAssign<Self>
    + MulAssign<<Self as SimdConsts>::Scalar>
    + BitAnd<Self, Output = Self>
    + BitAnd<<Self as SimdConsts>::Scalar, Output = Self>
    + BitAndAssign<Self>
    + BitAndAssign<<Self as SimdConsts>::Scalar>
    + BitOr<Self, Output = Self>
    + BitOr<<Self as SimdConsts>::Scalar, Output = Self>
    + BitOrAssign<Self>
    + BitOrAssign<<Self as SimdConsts>::Scalar>
    + BitXor<Self, Output = Self>
    + BitXor<<Self as SimdConsts>::Scalar, Output = Self>
    + BitXorAssign<Self>
    + BitXorAssign<<Self as SimdConsts>::Scalar>
    + Not<Output = Self>
{
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;

    fn neg(self) -> Self {
        Self::zeroes() - self
    }

    fn bit_and(self, rhs: Self) -> Self;
    fn bit_or(self, rhs: Self) -> Self;
    fn bit_xor(self, rhs: Self) -> Self;

    fn bit_not(self) -> Self;

    fn abs(self) -> Self;

    fn and_not(self, rhs: Self) -> Self;

    fn blendv(self, a: Self, b: Self) -> Self;

    fn cmp_eq(self, rhs: Self) -> Self;

    fn cmp_neq(self, rhs: Self) -> Self;

    fn cmp_lt(self, rhs: Self) -> Self;

    fn cmp_lte(self, rhs: Self) -> Self;

    fn cmp_gt(self, rhs: Self) -> Self;

    fn cmp_gte(self, rhs: Self) -> Self;

    fn max(self, rhs: Self) -> Self;

    fn min(self, rhs: Self) -> Self;

    fn horizontal_add(self) -> Self::HorizontalAddScalar;
}

pub trait SimdBase: SimdBaseOps + SimdBaseIo + SimdIter {}
impl<T: SimdBaseOps + SimdBaseIo + SimdIter> SimdBase for T {}
