use bytemuck::{Pod, Zeroable};
use math::color::Rgb;
use math::vec::vec3f;

#[derive(Debug, Copy, Clone)]
pub struct World {
    pub ambient_light: vec3f,
    pub light_dir: vec3f,
    pub fog_color: Rgb<f32>,
    pub fog_distance: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct WorldPayload {
    ambient_light: vec3f,
    _padding_0: u32,
    light_dir: vec3f,
    _padding_1: u32,
    fog_color: Rgb<f32>,
    fog_distance: f32,
}

impl World {
    pub fn payload(&self) -> WorldPayload {
        WorldPayload {
            ambient_light: self.ambient_light,
            _padding_0: 0,
            light_dir: self.light_dir,
            _padding_1: 0,
            fog_color: self.fog_color,
            fog_distance: self.fog_distance
        }
    }
}

impl<'a> Into<WorldPayload> for &'a World {
    fn into(self) -> WorldPayload {
        self.payload()
    }
}
