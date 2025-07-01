use std::ops::Add;

use herbolution_lib::aabb::Aabb;
use math::rotation::Euler;
use math::vec::{vec3d, vec3f, vec3i8, Vec3};

use crate::chunk::map::ChunkMap;
use crate::entity::EntityAbilities;

#[derive(Debug, Clone)]
pub struct EntityBody {
    pub(crate) position: vec3d,
    delta_pos: vec3d,
    pub(crate) rotation: Euler<f32>,
    boundary: Boundary,
    motion: vec3f,
    is_on_ground: bool,
    near_colliders: Vec<Aabb<f64>>,
}

impl EntityBody {
    pub fn new(position: vec3d, boundary: Boundary) -> Self {
        Self {
            position,
            delta_pos: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            boundary,
            motion: Vec3::ZERO,
            is_on_ground: false,
            near_colliders: vec![],
        }
    }

    pub fn update(&mut self, chunk_map: &mut ChunkMap, abilities: EntityAbilities) {
        self.update_translation(chunk_map, &abilities);
    }

    fn update_translation(&mut self, chunk_map: &mut ChunkMap, abilities: &EntityAbilities) {
        const DELTA_TIME: f64 = 1.0 / 60.0;

        let (parallel, perpendicular) = self.rotation.yaw_directions();
        let motion = self.motion.take();
        let direction = vec3d::ZERO
            .add(parallel.cast().unwrap() * motion.x as f64)
            .add(perpendicular.cast().unwrap() * motion.z as f64)
            .normalize();
        let speed = if self.is_on_ground { 3.0 } else { 0.8 } * DELTA_TIME * abilities.speed;

        self.delta_pos += direction * speed;
        if self.is_on_ground || !abilities.is_affected_by_gravity {
            self.delta_pos.y = motion.y as f64 / 3.2;
        }

        if abilities.is_affected_by_gravity {
            self.delta_pos.y -= 1.8 * DELTA_TIME;
        }

        let mut clipped_delta_pos = self.delta_pos;
        let mut bounds = self.bounds();

        chunk_map.get_near_colliders(bounds, &mut self.near_colliders);
        for collider in &self.near_colliders {
            clipped_delta_pos.y = collider.clip_dy_collision(&bounds, clipped_delta_pos.y);
        }

        bounds.add_y(clipped_delta_pos.y);

        for collider in &self.near_colliders {
            clipped_delta_pos.x = collider.clip_dx_collision(&bounds, clipped_delta_pos.x);
        }

        bounds.add_x(clipped_delta_pos.x);

        for collider in &self.near_colliders {
            clipped_delta_pos.z = collider.clip_dz_collision(&bounds, clipped_delta_pos.z);
        }

        bounds.add_z(clipped_delta_pos.z);

        self.is_on_ground = clipped_delta_pos.y != self.delta_pos.y && self.delta_pos.y < 0.0;

        if self.delta_pos.x != clipped_delta_pos.x {
            self.delta_pos.x = 0.0;
        }
        if self.delta_pos.y != clipped_delta_pos.y {
            self.delta_pos.y = 0.0;
        }
        if self.delta_pos.z != clipped_delta_pos.z {
            self.delta_pos.z = 0.0;
        }

        self.position = bounds.min;
        self.position.y = bounds.min.y;

        self.delta_pos.x *= 0.95;
        self.delta_pos.z *= 0.95;

        if self.is_on_ground {
            self.delta_pos.x *= 0.8;
            self.delta_pos.z *= 0.8;
        }
    }

    pub fn bounds(&self) -> Aabb<f64> {
        self.boundary.aabb.cast().unwrap() + self.position
    }

    pub fn eye_pos(&self) -> vec3d {
        self.boundary.eye_offset.cast::<f64>().unwrap() + self.position
    }

    pub fn apply_motion_command(&mut self, command: vec3i8) {
        if command.x != 0 {
            self.motion.x = command.x as f32;
        }

        if command.y != 0 {
            self.motion.y = command.y as f32;
        }

        if command.z != 0 {
            self.motion.z = command.z as f32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boundary {
    pub aabb: Aabb<f32>,
    pub eye_offset: vec3f,
}
