use crate::engine::binding::Binding;
use crate::engine::gpu::Gpu;
use crate::world::lighting::light::{AmbientLight, DirectionalLight, Light, PointLight};
use wgpu::ShaderStages;
use crate::engine::storage::Storage;

pub mod light;
pub mod level;

pub struct Lighting {
    ambient_light_set: Storage<AmbientLight>,
    directional_light_set: Storage<DirectionalLight>,
    pub(crate) point_light_set: Storage<PointLight>,
    pub(crate) binding: Binding,
}

impl Lighting {
    pub fn create(gpu: &Gpu) -> Self {
        let ambient_light_set = Storage::create(gpu, "ambient_light_array", vec![AmbientLight::INACTIVE]);
        let directional_light_set = Storage::create(gpu, "ambient_light_array", vec![DirectionalLight::INACTIVE]);
        let point_light_set = Storage::create(gpu, "ambient_light_array", vec![PointLight::INACTIVE]);
        let binding = gpu.build_binding("lighting")
            .with_storage(ShaderStages::FRAGMENT, &ambient_light_set)
            .with_storage(ShaderStages::FRAGMENT, &directional_light_set)
            .with_storage(ShaderStages::FRAGMENT, &point_light_set)
            .finish();

        Self { ambient_light_set, directional_light_set, point_light_set, binding }
    }

    pub fn add_light(&mut self, light: impl Into<Light>) {
        match light.into() {
            Light::Ambient(x) => self.ambient_light_set.edit(|set| set.push(x)),
            Light::Directional(x) => self.directional_light_set.edit(|set| set.push(x)),
            Light::Point(x) => self.point_light_set.edit(|set| set.push(x)),
        }
    }

    pub fn reset(&mut self) {
        self.ambient_light_set.edit(|set| *set = vec![AmbientLight::INACTIVE]);
        self.directional_light_set.edit(|set| *set = vec![DirectionalLight::INACTIVE]);
        self.point_light_set.edit(|set| *set = vec![PointLight::INACTIVE]);
    }
}