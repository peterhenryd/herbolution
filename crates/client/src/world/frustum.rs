use lib::geo::plane::Plane;
use math::mat::{mat4f, Mat4};
use math::vec::{vec3f, Vec3};

/// A frustum used for culling objects that are outside the camera's view volume.
#[derive(Debug, Clone, PartialEq)]
pub struct Frustum([Plane<f32>; 6]);

impl Frustum {
    /// Creates a new frustum from the provided view-projection mat.
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

    /// Checks if the frustum contains a cube at the specified origin with the given ext.
    pub fn contains_cube(&self, origin: vec3f, size: f32) -> bool {
        let origin = origin * size;
        let center = origin + Vec3::splat(size / 2.0);
        let radius = (size * size.sqrt()) / 2.0;

        for plane in self.0 {
            let dist = plane.a * center.x + plane.b * center.y + plane.c * center.z + plane.d;
            if dist < -radius {
                return false;
            }
        }

        true
    }
}
