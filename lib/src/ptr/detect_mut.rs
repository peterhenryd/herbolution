use std::mem::take;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct DetectMut<T> {
    value: T,
    was_modified: bool,
}

impl<T> DetectMut<T> {
    pub fn new(value: T) -> Self {
        Self { value, was_modified: true }
    }

    pub fn check(value: &mut Self) -> bool {
        take(&mut value.was_modified)
    }
}

impl<T> Deref for DetectMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DetectMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.was_modified = true;
        &mut self.value
    }
}
