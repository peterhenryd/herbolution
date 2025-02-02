pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vector4<U> {
        Vector4::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    #[cfg(feature = "bytemuck")]
    pub fn serial(self) -> SerialVector4<{ size_of::<T>() }> where T: bytemuck::NoUninit {
        SerialVector4([
            bytemuck::try_cast(self.x).unwrap(),
            bytemuck::try_cast(self.y).unwrap(),
            bytemuck::try_cast(self.z).unwrap(),
            bytemuck::try_cast(self.w).unwrap(),
        ])
    }
}

impl Vector4<f32> {
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        Self::new(self.x / l, self.y / l, self.z / l, self.w / l)
    }
}

impl<T: Copy> Copy for Vector4<T> {}

impl<T: Clone> Clone for Vector4<T> {
    fn clone(&self) -> Self {
        Vector4::new(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
    }
}

impl<T: Eq> Eq for Vector4<T> {}

impl<T: PartialEq> PartialEq for Vector4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T: Default> Default for Vector4<T> {
    fn default() -> Self {
        Vector4::new(Default::default(), Default::default(), Default::default(), Default::default())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

pub struct SerialVector4<const N: usize>(pub [[u8; N]; 4]);