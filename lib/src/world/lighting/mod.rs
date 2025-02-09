use crate::engine::binding::Binding;
use crate::engine::gpu::Gpu;
use crate::world::lighting::light::{AmbientLight, DirectionalLight, Light, LightKind, PointLight};
use wgpu::ShaderStages;
use crate::engine::storage::Storage;

pub mod light;

pub struct Lighting {
    ambient_light_set: Storage<AmbientLight>,
    directional_light_set: Storage<DirectionalLight>,
    point_light_set: Storage<PointLight>,
    pub(crate) binding: Binding,
}

impl Lighting {
    pub fn create(gpu: &Gpu) -> Self {
        let ambient_light_set = Storage::create(gpu, "ambient_light_array", vec![AmbientLight::default()]);
        let directional_light_set = Storage::create(gpu, "ambient_light_array", vec![DirectionalLight::default()]);
        let point_light_set = Storage::create(gpu, "ambient_light_array", vec![PointLight::default()]);
        let binding = gpu.build_binding("lighting")
            .with_storage(ShaderStages::FRAGMENT, &ambient_light_set)
            .with_storage(ShaderStages::FRAGMENT, &directional_light_set)
            .with_storage(ShaderStages::FRAGMENT, &point_light_set)
            .finish();

        Self { ambient_light_set, directional_light_set, point_light_set, binding }
    }

    pub fn add_light<L: Light>(&mut self, light: L) {
        match light.into_kind() {
            Some(LightKind::Ambient(light)) => {
                self.ambient_light_set.edit(|set| set.push(light));
            },
            Some(LightKind::Directional(light)) => {
                self.directional_light_set.edit(|set| set.push(light));
            }
            Some(LightKind::Point(light)) => {
                self.point_light_set.edit(|set| set.push(light));
            }
            _ => {}
        }
    }
}