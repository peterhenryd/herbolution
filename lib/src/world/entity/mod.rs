use rapier3d::dynamics::LockedAxes;
use rapier3d::na::Vector3;
use rapier3d::prelude::{Isometry, RigidBodyBuilder, RigidBodyHandle};
use math::vector::vec3f;
use crate::world::physics::Physics;

pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub position: vec3f,
    pub rigid_body_handle: RigidBodyHandle,
}

impl Entity {
    pub fn new(physics: &mut Physics, position: vec3f) -> Self {
        let rigid_body = RigidBodyBuilder::fixed()
            .locked_axes(LockedAxes::ROTATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Z)
            .position(Isometry::new(Vector3::new(position.x, position.y, position.z), Vector3::new(0.0, 0.0, 0.0)))
            .build();
        let rigid_body_handle = physics.rigid_body_set.insert(rigid_body);

        Self {
            position,
            rigid_body_handle,
        }
    }

    pub fn tick(&mut self) {

    }
}