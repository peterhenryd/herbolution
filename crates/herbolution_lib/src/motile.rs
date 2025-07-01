use std::time::Duration;

use math::vec::{vec3d, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Motile {
    velocity: vec3d,
    gravity: Gravity,
    dir: vec3d,
    damp: f64,
    drive: f64,
}

impl Motile {
    pub fn new(velocity: vec3d, gravity: Gravity, dir: vec3d, damp: f64, drive: f64) -> Self {
        Self {
            velocity,
            gravity,
            dir,
            damp,
            drive,
        }
    }

    pub fn simulate(&mut self, position: &mut vec3d, dt: Duration) {
        let gravity: Vec3<_> = self.gravity.into();

        let damping_force = -self.velocity * self.damp;
        let driving_force = self.dir * self.drive;

        let acceleration = gravity + damping_force + driving_force;

        self.velocity += acceleration * dt.as_secs_f64();

        *position += self.velocity * dt.as_secs_f64();
    }
}

impl Default for Motile {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            gravity: Gravity(9.81),
            dir: Vec3::ZERO,
            damp: 0.1,
            drive: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Gravity(pub f64);

impl Into<vec3d> for Gravity {
    fn into(self) -> vec3d {
        Vec3::new(0.0, -self.0, 0.0)
    }
}
