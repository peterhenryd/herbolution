use herbolution_math::spatial::aabb::Aabb;
use math::rotation::Euler;
use math::vector::{vec3d, vec3f, vec3i8, Vec3};
use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Deref, DerefMut};

use crate::chunk::map::ChunkMap;

#[derive(Debug, Clone)]
pub struct EntityBody {
    position: vec3d,
    delta_pos: vec3d,
    rotation: Euler<f32>,
    boundary: Boundary,
    motion: vec3f,
    is_on_ground: bool,
    near_colliders: Vec<Aabb<f64>>,
    pub abilities: EntityAbilities,
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAbilities {
    pub is_affected_by_gravity: bool,
    pub speed: f64,
}

impl EntityBody {
    pub fn new(position: vec3d, boundary: Boundary, abilities: EntityAbilities) -> Self {
        Self {
            position,
            delta_pos: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            boundary,
            motion: Vec3::ZERO,
            is_on_ground: false,
            abilities,
            near_colliders: vec![],
        }
    }

    pub fn update(&mut self, chunk_map: &mut ChunkMap) {
        self.update_translation(chunk_map);
    }

    fn update_translation(&mut self, chunk_map: &mut ChunkMap) {
        const DELTA_TIME: f64 = 1.0 / 60.0;

        let (parallel, perpendicular) = self.rotation.yaw_directions();
        let motion = self.motion.take();
        let direction = vec3d::ZERO
            .add(parallel.cast() * motion.x as f64)
            .add(perpendicular.cast() * motion.z as f64)
            .normalize();
        let speed = if self.is_on_ground { 2.25 } else { 0.75 } * self.abilities.speed * DELTA_TIME;

        self.delta_pos += direction * speed;
        if self.is_on_ground || !self.abilities.is_affected_by_gravity {
            self.delta_pos.y = motion.y as f64 / 4.0;
        }

        if self.abilities.is_affected_by_gravity {
            self.delta_pos.y -= 1.1 * DELTA_TIME;
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
        self.boundary.aabb.cast() + self.position
    }

    pub fn eye_position(&self) -> vec3d {
        self.position + self.boundary.eye_offset.cast()
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

    pub fn position(&self) -> vec3d {
        self.position
    }

    pub fn rotation(&self) -> &Euler<f32> {
        &self.rotation
    }

    pub fn rotation_mut(&mut self) -> RotateEntity<'_> {
        RotateEntity { euler: &mut self.rotation }
    }
}

#[derive(Debug)]
pub struct RotateEntity<'a> {
    euler: &'a mut Euler<f32>,
}

impl<'a> Deref for RotateEntity<'a> {
    type Target = Euler<f32>;

    fn deref(&self) -> &Self::Target {
        self.euler
    }
}

impl<'a> DerefMut for RotateEntity<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.euler
    }
}

impl Drop for RotateEntity<'_> {
    fn drop(&mut self) {
        self.euler.pitch = self
            .euler
            .pitch
            .clamp(-FRAC_PI_2 + f32::EPSILON, FRAC_PI_2 - f32::EPSILON);
    }
}

#[derive(Debug, Clone)]
pub struct Boundary {
    pub aabb: Aabb<f32>,
    pub eye_offset: vec3f,
}
