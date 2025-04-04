use crate::world::chunk::map::ChunkMap;
use crate::world::entity::EntityAbilities;
use lib::geometry::cuboid::Cuboid;
use math::num::traits::ConstZero;
use math::vector::{vec3f, vec3i8, Vec3};
use std::ops::Add;
use math::angle::Rad;
use math::rotation::Euler;
use crate::DELTA_TIME;

#[derive(Debug, Clone)]
pub struct EntityBody {
    /// The position of the entity in the world.
    pub(crate) pos: vec3f,
    delta_pos: vec3f,
    pub(crate) rotation: Euler<Rad<f32>>,
    boundary: Boundary,
    motion: vec3f,
    is_on_ground: bool,
    near_colliders: Vec<Cuboid<f32>>,
}

impl EntityBody {
    pub fn new(position: vec3f, boundary: Boundary) -> Self {
        Self {
            pos: position,
            delta_pos: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            boundary,
            motion: Vec3::ZERO,
            is_on_ground: false,
            near_colliders: vec![],
        }
    }

    pub fn update(&mut self, chunk_map: &mut ChunkMap, abilities: EntityAbilities) {
        self.apply_translation(chunk_map, &abilities);
    }

    fn apply_translation(
        &mut self,
        chunk_map: &mut ChunkMap,
        _: &EntityAbilities,
    ) {
        let (parallel, perpendicular) = self.rotation.yaw_directions();
        let motion = self.motion.take();
        let direction = vec3f::ZERO
            .add(parallel * motion.x)
            .add(perpendicular * motion.z)
            .normalize();
        let speed = if self.is_on_ground { 2.0 } else { 0.5 } * DELTA_TIME;

        self.delta_pos += direction * speed;
        if self.is_on_ground {
            self.delta_pos.y = motion.y / 3.0;
        }

        self.delta_pos.y -= 1.8 * DELTA_TIME;

        let mut clipped_delta_pos = self.delta_pos;
        let mut bounds = self.bounds();

        chunk_map.get_near_colliders(bounds, &mut self.near_colliders);
        for collider in &self.near_colliders {
            collider.clip_collision(&bounds, &mut clipped_delta_pos);
        }

        bounds += clipped_delta_pos;

        self.is_on_ground = clipped_delta_pos.y != self.delta_pos.y && self.delta_pos.y < 0.0;

        self.delta_pos.zero_inequalities(&clipped_delta_pos);

        self.pos = bounds.center();
        self.pos.y = bounds.min.y + self.boundary.eye_offset.y;

        self.delta_pos.x *= 0.95;
        self.delta_pos.z *= 0.95;

        if self.is_on_ground {
            self.delta_pos.x *= 0.8;
            self.delta_pos.z *= 0.8;
        }
    }

    fn bounds(&self) -> Cuboid<f32> {
        self.boundary.cuboid + self.pos
    }

    pub fn eye_position(&self) -> vec3f {
        self.boundary.eye_offset + self.pos
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
    pub cuboid: Cuboid<f32>,
    pub eye_offset: vec3f,
}