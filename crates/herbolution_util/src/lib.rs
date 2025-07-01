#![feature(duration_constants)]
extern crate herbolution_math as math;

use std::env::home_dir;
use std::path::PathBuf;

pub mod aabb;
pub mod channel;
pub mod chunk;
pub mod display;
pub mod face;
pub mod fs;
pub mod group_key;
pub mod light;
pub mod motile;
pub mod plane;
pub mod point;
pub mod ptr;
pub mod save;
pub mod time;

pub fn default<T: Default>() -> T {
    T::default()
}

pub fn root_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".herbolution")
}
