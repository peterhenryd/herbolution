use bytemuck::{Pod, Zeroable};
use lib::color::{Color, Rgb, Rgba};
use lib::vector::{Vec4, vec3f, vec4f};

#[derive(Debug, Copy, Clone)]
pub struct World {
    pub ambient_light: vec3f,
    pub light_dir: vec3f,
    pub fog_color: Rgb<f32>,
    pub fog_distance: f32,
    pub fog_density: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct WorldPayload {
    ambient_light: vec4f,
    light_dir: vec4f,
    fog_color: Rgba<f32>,
    fog_distance_density: vec4f,
}

impl World {
    pub fn payload(&self) -> WorldPayload {
        WorldPayload {
            ambient_light: self.ambient_light.extend(0.0),
            light_dir: self.light_dir.extend(0.0),
            fog_color: self.fog_color.to_rgba(),
            fog_distance_density: Vec4::new(self.fog_distance, self.fog_density, 0.0, 0.0),
        }
    }
}

impl<'a> Into<WorldPayload> for &'a World {
    fn into(self) -> WorldPayload {
        self.payload()
    }
}
