use serde::{Deserialize, Serialize};

use crate::vec::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3<T> {
    pub const fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self { x, y, z }
    }
}
