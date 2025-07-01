#![feature(duration_constants)]
extern crate herbolution_math as math;

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
