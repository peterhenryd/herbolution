use crate::chunk::map::ChunkMap;
use herbolution_math::spatial::aabb::Aabb;
use math::rotation::Euler;
use math::vector::{vec3d, vec3f, Vec3};
use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Deref, DerefMut};
use std::time::Duration;

const GRAVITY: f64 = 64.0;
const JUMP_FORCE: f64 = 12.0;
const GROUND_FRICTION: f64 = 11.0;
const AIR_FRICTION: f64 = 2.0;

#[derive(Debug, Clone)]
pub struct EntityBody {
    position: vec3d,
    velocity: vec3d,
    rotation: Euler<f32>,
    boundary: Boundary,
    pub(crate) motion: vec3f,
    is_on_ground: bool,
    near_colliders: Vec<Aabb<f64>>,
    pub attrs: EntityAttrs,
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAttrs {
    pub has_gravity: bool,
    pub acceleration_rate: f64,
    pub terminal_velocity: vec3d,
}

impl EntityBody {
    pub fn new(position: vec3d, boundary: Boundary, attrs: EntityAttrs) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            boundary,
            motion: Vec3::ZERO,
            is_on_ground: false,
            attrs,
            near_colliders: vec![],
        }
    }

    pub fn update(&mut self, chunk_map: &mut ChunkMap, dt: Duration) {
        self.apply_physics_and_collision(chunk_map, dt);
    }

    fn apply_physics_and_collision(&mut self, chunk_map: &mut ChunkMap, dt: Duration) {
        let dt_secs = dt.as_secs_f64();

        self.apply_input_to_velocity(dt_secs);

        if self.attrs.has_gravity {
            self.velocity.y -= GRAVITY * dt_secs;
        }

        self.apply_friction(dt_secs);

        self.velocity = self.velocity.min(self.attrs.terminal_velocity);

        let step = self.velocity * dt_secs;
        let clipped_step = self.collide_and_clip(chunk_map, step);

        self.update_state_after_collision(step, clipped_step);

        self.position += clipped_step;
    }

    fn apply_input_to_velocity(&mut self, dt_secs: f64) {
        let (parallel, perpendicular) = self.rotation.yaw_directions();
        let direction = vec3d::ZERO
            .add(parallel.cast() * self.motion.x as f64)
            .add(perpendicular.cast() * self.motion.z as f64)
            .normalize();

        let mut speed = self.attrs.acceleration_rate;
        if self.is_on_ground || !self.attrs.has_gravity {
            speed *= 3.0;
        }

        self.velocity.x += direction.x * speed * dt_secs;
        self.velocity.z += direction.z * speed * dt_secs;

        if self.is_on_ground || !self.attrs.has_gravity {
            self.velocity.y = JUMP_FORCE * self.motion.y as f64;
        }
    }

    fn apply_friction(&mut self, dt_secs: f64) {
        if self.velocity.length_squared() < f64::EPSILON * f64::EPSILON {
            self.velocity.x = 0.0;
            self.velocity.z = 0.0;
            return;
        }

        let friction = if self.is_on_ground { GROUND_FRICTION } else { AIR_FRICTION };
        let friction_step = (-friction * dt_secs).exp();

        self.velocity.x *= friction_step;
        self.velocity.z *= friction_step;
    }

    fn collide_and_clip(&mut self, chunk_map: &mut ChunkMap, step: vec3d) -> vec3d {
        let mut clipped_step = step;
        let mut bounds = self.bounds();

        chunk_map.get_near_colliders(bounds, &mut self.near_colliders);

        for collider in &self.near_colliders {
            clipped_step.y = collider.clip_dy_collision(&bounds, clipped_step.y);
        }
        bounds.add_y(clipped_step.y);

        for collider in &self.near_colliders {
            clipped_step.x = collider.clip_dx_collision(&bounds, clipped_step.x);
        }
        bounds.add_x(clipped_step.x);

        for collider in &self.near_colliders {
            clipped_step.z = collider.clip_dz_collision(&bounds, clipped_step.z);
        }

        clipped_step
    }

    fn update_state_after_collision(&mut self, delta_pos: vec3d, clipped_delta_pos: vec3d) {
        self.is_on_ground = (delta_pos.y < 0.0) && (clipped_delta_pos.y != delta_pos.y);

        if clipped_delta_pos.x != delta_pos.x {
            self.velocity.x = 0.0;
        }
        if clipped_delta_pos.y != delta_pos.y {
            self.velocity.y = 0.0;
        }
        if clipped_delta_pos.z != delta_pos.z {
            self.velocity.z = 0.0;
        }
    }

    pub fn bounds(&self) -> Aabb<f64> {
        self.boundary.aabb.cast() + self.position
    }

    pub fn eye_position(&self) -> vec3d {
        self.position + self.boundary.eye_offset.cast()
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

impl Deref for RotateEntity<'_> {
    type Target = Euler<f32>;

    fn deref(&self) -> &Self::Target {
        self.euler
    }
}

impl DerefMut for RotateEntity<'_> {
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
