use crate::vector::vec3;

pub struct Matrix3<T> {
    pub x: vec3<T>,
    pub y: vec3<T>,
    pub z: vec3<T>,
}

impl<T> Matrix3<T> {
    #[cfg(feature = "bytemuck")]
    pub fn serial(self) -> SerialMatrix3<{ size_of::<T>() }> where T: bytemuck::NoUninit {
        SerialMatrix3([
            self.x.serial().0,
            self.y.serial().0,
            self.z.serial().0,
        ])
    }
}

impl<T: Copy> Copy for Matrix3<T> {}

impl<T: Clone> Clone for Matrix3<T> {
    fn clone(&self) -> Self {
        Matrix3 {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T: Eq> Eq for Matrix3<T> {}

impl<T: PartialEq> PartialEq for Matrix3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Default> Default for Matrix3<T> {
    fn default() -> Self {
        Matrix3 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct SerialMatrix3<const N: usize>(pub [[[u8; N]; 3]; 3]);