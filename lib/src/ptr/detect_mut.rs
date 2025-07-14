use std::mem::take;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DetectMut<T> {
    value: T,
    was_modified: bool,
}

impl<T> DetectMut<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self { value, was_modified: true }
    }

    #[inline]
    pub fn check(value: &mut Self) -> bool {
        take(&mut value.was_modified)
    }
}

impl<T> Deref for DetectMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for DetectMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.was_modified = true;
        &mut self.value
    }
}
