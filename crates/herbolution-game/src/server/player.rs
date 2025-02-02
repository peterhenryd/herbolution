use crate::engine::camera::ViewRotation;
use crate::game::message::GameMessage;
use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub position: Vec3,
    pub rotation: ViewRotation,
    health: u32,
    pub(crate) motion: PlayerMotion,
}

impl Player {
    pub fn update(&mut self) -> [GameMessage; 2] {
        if let Some((dx, dy)) = self.motion.rotation.take() {
            self.rotation.yaw += -dx.to_radians() as f32;
            self.rotation.pitch += -dy.to_radians() as f32;
        }

        self.rotation.pitch = self.rotation.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

        let velocity = self.motion.get_velocity(self.rotation);
        self.position += velocity;

        [
            GameMessage::MovePlayer { velocity },
            GameMessage::RotatePlayer {
                rotation: self.rotation,
            },
        ]
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
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
    max_speed: f32,
    acceleration: f32,
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
            max_speed: 12.0,
            acceleration: 0.098,
        }
    }
}

impl PlayerMotion {
    pub fn get_velocity(&mut self, rotation: ViewRotation) -> Vec3 {
        let (straight, side) = rotation.into_directions();
        let up = Vec3::Y;

        let mut direction = Vec3::ZERO;
        direction += straight * (self.forward - self.backward);
        direction += side * (self.leftward - self.rightward) * 0.75;
        direction += up * (self.upward - self.downward) * 0.9;

        if direction == Vec3::ZERO {
            self.speed = 0.0;
        }

        direction
    }
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: ViewRotation::default(),
            health: 100,
            motion: PlayerMotion::default(),
        }
    }
}