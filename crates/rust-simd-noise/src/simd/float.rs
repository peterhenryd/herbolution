#![allow(dead_code)]

use std::ops::Add;

pub trait Float: Copy + private::Sealed + Sized + From<f32> + Add<Output = Self> {
    const ZERO: Self;
    const MIN: Self;
    const MAX: Self;

    fn m_floor(self) -> Self;

    fn m_ceil(self) -> Self;

    fn m_round(self) -> Self;

    fn m_trunc(self) -> Self;

    fn m_fract(self) -> Self;

    fn m_abs(self) -> Self;

    fn m_mul_add(self, a: Self, b: Self) -> Self;

    fn m_powf(self, n: Self) -> Self;

    fn m_sqrt(self) -> Self;

    fn m_exp(self) -> Self;

    fn m_exp2(self) -> Self;

    fn m_ln(self) -> Self;

    fn m_log(self, base: Self) -> Self;

    fn m_log2(self) -> Self;

    fn m_log10(self) -> Self;

    fn m_cbrt(self) -> Self;

    fn m_hypot(self, other: Self) -> Self;

    fn m_sin(self) -> Self;

    fn m_cos(self) -> Self;

    fn m_tan(self) -> Self;

    fn m_asin(self) -> Self;

    fn m_acos(self) -> Self;

    fn m_atan(self) -> Self;

    fn m_atan2(self, other: Self) -> Self;

    #[inline]
    fn m_sin_cos(self) -> (Self, Self)
    where
        Self: Copy,
    {
        (self.m_sin(), self.m_cos())
    }

    fn m_exp_m1(self) -> Self;

    fn m_ln_1p(self) -> Self;

    fn m_sinh(self) -> Self;

    fn m_cosh(self) -> Self;

    fn m_tanh(self) -> Self;

    fn m_asinh(self) -> Self;

    fn m_acosh(self) -> Self;

    fn m_atanh(self) -> Self;

    fn cast_i64(self) -> i64;
}

mod private {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;

    #[inline]
    fn m_floor(self) -> Self {
        f32::floor(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        f32::ceil(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        f32::round(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        f32::trunc(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        f32::abs(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        f32::powf(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        f32::exp(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        f32::exp2(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        f32::ln(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        f32::log(self, base)
    }

    #[inline]
    fn m_log2(self) -> Self {
        f32::log2(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        f32::log10(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        f32::cbrt(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        f32::hypot(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        f32::sin(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        f32::cos(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        f32::tan(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        f32::asin(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        f32::acos(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        f32::atan(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        f32::exp_m1(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        f32::ln_1p(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        f32::sinh(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        f32::cosh(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        f32::tanh(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        f32::asinh(self)
    }

    #[inline]
    fn m_acosh(self) -> Self {
        f32::acosh(self)
    }

    #[inline]
    fn m_atanh(self) -> Self {
        f32::atanh(self)
    }

    fn cast_i64(self) -> i64 {
        self as i64
    }
}

impl Float for f64 {
    const ZERO: Self = 0.0;
    const MIN: Self = f64::MIN;
    const MAX: Self = f64::MAX;

    #[inline]
    fn m_floor(self) -> Self {
        f64::floor(self)
    }

    #[inline]
    fn m_ceil(self) -> Self {
        f64::ceil(self)
    }

    #[inline]
    fn m_round(self) -> Self {
        f64::round(self)
    }

    #[inline]
    fn m_trunc(self) -> Self {
        f64::trunc(self)
    }

    #[inline]
    fn m_fract(self) -> Self {
        self - self.trunc()
    }

    #[inline]
    fn m_abs(self) -> Self {
        f64::abs(self)
    }

    #[inline]
    fn m_mul_add(self, a: Self, b: Self) -> Self {
        f64::mul_add(self, a, b)
    }

    #[inline]
    fn m_powf(self, n: Self) -> Self {
        f64::powf(self, n)
    }

    #[inline]
    fn m_sqrt(self) -> Self {
        f64::sqrt(self)
    }

    #[inline]
    fn m_exp(self) -> Self {
        f64::exp(self)
    }

    #[inline]
    fn m_exp2(self) -> Self {
        f64::exp2(self)
    }

    #[inline]
    fn m_ln(self) -> Self {
        f64::ln(self)
    }

    #[inline]
    fn m_log(self, base: Self) -> Self {
        f64::log(self, base)
    }

    #[inline]
    fn m_log2(self) -> Self {
        f64::log2(self)
    }

    #[inline]
    fn m_log10(self) -> Self {
        f64::log10(self)
    }

    #[inline]
    fn m_cbrt(self) -> Self {
        f64::cbrt(self)
    }

    #[inline]
    fn m_hypot(self, other: Self) -> Self {
        f64::hypot(self, other)
    }

    #[inline]
    fn m_sin(self) -> Self {
        f64::sin(self)
    }

    #[inline]
    fn m_cos(self) -> Self {
        f64::cos(self)
    }

    #[inline]
    fn m_tan(self) -> Self {
        f64::tan(self)
    }

    #[inline]
    fn m_asin(self) -> Self {
        f64::asin(self)
    }

    #[inline]
    fn m_acos(self) -> Self {
        f64::acos(self)
    }

    #[inline]
    fn m_atan(self) -> Self {
        f64::atan(self)
    }

    #[inline]
    fn m_atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }

    #[inline]
    fn m_exp_m1(self) -> Self {
        f64::exp_m1(self)
    }

    #[inline]
    fn m_ln_1p(self) -> Self {
        f64::ln_1p(self)
    }

    #[inline]
    fn m_sinh(self) -> Self {
        f64::sinh(self)
    }

    #[inline]
    fn m_cosh(self) -> Self {
        f64::cosh(self)
    }

    #[inline]
    fn m_tanh(self) -> Self {
        f64::tanh(self)
    }

    #[inline]
    fn m_asinh(self) -> Self {
        f64::asinh(self)
    }

    #[inline]
    fn m_acosh(self) -> Self {
        f64::acosh(self)
    }

    #[inline]
    fn m_atanh(self) -> Self {
        f64::atanh(self)
    }

    fn cast_i64(self) -> i64 {
        self as i64
    }
}
