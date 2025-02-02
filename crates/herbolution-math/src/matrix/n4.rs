use crate::vector::vec4;

pub struct Matrix4<T> {
    pub x: vec4<T>,
    pub y: vec4<T>,
    pub z: vec4<T>,
    pub w: vec4<T>,
}

impl<T> Matrix4<T> {
    pub const fn new(x: vec4<T>, y: vec4<T>, z: vec4<T>, w: vec4<T>) -> Self {
        Self { x, y, z, w }
    }

    #[cfg(feature = "bytemuck")]
    pub fn serial(self) -> SerialMatrix4<{ size_of::<T>() }> where T: bytemuck::NoUninit {
        SerialMatrix4([
            self.x.serial().0,
            self.y.serial().0,
            self.z.serial().0,
            self.w.serial().0,
        ])
    }
}

impl<T: Copy> Copy for Matrix4<T> {}

impl<T: Clone> Clone for Matrix4<T> {
    fn clone(&self) -> Self {
        Matrix4 {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            w: self.w.clone(),
        }
    }
}

impl<T: Eq> Eq for Matrix4<T> {}

impl<T: PartialEq> PartialEq for Matrix4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T: Default> Default for Matrix4<T> {
    fn default() -> Self {
        Matrix4 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix4")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct SerialMatrix4<const N: usize>(pub [[[u8; N]; 4]; 4]);