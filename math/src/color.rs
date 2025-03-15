use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[const_trait]
pub trait Color {
    type Comp: ColorComp;

    fn from_rgb(r: Self::Comp, g: Self::Comp, b: Self::Comp) -> Self;

    fn into_rgba(self) -> Rgba<Self::Comp>;
}

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: ColorComp> const Color for Rgba<T> {
    type Comp = T;

    fn from_rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b, a: T::MAX }
    }

    fn into_rgba(self) -> Rgba<T> {
        self
    }
}

unsafe impl<T: Zeroable> Zeroable for Rgba<T> {}

unsafe impl<T: Pod> Pod for Rgba<T> {}

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: ColorComp> const Color for Rgb<T> {
    type Comp = T;

    fn from_rgb(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    fn into_rgba(self) -> Rgba<T> {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: T::MAX,
        }
    }
}

pub trait ColorComp {
    const MIN: Self;
    const MAX: Self;
}

impl ColorComp for u8 {
    const MIN: Self = 0;
    const MAX: Self = 255;
}

impl ColorComp for f32 {
    const MIN: Self = 0.0;
    const MAX: Self = 1.0;
}

impl ColorComp for f64 {
    const MIN: Self = 0.0;
    const MAX: Self = 1.0;
}

#[const_trait]
pub trait ColorConsts {
    const WHITE: Self;
    const BLACK: Self;
}

impl<C: const Color> const ColorConsts for C {
    const WHITE: Self = C::from_rgb(C::Comp::MAX, C::Comp::MAX, C::Comp::MAX);
    const BLACK: Self = C::from_rgb(C::Comp::MIN, C::Comp::MIN, C::Comp::MIN);
}

impl<C: ColorComp> Rgba<C> {
    pub const TRANSPARENT: Self = Self::new(C::MIN, C::MIN, C::MIN, C::MIN);
}

unsafe impl<T: Zeroable> Zeroable for Rgb<T> {}

unsafe impl<T: Pod> Pod for Rgb<T> {}

impl Into<Rgba<f32>> for Rgba<u8> {
    fn into(self) -> Rgba<f32> {
        Rgba {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
            a: self.a as f32 / 255.0,
        }
    }
}

impl Into<Rgba<f64>> for Rgba<u8> {
    fn into(self) -> Rgba<f64> {
        Rgba {
            r: self.r as f64 / 255.0,
            g: self.g as f64 / 255.0,
            b: self.b as f64 / 255.0,
            a: self.a as f64 / 255.0,
        }
    }
}
