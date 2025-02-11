use crate::engine::geometry::plane::Plane;
use crate::world::camera::proj::perspective::Perspective;
use crate::world::camera::Camera;
use crate::world::chunk;
use math::matrix::{mat4, mat4f};
use math::vector::{vec3, vec3i};

pub struct Frustum([Plane<f32>; 6]);

impl Frustum {
    pub fn new(camera: &Camera<Perspective>) -> Self {
        Self(get_planes(camera.to_mat4f()))
    }

    pub fn contains_chunk(&self, chunk_pos: vec3i) -> bool {
        let chunk_world_pos = (chunk_pos * chunk::LENGTH as i32).cast::<f32>();
        let center = chunk_world_pos + vec3::splat(chunk::LENGTH as f32 / 2.0);
        let radius = (chunk::LENGTH as f32 * (chunk::LENGTH as f32).sqrt()) / 2.0;

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

const SHRINK_FACTOR: f32 = 1.5;

fn get_planes(mat4 { x, y, z, w }: mat4f) -> [Plane<f32>; 6] {
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