use crate::world::transform::Rotation;
use math::angle::Rad;
use math::vector::{vec3, vec3f};
use std::f64::consts::FRAC_PI_2;
use std::time::Duration;
use crate::engine::geometry::cuboid::Cuboid;
use crate::engine::uniform::Uniform;
use crate::world::camera::Camera;
use crate::world::camera::proj::perspective::Perspective;
use crate::world::chunk::map::ChunkMap;
use crate::world::chunk::material::Material;

#[derive(Debug)]
pub struct Player {
    pub position: vec3f,
    pub rotation: Rotation,
    pub health: u32,
    pub(crate) motion: PlayerMotion,
    pub break_cube: u32,
    pub capabilities: Capabilities,
}

impl Player {
    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap, camera: &mut Uniform<Camera<Perspective>>) {
        while self.break_cube > 0 {
            self.break_cube -= 1;

            let Some(pos) = chunk_map.cast_ray(
                camera.transform.position,
                camera.transform.rotation.into_center(),
            ) else {
                continue;
            };

            chunk_map.set_cube(pos, Material::Air);
        }

        if let Some((dx, dy)) = self.motion.rotation.take() {
            self.rotation.yaw += -dx.to_radians() * dt.as_secs_f64() * 4.0;
            self.rotation.pitch += -dy.to_radians() * dt.as_secs_f64() * 4.0;
        }

        self.rotation.pitch = Rad(self.rotation.pitch.0.clamp(-FRAC_PI_2, FRAC_PI_2));
        self.position += self.motion.get_velocity(self.rotation) * dt.as_secs_f32();

        /*
        // Poor man's gravity
        if !chunk_map.check_collision(self.get_collision_box()) {
            self.position.y -= self.motion.speed * dt.as_secs_f32();
            self.motion.speed += self.motion.acceleration * dt.as_secs_f32();
            self.motion.speed = self.motion.speed.max(self.motion.max_speed);
        } else {
            self.motion.speed -= 0.1 * dt.as_secs_f32();
            self.motion.speed = self.motion.speed.max(0.0);
        }
         */

        if camera.transform.position != self.position
            || camera.transform.rotation != self.rotation
        {
            camera.edit(|c| {
                c.transform.position = self.position;
                c.transform.rotation = self.rotation;
            });
        }
    }

    pub fn get_collision_box(&self) -> Cuboid<f32> {
        let half = vec3::new(0.3, 1.8, 0.3);
        Cuboid::new(self.position - half, self.position + half)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerMotion {
    // Units per second.
    pub forward: f32,
    pub backward: f32,
    pub leftward: f32,
    pub rightward: f32,
    pub upward: f32,
    pub downward: f32,
    pub rotation: Option<(f64, f64)>,
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
}

impl PlayerMotion {
    pub(crate) fn reset(&mut self) {
        self.forward = 0.0;
        self.backward = 0.0;
        self.leftward = 0.0;
        self.rightward = 0.0;
        self.upward = 0.0;
        self.downward = 0.0;
        self.rotation = None;
    }
}

impl Default for PlayerMotion {
    fn default() -> Self {
        Self {
            forward: 0.0,
            backward: 0.0,
            leftward: 0.0,
            rightward: 0.0,
            upward: 0.0,
            downward: 0.0,
            rotation: None,
            speed: 0.0,
            acceleration: 0.098,
            max_speed: 36.0,
        }
    }
}

impl PlayerMotion {
    pub fn get_velocity(&mut self, rotation: Rotation) -> vec3f {
        let (straight, side) = rotation.into_directions();
        let (straight, side) = (straight.cast() * 18.0, side.cast() * 18.0);
        let up = vec3::y() * 18.0;

        let mut direction = vec3::zero();
        direction += straight * (self.forward - self.backward);
        direction += side * (self.leftward - self.rightward) * 0.75;
        direction += up * (self.upward - self.downward) * 0.9;

        direction
    }
}

impl Player {
    pub fn new(position: vec3f) -> Self {
        Self {
            position,
            rotation: Rotation::default(),
            health: 100,
            motion: PlayerMotion::default(),
            break_cube: 0,
            capabilities: Capabilities::new(),
        }
    }
}

#[derive(Debug)]
pub struct Capabilities {
    pub can_fly: bool,
}

impl Capabilities {
    pub fn new() -> Self {
        Self {
            can_fly: false,
        }
    }
}