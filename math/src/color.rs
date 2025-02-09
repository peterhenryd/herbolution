use bytemuck::{Pod, Zeroable};

#[derive(derive::Vector)]
pub struct Color3<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color4<T> {
    pub fn extend(self, alpha: T) -> Color4<T> {
        Color4::new(self.r, self.g, self.b, alpha)
    }

    pub fn opaque(self) -> Color4<T>
    where
        T: ColorComp,
    {
        self.extend(T::MAX)
    }
}

impl<T: ColorComp> Color3<T> {
    pub const WHITE: Self = Self::new(T::MAX, T::MAX, T::MAX);
    pub const GRAY: Self = Self::new(T::HALF, T::HALF, T::HALF);
    pub const BLACK: Self = Self::new(T::MIN, T::MIN, T::MIN);

    pub const RED: Self = Self::new(T::MAX, T::MIN, T::MIN);
    pub const GREEN: Self = Self::new(T::MIN, T::MAX, T::MIN);
    pub const BLUE: Self = Self::new(T::MIN, T::MIN, T::MAX);

    pub const YELLOW: Self = Self::new(T::MAX, T::MAX, T::MIN);
    pub const CYAN: Self = Self::new(T::MIN, T::MAX, T::MAX);
    pub const MAGENTA: Self = Self::new(T::MAX, T::MIN, T::MAX);

    pub const ORANGE: Self = Self::new(T::MAX, T::HALF, T::MIN);
    pub const PURPLE: Self = Self::new(T::HALF, T::MIN, T::MAX);
    pub const LIME: Self = Self::new(T::MIN, T::MAX, T::HALF);

    pub const PINK: Self = Self::new(T::MAX, T::HALF, T::HALF);
    pub const TEAL: Self = Self::new(T::HALF, T::MAX, T::HALF);
    pub const LAVENDER: Self = Self::new(T::HALF, T::HALF, T::MAX);

    pub const BROWN: Self = Self::new(T::HALF, T::HALF, T::MIN);
    pub const OLIVE: Self = Self::new(T::HALF, T::MAX, T::HALF);
    pub const MAROON: Self = Self::new(T::HALF, T::MIN, T::HALF);
}

#[derive(derive::Vector)]
pub struct Color4<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: ColorComp> Color4<T> {
    pub const TRANSPARENT: Self = Self::new(T::MIN, T::MIN, T::MIN, T::MIN);

    pub const WHITE: Self = Self::new(T::MAX, T::MAX, T::MAX, T::MAX);
    pub const GRAY: Self = Self::new(T::HALF, T::HALF, T::HALF, T::MAX);
    pub const BLACK: Self = Self::new(T::MIN, T::MIN, T::MIN, T::MAX);

    pub const RED: Self = Self::new(T::MAX, T::MIN, T::MIN, T::MAX);
    pub const GREEN: Self = Self::new(T::MIN, T::MAX, T::MIN, T::MAX);
    pub const BLUE: Self = Self::new(T::MIN, T::MIN, T::MAX, T::MAX);

    pub const YELLOW: Self = Self::new(T::MAX, T::MAX, T::MIN, T::MAX);
    pub const CYAN: Self = Self::new(T::MIN, T::MAX, T::MAX, T::MAX);
    pub const MAGENTA: Self = Self::new(T::MAX, T::MIN, T::MAX, T::MAX);

    pub const ORANGE: Self = Self::new(T::MAX, T::HALF, T::MIN, T::MAX);
    pub const PURPLE: Self = Self::new(T::HALF, T::MIN, T::MAX, T::MAX);
    pub const LIME: Self = Self::new(T::MIN, T::MAX, T::HALF, T::MAX);

    pub const PINK: Self = Self::new(T::MAX, T::HALF, T::HALF, T::MAX);
    pub const TEAL: Self = Self::new(T::HALF, T::MAX, T::HALF, T::MAX);
    pub const LAVENDER: Self = Self::new(T::HALF, T::HALF, T::MAX, T::MAX);

