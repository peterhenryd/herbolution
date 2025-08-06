use std::ops::{Add, AddAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

pub const CHUNK_EXP: u32 = 5;
pub const CHUNK_LENGTH: usize = 32;
pub const CHUNK_AREA: usize = CHUNK_LENGTH.pow(2);
pub const CHUNK_VOLUME: usize = CHUNK_LENGTH.pow(3);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { current: value, max: value }
    }

    pub fn get(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn percent(&self) -> f32 {
        if self.max == 0.0 { 0.0 } else { self.current / self.max }
    }
}

impl Add<f32> for Health {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            current: self.current.add(rhs).min(self.max),
            max: self.max,
        }
    }
}

impl AddAssign<f32> for Health {
    fn add_assign(&mut self, rhs: f32) {
        self.current = self.current.add(rhs).min(self.max);
    }
}

impl Sub<f32> for Health {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            current: (self.current - rhs).max(0.0),
            max: self.max,
        }
    }
}

impl SubAssign<f32> for Health {
    fn sub_assign(&mut self, rhs: f32) {
        self.current = (self.current - rhs).max(0.0);
    }
}
