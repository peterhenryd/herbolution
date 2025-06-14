#![feature(duration_constants)]

use std::mem::take;
use std::ops::{Deref, DerefMut};

use hashbrown::Equivalent;

pub mod display;
pub mod fps;
pub mod fs;
pub mod geo;
pub mod light;
pub mod time;

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

impl<T> From<T> for TrackMut<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GroupKeyBuf {
    pub group: String,
    pub key: String,
}

impl GroupKeyBuf {
    pub fn new(group: String, key: String) -> Self {
        Self { group, key }
    }

    pub fn as_ref(&self) -> GroupKey<'_> {
        GroupKey { group: &self.group, key: &self.key }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GroupKey<'a> {
    pub group: &'a str,
    pub key: &'a str,
}

impl<'a> From<&'a GroupKeyBuf> for GroupKey<'a> {
    fn from(value: &'a GroupKeyBuf) -> Self {
        Self { group: &value.group, key: &value.key }
    }
}

impl Equivalent<GroupKeyBuf> for GroupKey<'_> {
    fn equivalent(&self, other: &GroupKeyBuf) -> bool {
        self.group == &other.group && self.key == &other.key
    }
}
