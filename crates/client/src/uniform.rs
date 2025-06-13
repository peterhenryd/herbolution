use crate::gpu::binding::Payload;
use bytemuck::{Pod, Zeroable};
use lib::geo::plane::Plane;
use math::color::{Color, Rgb};
use math::matrix::{mat4f, Mat4};
use math::proj::Proj;
use math::rotation::Euler;
use math::vector::{vec3d, vec3f, vec3i, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera<P> {
    pub position: vec3d,
    pub rot: Euler<f32>,
    pub proj: P,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraPayload {
    pub view_proj: mat4f,
    pub world_position: vec3f,
    pub _padding_0: u32,
    pub world_position_int: vec3i,
    pub _padding_1: u32,
    pub world_position_frac: vec3f,
    pub _padding_2: u32, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frustum([Plane<f32>; 6]);

impl<P> Camera<P> {
    pub fn new(position: vec3d, proj: P) -> Self {
        Self {
            position,
            rot: Euler::IDENTITY,
            proj,
        }
    }

    pub fn view_proj_matrix(&self) -> mat4f
    where 
        P: Proj,
    {
        self.proj.to_matrix() * Mat4::view_origin(self.rot)
    }
}

impl<P: Proj> Payload for Camera<P> {
    type Output = CameraPayload;

    fn payload(&self) -> Self::Output {
        CameraPayload {
            view_proj: self.view_proj_matrix(),
            world_position: self.position.cast().unwrap(),
            _padding_0: 0,
            world_position_int: self.position.cast().unwrap(),
            _padding_1: 0,
            world_position_frac: self.position.fract().cast().unwrap(),
            _padding_2: 0,
        }
    }
}

impl Frustum {
    pub fn new(view_proj: mat4f) -> Self {
        let Mat4 { x, y, z, w } = view_proj;

        let left = Plane::new(x.w + x.x, y.w + y.x, z.w + z.x, w.w + w.x);
        let right = Plane::new(x.w - x.x, y.w - y.x, z.w - z.x, w.w - w.x);
        let top = Plane::new(x.w - x.y, y.w - y.y, z.w - z.y, w.w - w.y);
        let bottom = Plane::new(x.w + x.y, y.w + y.y, z.w + z.y, w.w + w.y);
        let near = Plane::new(x.w + x.z, y.w + y.z, z.w + z.z, w.w + w.z);
        let far = Plane::new(x.w - x.z, y.w - y.z, z.w - z.z, w.w - w.z);

        Self([near, far, left, right, top, bottom].map(Plane::normalize))
    }

    pub fn contains_cube(&self, origin: vec3f, size: f32) -> bool {
        let origin = origin * size;
        let center = origin + Vec3::splat(size / 2.0);
        let radius = (size * size.sqrt()) / 2.0;

        for plane in self.0 {
            let dist = plane.a * center.x
                + plane.b * center.y
                + plane.c * center.z
                + plane.d;
            if dist < -radius {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct World {
    pub ambient_light: vec3f,
    pub light_dir: vec3f,
    pub fog_color: Rgb<f32>,
    pub fog_distance: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            ambient_light: Vec3::splat(0.5),
            light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
            fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
            fog_distance: 150.0,
        }
    }
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

impl Payload for World {
    type Output = WorldPayload;

    fn payload(&self) -> Self::Output {
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