    pub const BROWN: Self = Self::new(T::HALF, T::HALF, T::MIN, T::MAX);
    pub const OLIVE: Self = Self::new(T::HALF, T::MAX, T::HALF, T::MAX);
    pub const MAROON: Self = Self::new(T::HALF, T::MIN, T::HALF, T::MAX);
}

pub trait ColorComp {
    const MIN: Self;
    const HALF: Self;
    const MAX: Self;
}

impl ColorComp for u8 {
    const MIN: Self = u8::MIN;
    const HALF: Self = u8::MAX / 2;
    const MAX: Self = u8::MAX;
}

impl ColorComp for f32 {
    const MIN: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
}

impl ColorComp for f64 {
    const MIN: Self = 0.0;
    const HALF: Self = 0.5;
    const MAX: Self = 1.0;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor3U8(pub [u8; 3]);

impl Color3<u8> {
    pub fn into_arr(self) -> ArrColor3U8 {
        ArrColor3U8([self.r, self.g, self.b])
    }
}

impl ArrColor3U8 {
    pub fn into_struct(self) -> Color3<u8> {
        Color3::new(self.0[0], self.0[1], self.0[2])
    }
}

impl From<Color3<u8>> for ArrColor3U8 {
    fn from(color: Color3<u8>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor3U8> for Color3<u8> {
    fn from(arr: ArrColor3U8) -> Self {
        arr.into_struct()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor3F32(pub [f32; 3]);

impl Color3<f32> {
    pub fn into_arr(self) -> ArrColor3F32 {
        ArrColor3F32([self.r, self.g, self.b])
    }
}

impl ArrColor3F32 {
    pub fn into_struct(self) -> Color3<f32> {
        Color3::new(self.0[0], self.0[1], self.0[2])
    }
}

impl From<Color3<f32>> for ArrColor3F32 {
    fn from(color: Color3<f32>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor3F32> for Color3<f32> {
    fn from(arr: ArrColor3F32) -> Self {
        arr.into_struct()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor3F64(pub [f64; 3]);

impl Color3<f64> {
    pub fn into_arr(self) -> ArrColor3F64 {
        ArrColor3F64([self.r, self.g, self.b])
    }
}

impl ArrColor3F64 {
    pub fn into_struct(self) -> Color3<f64> {
        Color3::new(self.0[0], self.0[1], self.0[2])
    }
}

impl From<Color3<f64>> for ArrColor3F64 {
    fn from(color: Color3<f64>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor3F64> for Color3<f64> {
    fn from(arr: ArrColor3F64) -> Self {
        arr.into_struct()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor4U8(pub [u8; 4]);

impl Color4<u8> {
    pub fn into_arr(self) -> ArrColor4U8 {
        ArrColor4U8([self.r, self.g, self.b, self.a])
    }
}

impl ArrColor4U8 {
    pub fn into_struct(self) -> Color4<u8> {
        Color4::new(self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl From<Color4<u8>> for ArrColor4U8 {
    fn from(color: Color4<u8>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor4U8> for Color4<u8> {
    fn from(arr: ArrColor4U8) -> Self {
        arr.into_struct()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor4F32(pub [f32; 4]);

impl Color4<f32> {
    pub fn into_arr(self) -> ArrColor4F32 {
        ArrColor4F32([self.r, self.g, self.b, self.a])
    }
}

impl ArrColor4F32 {
    pub fn into_struct(self) -> Color4<f32> {
        Color4::new(self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl From<Color4<f32>> for ArrColor4F32 {
    fn from(color: Color4<f32>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor4F32> for Color4<f32> {
    fn from(arr: ArrColor4F32) -> Self {
        arr.into_struct()
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrColor4F64(pub [f64; 4]);

impl Color4<f64> {
    pub fn into_arr(self) -> ArrColor4F64 {
        ArrColor4F64([self.r, self.g, self.b, self.a])
    }
}

impl ArrColor4F64 {
    pub fn into_struct(self) -> Color4<f64> {
        Color4::new(self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl From<Color4<f64>> for ArrColor4F64 {
    fn from(color: Color4<f64>) -> Self {
        color.into_arr()
    }
}

impl From<ArrColor4F64> for Color4<f64> {
    fn from(arr: ArrColor4F64) -> Self {
        arr.into_struct()
    }
}