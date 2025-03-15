use lib::geometry::plane::Plane;
use math::matrix::{mat4f, Mat4};
use math::projection::perspective::Perspective;
use math::vector::{vec3i, Vec3};
use crate::camera::Camera;

pub struct Frustum([Plane<f32>; 6]);

impl Frustum {
    pub fn new(camera: &Camera<Perspective>) -> Self {
        Self(get_planes(camera.view_projection_matrix()))
    }

    pub fn contains_chunk(&self, chunk_pos: vec3i, length: i32) -> bool {
        let chunk_world_pos = (chunk_pos * length).cast::<f32>().unwrap();
        let center = chunk_world_pos + Vec3::splat(length as f32 / 2.0);
        let radius = (length as f32 * (length as f32).sqrt()) / 2.0;

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

const SHRINK_FACTOR: f32 = 0.1;

fn get_planes(Mat4 { x, y, z, w }: mat4f) -> [Plane<f32>; 6] {
    let left = Plane::new(x.w + x.x, y.w + y.x, z.w + z.x, w.w + w.x);
    let right = Plane::new(x.w - x.x, y.w - y.x, z.w - z.x, w.w - w.x);
    let top = Plane::new(x.w - x.y, y.w - y.y, z.w - z.y, w.w - w.y);
    let bottom = Plane::new(x.w + x.y, y.w + y.y, z.w + z.y, w.w + w.y);
    let near = Plane::new(x.w + x.z, y.w + y.z, z.w + z.z, w.w + w.z);
    let far = Plane::new(x.w - x.z, y.w - y.z, z.w - z.z, w.w - w.z);

    [near, far, left, right, top, bottom]
        .map(Plane::normalize)
        .map(|plane| Plane { d: plane.d - SHRINK_FACTOR, ..plane })
}