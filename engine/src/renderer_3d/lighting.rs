use bytemuck::{Pod, Zeroable};
use wgpu::ShaderStages;
use math::color::{ColorConsts, Rgb};
use math::num::traits::ConstZero;
use math::vector::{vec3f, Vec3};
use crate::gpu::handle::Handle;
use crate::gpu::mem::buffer::ArrayBuffer;
use crate::gpu::mem::payload::AutoShaderPayload;

pub struct Lighting {
    pub(crate) ambient_light_set: ArrayBuffer<AmbientLight>,
    pub(crate) directional_light_set: ArrayBuffer<DirectionalLight>,
    pub point_light_set: ArrayBuffer<PointLight>,
}

impl Lighting {
    pub fn create(handle: &Handle) -> Self {
        let ambient_light_set = handle.create_array_buffer(vec![AmbientLight::INACTIVE], ShaderStages::FRAGMENT);
        let directional_light_set = handle.create_array_buffer(vec![DirectionalLight::INACTIVE], ShaderStages::FRAGMENT);
        let point_light_set = handle.create_array_buffer(vec![PointLight::INACTIVE], ShaderStages::FRAGMENT);

        Self {
            ambient_light_set,
            directional_light_set,
            point_light_set,
        }
    }

    pub fn add_light(&mut self, light: impl Into<Light>) {
        match light.into() {
            Light::Ambient(x) => {
                for light in &mut *self.ambient_light_set {
                    if *light == AmbientLight::INACTIVE {
                        *light = x;
                        return;
                    }
                }
            }
            Light::Directional(x) => {
                for light in &mut *self.directional_light_set {
                    if *light == DirectionalLight::INACTIVE {
                        *light = x;
                        return;
                    }
                }
            }
            Light::Point(x) => {
                for light in &mut *self.point_light_set {
                    if *light == PointLight::INACTIVE {
                        *light = x;
                        return;
                    }
                }
            }
        }
    }

    pub fn submit(&mut self, handle: &Handle) {
        self.ambient_light_set.submit(handle);
        self.directional_light_set.submit(handle);
        self.point_light_set.submit(handle);
    }

    pub fn reset(&mut self) {
        *self.ambient_light_set = vec![AmbientLight::INACTIVE];
        *self.directional_light_set = vec![DirectionalLight::INACTIVE];
        *self.point_light_set = vec![PointLight::INACTIVE];
    }
}

#[derive(Clone)]
pub enum Light {
    Ambient(AmbientLight),
    Directional(DirectionalLight),
    Point(PointLight),
}

impl Light {
    pub fn color(&self) -> Rgb<f32> {
        match self {
            Light::Ambient(light) => light.color,
            Light::Directional(light) => light.color,
            Light::Point(light) => light.color,
        }
    }

    pub fn intensity(&self) -> f32 {
        match self {
            Light::Ambient(light) => light.intensity,
            Light::Directional(light) => light.intensity,
            Light::Point(light) => light.intensity,
        }
    }
}

impl From<AmbientLight> for Light {
    fn from(light: AmbientLight) -> Self {
        Light::Ambient(light)
    }
}

impl From<DirectionalLight> for Light {
    fn from(light: DirectionalLight) -> Self {
        Light::Directional(light)
    }
}

impl From<PointLight> for Light {
    fn from(light: PointLight) -> Self {
        Light::Point(light)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct AmbientLight {
    pub color: Rgb<f32>,
    pub intensity: f32,
}

impl AmbientLight {
    pub const INACTIVE: Self = Self {
        color: Rgb::WHITE,
        intensity: 0.0,
    };
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Rgb::new(1.0, 1.0, 1.0),
            intensity: 0.5,
        }
    }
}

impl AutoShaderPayload for AmbientLight {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct DirectionalLight {
    pub color: Rgb<f32>,
    pub intensity: f32,
    pub direction: vec3f,
}

impl DirectionalLight {
    pub const INACTIVE: Self = Self {
        color: Rgb::WHITE,
        intensity: 0.0,
        direction: Vec3::ZERO,
    };
}

impl AutoShaderPayload for DirectionalLight {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct PointLight {
    pub color: Rgb<f32>,
    pub intensity: f32,
    pub position: vec3f,
    pub range: f32,
}

impl PointLight {
    pub const INACTIVE: Self = Self {
        color: Rgb::WHITE,
        intensity: 0.0,
        position: Vec3::ZERO,
        range: 0.0,
    };
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            color: Rgb::new(1.0, 1.0, 1.0),
            intensity: 0.5,
            position: vec3f::new(0., 128., 0.),
            range: 10.0,
        }
    }
}

impl AutoShaderPayload for PointLight {}