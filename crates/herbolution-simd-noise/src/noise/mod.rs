use std::marker::ConstParamTy;

pub use cellular::CellularNoise;
pub use cellular2::Cellular2Noise;
pub use fbm::FbmNoise;
pub use gradient::GradientNoise;
pub use ridge::RidgeNoise;
pub use turbulence::TurbulenceNoise;

mod cellular;
mod cellular2;
pub mod f32;
mod fbm;
mod gradient;
mod ridge;
mod turbulence;

pub trait Noise<const D: NoiseDim>: From<NoiseTransform<D>> + Into<NoiseType<D>> {
    fn set_seed(&mut self, seed: i64);

    #[inline]
    fn with_seed(mut self, seed: i64) -> Self {
        self.set_seed(seed);
        self
    }

    fn seed(&self) -> i64;

    fn set_freq(&mut self, freq: [f32; D.dim()]);

    #[inline]
    fn with_freq(mut self, freq: [f32; D.dim()]) -> Self {
        self.set_freq(freq);
        self
    }

    fn freq(&self) -> &[f32];

    fn generate(self) -> ([f32; D.size()], f32, f32);

    fn validate(&self);

    fn generate_scaled(self, min: f32, max: f32) -> [f32; D.size()];
}

pub trait OctaveNoise {
    fn set_lacunarity(&mut self, lacunarity: f32);

    #[inline]
    fn with_lacunarity(mut self, lacunarity: f32) -> Self
    where
        Self: Sized,
    {
        self.set_lacunarity(lacunarity);
        self
    }

    fn set_gain(&mut self, gain: f32);

    #[inline]
    fn with_gain(mut self, gain: f32) -> Self
    where
        Self: Sized,
    {
        self.set_gain(gain);
        self
    }

    fn set_octaves(&mut self, octaves: u8);

    #[inline]
    fn with_octaves(mut self, octaves: u8) -> Self
    where
        Self: Sized,
    {
        self.set_octaves(octaves);
        self
    }
}

#[derive(Copy, Clone)]
pub struct NoiseTransform<const D: NoiseDim> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub min: f32,
    pub max: f32,
    pub seed: i64,
}

impl<const D: NoiseDim> NoiseTransform<D> {
    #[inline]
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    #[inline]
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    #[inline]
    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    #[inline]
    pub fn with_w(mut self, w: f32) -> Self {
        self.w = w;
        self
    }

    #[inline]
    pub fn with_min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    #[inline]
    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

#[derive(Debug, ConstParamTy, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NoiseDim {
    pub(crate) x_extent: usize,
    pub(crate) y_extent: usize,
    pub(crate) z_extent: usize,
    pub(crate) w_extent: usize,
    dim: usize,
}

pub trait DimNoise<const D: NoiseDim> {
    fn dim(&self) -> NoiseTransform<D>;
}

impl NoiseDim {
    #[inline]
    pub const fn new_1d(x_extent: usize) -> Self {
        Self {
            x_extent,
            y_extent: 1,
            z_extent: 1,
            w_extent: 1,
            dim: 1,
        }
    }

    #[inline]
    pub const fn new_2d(x_extent: usize, y_extent: usize) -> Self {
        NoiseDim {
            x_extent,
            y_extent,
            z_extent: 1,
            w_extent: 1,
            dim: 2,
        }
    }

    #[inline]
    pub const fn new_3d(x_extent: usize, y_extent: usize, z_extent: usize) -> Self {
        NoiseDim {
            x_extent,
            y_extent,
            z_extent,
            w_extent: 1,
            dim: 3,
        }
    }

    #[inline]
    pub const fn new_4d(x_extent: usize, y_extent: usize, z_extent: usize, w_extent: usize) -> Self {
        NoiseDim {
            x_extent,
            y_extent,
            z_extent,
            w_extent,
            dim: 4,
        }
    }

    #[inline]
    pub const fn size(self) -> usize {
        self.x_extent * self.y_extent * self.z_extent * self.w_extent
    }

    #[inline]
    pub const fn dim(self) -> usize {
        self.dim
    }
}

impl<const D: NoiseDim> NoiseTransform<D> {
    #[inline]
    pub fn new() -> NoiseTransform<D> {
        NoiseTransform {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            min: 0.0,
            max: 1.0,
            seed: 0,
        }
    }

    #[inline]
    pub fn from_seed(seed: i64) -> NoiseTransform<D> {
        NoiseTransform {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            min: 0.0,
            max: 1.0,
            seed,
        }
    }
}

#[derive(Copy, Clone)]
pub enum NoiseType<const D: NoiseDim> {
    Fbm(FbmNoise<D>),
    Ridge(RidgeNoise<D>),
    Turbulence(TurbulenceNoise<D>),
    Gradient(GradientNoise<D>),
    Cellular(CellularNoise<D>),
    Cellular2(Cellular2Noise<D>),
}

impl<const D: NoiseDim> From<FbmNoise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: FbmNoise<D>) -> Self {
        NoiseType::Fbm(value)
    }
}

impl<const D: NoiseDim> From<RidgeNoise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: RidgeNoise<D>) -> Self {
        NoiseType::Ridge(value)
    }
}

impl<const D: NoiseDim> From<TurbulenceNoise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: TurbulenceNoise<D>) -> Self {
        NoiseType::Turbulence(value)
    }
}

impl<const D: NoiseDim> From<GradientNoise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: GradientNoise<D>) -> Self {
        NoiseType::Gradient(value)
    }
}

impl<const D: NoiseDim> From<CellularNoise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: CellularNoise<D>) -> Self {
        NoiseType::Cellular(value)
    }
}

impl<const D: NoiseDim> From<Cellular2Noise<D>> for NoiseType<D> {
    #[inline]
    fn from(value: Cellular2Noise<D>) -> Self {
        NoiseType::Cellular2(value)
    }
}

impl<const D: NoiseDim> DimNoise<D> for NoiseType<D> {
    #[inline]
    fn dim(&self) -> NoiseTransform<D> {
        match self {
            NoiseType::Fbm(s) => s.dim(),
            NoiseType::Ridge(s) => s.dim(),
            NoiseType::Turbulence(s) => s.dim(),
            NoiseType::Gradient(s) => s.dim(),
            NoiseType::Cellular(s) => s.dim(),
            NoiseType::Cellular2(s) => s.dim(),
        }
    }
}
