use std::time::Duration;

use crate::vector::{Vec3, vec3d};

#[derive(Debug, Clone, PartialEq)]
pub struct Motile {
    pub velocity: vec3d,
    pub gravity: Gravity,
    pub dir: vec3d,
    pub damp: f64,
    pub drive: f64,
    pub jump: f64,
}

impl Motile {
    pub fn simulate(&mut self, position: &mut vec3d, dt: Duration) {
        let damped_velocity = -self.velocity * self.damp;

        let user_force = self.dir * self.drive;
        let up_force = Vec3::new(0.0, self.jump, 0.0);
        let down_force: Vec3<_> = self.gravity.into();

        let acceleration = damped_velocity + user_force + up_force + down_force;

        self.jump += -self.jump * self.damp;
        self.drive += -self.drive * self.damp;
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
            drive: 0.1,
            jump: 0.0,
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
