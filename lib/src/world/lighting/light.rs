use bytemuck::{Pod, Zeroable};
use math::color::{ArrColor3F32, Color3};
use math::to_no_uninit::ToNoUninit;
use math::vector::{vec3, vec3f, ArrVec3F32};

#[derive(Clone)]
pub enum Light {
    Ambient(AmbientLight),
    Directional(DirectionalLight),
    Point(PointLight),
}

impl Light {
    pub fn color(&self) -> Color3<f32> {
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

#[derive(Debug, Copy, Clone)]
pub struct AmbientLight {
    pub color: Color3<f32>,
    pub intensity: f32,
}

impl AmbientLight {
    pub const INACTIVE: Self = Self { color: Color3::WHITE, intensity: 0.0 };
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color3::new(1.0, 1.0, 1.0),
            intensity: 0.5,
        }
    }
}

impl ToNoUninit for AmbientLight {
    type Output = ArrAmbientLight;

    fn to_no_uninit(&self) -> Self::Output {
        ArrAmbientLight(self.color.into(), self.intensity)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrAmbientLight(ArrColor3F32, f32);

#[derive(Debug, Copy, Clone)]
pub struct DirectionalLight {
    pub color: Color3<f32>,
    pub intensity: f32,
    pub direction: vec3f,
}

impl DirectionalLight {
    pub const INACTIVE: Self = Self { color: Color3::WHITE, intensity: 0.0, direction: vec3::ZERO };
}

impl ToNoUninit for DirectionalLight {
    type Output = ArrDirectionalLight;

    fn to_no_uninit(&self) -> Self::Output {
        ArrDirectionalLight(self.color.into(), self.intensity, self.direction.into(), 0)
    }
}

// The fourth element is padding to align the struct to 16 bytes.
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrDirectionalLight(ArrColor3F32, f32, ArrVec3F32, u32);

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub color: Color3<f32>,
    pub intensity: f32,
    pub position: vec3f,
    pub range: f32,
}

impl PointLight {
    pub const INACTIVE: Self = Self {
        color: Color3::WHITE,
        intensity: 0.0,
        position: vec3::ZERO,
        range: 0.0,
    };
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            color: Color3::new(1.0, 1.0, 1.0),
            intensity: 0.5,
            position: vec3f::new(0., 128., 0.),
            range: 10.0,
        }
    }
}

impl ToNoUninit for PointLight {
    type Output = ArrPointLight;

    fn to_no_uninit(&self) -> Self::Output {
        ArrPointLight(self.color.into(), self.intensity, self.position.into(), self.range)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrPointLight(ArrColor3F32, f32, ArrVec3F32, f32);