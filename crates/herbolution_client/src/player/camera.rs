use engine::sculptor::Sculptor;
use engine::Engine;
use gpu::View;
use lib::chunk;
use math::proj::Perspective;
use math::size::size2u;
use math::vector::{vec3i, Vec3};

use crate::world::frustum::Frustum;

/// The camera with additional information used for culling and camera-relative rendering.
#[derive(Debug)]
pub struct PlayerCamera {
    pub(crate) video: gpu::Camera<Perspective>,
    pub(crate) frustum: Frustum,
    pub(crate) chunk_position: vec3i,
}

impl PlayerCamera {
    /// Creates a new instance.
    pub fn new(resolution: size2u, sculptor: &mut Sculptor) -> Self {
        let aspect = resolution.cast::<f32>().unwrap().aspect();
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.001, 500.0);
        let video = gpu::Camera::new(Vec3::ZERO, View::rotatable(), perspective);
        sculptor.update_camera(&video);

        Self {
            frustum: Frustum::new(video.view_proj()),
            video,
            chunk_position: vec3i::ZERO,
        }
    }

    /// Submits the camera to the video state and calculates a new frustum and chunk position.
    pub fn update(&mut self, engine: &mut Engine) {
        engine.video.sculptor.update_camera(&self.video);
        self.frustum = Frustum::new(self.video.view_proj());
        self.chunk_position = self.video.position.cast() / chunk::LENGTH as i32;
    }
}
