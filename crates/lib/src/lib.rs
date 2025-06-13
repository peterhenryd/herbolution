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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GroupKeyBuf {
    pub group: String,
    pub key: String,
}

impl GroupKeyBuf {
    pub fn new(group: String, key: String) -> Self {
        Self { group, key }
    }

    pub fn as_ref(&self) -> GroupKey {
        GroupKey {
            group: &self.group,
            key: &self.key,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GroupKey<'a> {
    pub group: &'a str,
    pub key: &'a str,
}

impl<'a> From<&'a GroupKeyBuf> for GroupKey<'a> {
    fn from(value: &'a GroupKeyBuf) -> Self {
        Self {
            group: &value.group,
            key: &value.key,
        }
    }
}

impl Equivalent<GroupKeyBuf> for GroupKey<'_> {
    fn equivalent(&self, other: &GroupKeyBuf) -> bool {
        self.group == &other.group && self.key == &other.key
    }
}