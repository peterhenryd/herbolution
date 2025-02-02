pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Vector3<U> {
        Vector3::new(f(self.x), f(self.y), f(self.z))
    }

    #[cfg(feature = "bytemuck")]
    pub fn serial(self) -> SerialVector3<{ size_of::<T>() }> where T: bytemuck::NoUninit {
        SerialVector3([
            bytemuck::try_cast(self.x).unwrap(),
            bytemuck::try_cast(self.y).unwrap(),
            bytemuck::try_cast(self.z).unwrap(),
        ])
    }
}

impl Vector3<f32> {
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        Self::new(self.x / l, self.y / l, self.z / l)
    }
}

impl<T: Copy> Copy for Vector3<T> {}

impl<T: Clone> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        Vector3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Eq> Eq for Vector3<T> {}

impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Default> Default for Vector3<T> {
    fn default() -> Self {
        Vector3::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v3({}, {}. {})", self.x, self.y, self.z)
    }
}

pub struct SerialVector3<const N: usize>(pub [[u8; N]; 3]);