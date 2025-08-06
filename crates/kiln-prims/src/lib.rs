#![feature(const_precise_live_drops)]
#![feature(const_trait_impl)]

pub mod aabb;
pub mod camera;
pub mod color;
pub mod macros;
pub mod matrix;
pub mod proj;
pub mod rotation;
pub mod size;
pub mod spatial;
pub mod transform;
pub mod vector;

pub fn calculate_atlas_size(count: u32, dimension: u32) -> u32 {
    if count == 0 || dimension == 0 {
        return 0;
    }

    let textures_per_side = (count as f64).sqrt().ceil() as u32;
    (textures_per_side * dimension).next_power_of_two()
}
