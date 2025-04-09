use crate::gpu::mem::payload::ShaderPayload;
use bytemuck::{Pod, Zeroable};
use lib::geometry::plane::Plane;
use math::angle::Rad;
use math::matrix::{mat4f, Mat4};
use math::proj::Proj;
use math::rotation::Euler;
use math::vector::{vec3f, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera<P> {
    pub pos: vec3f,
    pub rot: Euler<Rad<f32>>,
    pub proj: P,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraShaderPayload {
    pub view_proj: mat4f,
    pub pos: vec3f,
    pub _padding: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frustum([Plane<f32>; 6]);

impl<P> Camera<P> {
    pub fn new(pos: vec3f, proj: P) -> Self {
        Self {
            pos,
            rot: Euler::IDENTITY,
            proj,
        }
    }

    pub fn view_proj_matrix(&self) -> mat4f
    where P: Proj
    {
        self.proj.to_matrix() * Mat4::view(self.pos, self.rot)
    }
}

impl<P: Proj> ShaderPayload for Camera<P> {
    type Output<'a> = CameraShaderPayload where Self: 'a;

    fn payload(&self) -> Self::Output<'_> {
        CameraShaderPayload {
            view_proj: self.view_proj_matrix(),
            pos: self.pos,
            _padding: 0,
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