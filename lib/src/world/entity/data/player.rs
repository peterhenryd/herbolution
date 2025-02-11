use crate::engine::geometry::cuboid::Cuboid;
use crate::listener::InputEvent;
use crate::world::chunk::map::ChunkMap;
use crate::world::chunk::material::Material;
use crate::world::entity::physics::EntityPhysics;
use crate::world::entity::EntityAbilities;
use crate::world::transform::{Rotation, Transform};
use math::angle::Rad;
use math::vector::vec3;
use std::f64::consts::FRAC_PI_2;
use std::ops::SubAssign;
use std::time::Duration;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub struct PlayerEntityData {
    pub controller: PlayerController,
}

impl PlayerEntityData {
    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap, physics: &mut EntityPhysics, abilities: EntityAbilities) {
        self.controller.update(dt, chunk_map, physics, abilities);
    }
}

impl Default for PlayerEntityData {
    fn default() -> Self {
        Self {
            controller: PlayerController::default(),
        }
    }
}

#[derive(Debug)]
pub struct PlayerController {
    pub motion: PlayerMotion,
    pub actions: PlayerActions,
}

impl PlayerController {
    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap, physics: &mut EntityPhysics, abilities: EntityAbilities) {
        self.motion.apply(dt, chunk_map, physics, abilities);
        self.actions.update(chunk_map, physics);
    }

    pub fn on_input(&mut self, event: &InputEvent) {
        use ElementState::Pressed;
        use InputEvent::{Key, MouseClick, MouseMoving};
        use MouseButton::Left;

        match event {
            Key { code, state } => self.motion.on_input(*code, *state),
            &MouseMoving { dx, dy } => self.motion.delta_rotation
                .get_or_insert(Rotation::default())
                .sub_assign(Rotation::new(dx.to_radians(), dy.to_radians()) * 4.0),
            MouseClick { button: Left, state: Pressed, } => self.actions.is_left_hand_active = true,
            _ => {}
        }
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            motion: PlayerMotion::default(),
            actions: PlayerActions::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerMotion {
    forces: Cuboid<f32>,
    pub delta_rotation: Option<Rotation>,
    jump_update_count: u32,
}

impl PlayerMotion {
    pub fn on_input(&mut self, code: KeyCode, state: ElementState) {
        let f = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match code {
            KeyCode::KeyS => self.forces.min.x = f,
            KeyCode::KeyW => self.forces.max.x = f,
            KeyCode::ShiftLeft => self.forces.min.y = f,
            KeyCode::Space => self.forces.max.y = f,
            KeyCode::KeyD => self.forces.min.z = f,
            KeyCode::KeyA => self.forces.max.z = f,
            _ => {}
        }
    }

    pub fn apply(&mut self, dt: Duration, chunk_map: &mut ChunkMap, physics: &mut EntityPhysics, abilities: EntityAbilities) {
        self.apply_translation(dt, chunk_map, physics, abilities);
        self.apply_rotation(dt, &mut physics.transform);
    }

    fn apply_translation(&mut self, dt: Duration, chunk_map: &mut ChunkMap, physics: &mut EntityPhysics, abilities: EntityAbilities) {
        let forces = self.forces.max - self.forces.min;
        let (straight, side) = physics.transform.rotation.into_directions();
        let (straight, side) = (straight.cast::<f32>(), side.cast::<f32>());
        let up = vec3::y();

        let mut velocity = vec3::ZERO;
        velocity += straight * forces.x;
        velocity += side * forces.z;

        if abilities.is_affected_by_gravity && self.forces.max.y > 0.0 {
            self.jump_update_count = 40;
            self.forces.max.y = 0.0;
        }

        if abilities.is_affected_by_gravity && self.jump_update_count > 0 {
            velocity += up * -(20 - self.jump_update_count as i32) as f32 * 8.0;
            self.jump_update_count -= 1;
        } else {
            velocity += up * forces.y;
        }

        if velocity != vec3::ZERO {
            velocity = velocity.normalize() * dt.as_secs_f32() * 18.0;
        }

        let bounding_box = physics.get_bounding_box() + velocity;
        for collider in chunk_map.get_near_colliders(bounding_box) {
            if bounding_box.intersects(&collider) {
                return;
            }
        }

        physics.transform.position += velocity;
    }

    fn apply_rotation(&mut self, dt: Duration, transform: &mut Transform) {
        let rotation = self.delta_rotation.take().unwrap_or_default();
        let rotation = Rotation {
            yaw: rotation.yaw * dt.as_secs_f64(),
            pitch: rotation.pitch * dt.as_secs_f64(),
        };

        transform.rotation += rotation;
        transform.rotation.pitch = Rad(transform.rotation.pitch.0.clamp(-FRAC_PI_2, FRAC_PI_2));
    }
}

impl Default for PlayerMotion {
    fn default() -> Self {
        Self {
            forces: Cuboid::ZERO,
            delta_rotation: None,
            jump_update_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct PlayerActions {
    pub is_left_hand_active: bool,
}

impl PlayerActions {
    pub fn update(&mut self, chunk_map: &mut ChunkMap, physics: &EntityPhysics) {
        if !self.is_left_hand_active {
            return;
        }

        self.is_left_hand_active = false;

        let eye_position = physics.get_eye_position();
        let origin = eye_position + eye_position.signum() * vec3::splat(0.5);
        let direction = physics.transform.rotation.into_center();
        let Some(pos) = chunk_map.cast_ray(origin, direction) else { return };

        chunk_map.set_cube(pos, Material::Air);
    }
}

impl Default for PlayerActions {
    fn default() -> Self {
        Self {
            is_left_hand_active: false,
        }
    }
}