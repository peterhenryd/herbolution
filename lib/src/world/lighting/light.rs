use bytemuck::{Pod, Zeroable};
use math::color::{ArrColor3F32, Color3};
use math::vector::{vec3f, ArrVec3F32};
use crate::engine::as_no_uninit::AsNoUninit;

#[derive(Clone)]
pub enum LightKind {
    Ambient(AmbientLight),
    Directional(DirectionalLight),
    Point(PointLight),
}

pub enum LightType {
    Ambient,
    Directional,
    Point,
}

pub trait Light {
    const TYPE: LightType;

    fn color(&self) -> Color3<f32>;

    fn intensity(&self) -> f32;

    fn into_kind(self) -> Option<LightKind>;
}

#[derive(Debug, Copy, Clone)]
pub struct AmbientLight {
    pub color: Color3<f32>,
    pub intensity: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color3::new(1.0, 1.0, 1.0),
            intensity: 1.0,
        }
    }
}

impl Light for AmbientLight {
    const TYPE: LightType = LightType::Ambient;

    fn color(&self) -> Color3<f32> {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn into_kind(self) -> Option<LightKind> {
        Some(LightKind::Ambient(self))
    }
}

impl AsNoUninit for AmbientLight {
    type Output = ArrAmbientLight;

    fn as_no_uninit(&self) -> Self::Output {
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

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            color: Color3::new(1.0, 1.0, 1.0),
            intensity: 1.0,
            direction: vec3f::new(0.0, -1.0, 0.0),
        }
    }
}

impl Light for DirectionalLight {
    const TYPE: LightType = LightType::Directional;

    fn color(&self) -> Color3<f32> {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn into_kind(self) -> Option<LightKind> {
        Some(LightKind::Directional(self))
    }
}

impl AsNoUninit for DirectionalLight {
    type Output = ArrDirectionalLight;

    fn as_no_uninit(&self) -> Self::Output {
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

impl Default for PointLight {
    fn default() -> Self {
        Self {
            color: Color3::new(1.0, 1.0, 1.0),
            intensity: 1.0,
            position: vec3f::zero(),
            range: 10.0,
        }
    }
}

impl Light for PointLight {
    const TYPE: LightType = LightType::Point;

    fn color(&self) -> Color3<f32> {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn into_kind(self) -> Option<LightKind> {
        Some(LightKind::Point(self))
    }
}

impl AsNoUninit for PointLight {
    type Output = ArrPointLight;

    fn as_no_uninit(&self) -> Self::Output {
        ArrPointLight(self.color.into(), self.intensity, self.position.into(), self.range)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrPointLight(ArrColor3F32, f32, ArrVec3F32, f32);