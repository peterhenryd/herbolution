#![feature(duration_constants)]

use std::mem::take;
use std::ops::{Deref, DerefMut};

pub mod counter;
pub mod display;
pub mod fps;
pub mod geometry;
pub mod light;
pub mod time;

pub struct Modify<T> {
    value: T,
    is_dirty: bool,
}

impl<T> Modify<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            is_dirty: true,
        }
    }

    pub fn take_modified(&mut self) -> Option<&T> {
        take(&mut self.is_dirty).then(|| &self.value)
    }
}

impl<T> Deref for Modify<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Modify<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_dirty = true;
        &mut self.value
    }
}

impl<T> From<T> for Modify<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}