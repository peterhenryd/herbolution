use std::mem::take;
use std::ops::{Deref, DerefMut};

/// A smart pointer that tracks whether the contained value has been modified.
#[derive(Debug)]
pub struct TrackMut<T> {
    value: T,
    was_modified: bool,
}

impl<T> TrackMut<T> {
    /// Creates a new instance with the provided value marked as updated.
    pub fn new(value: T) -> Self {
        Self { value, was_modified: true }
    }

    /// Returns whether the value has been modified, while also marking it as not updated.
    pub fn check(value: &mut Self) -> bool {
        take(&mut value.was_modified)
    }
}

impl<T> Deref for TrackMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for TrackMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.was_modified = true;
        &mut self.value
    }
}
