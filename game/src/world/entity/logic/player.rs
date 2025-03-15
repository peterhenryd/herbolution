use crate::world::chunk::map::ChunkMap;
use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::EntityLogic;
use crate::world::entity::{EntityAbilities, EntityData};
use crate::{ActionImpulse, ActionState, Hand};
use math::angle::Rad;
use math::num::traits::ConstZero;
use math::rotation::Euler;
use math::transform::Transform;
use math::vector::{vec3f, vec3i, Vec3};
use std::f32::consts::FRAC_PI_2;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct PlayerLogic {
    pub controller: PlayerController,
}

impl EntityLogic for PlayerLogic {
    fn tick(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap) {
        self.controller.apply(chunk_map, &mut data.body, &data.abilities);
    }

    fn on_action_impulse(&mut self, impulse: ActionImpulse) {
        match impulse {
            ActionImpulse::Move { forces } => {
                self.controller.forces = forces;
            }
            ActionImpulse::Rotate { delta_rotation } => {
                self.controller.rotation -= delta_rotation;
            }
            ActionImpulse::Interact { hand: Hand::Left, state: ActionState::Once } => {
                self.controller.is_left_hand_active = true;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct PlayerController {
    forces: vec3f,
    rotation: Euler<Rad<f32>>,
    is_left_hand_active: bool,
    target_tx: Sender<Option<vec3i>>,
    prev_target: Option<vec3i>,
}

impl PlayerController {
    pub fn new(target_tx: Sender<Option<vec3i>>) -> Self {
        Self {
            forces: vec3f::ZERO,
            rotation: Euler::IDENTITY,
            is_left_hand_active: false,
            target_tx,
            prev_target: None,
        }
    }

    pub fn apply(
        &mut self,
        chunk_map: &mut ChunkMap,
        body: &mut EntityBody,
        abilities: &EntityAbilities,
    ) {
        self.apply_translation(chunk_map, body, abilities);
        self.apply_rotation(&mut body.transform);

        let origin = body.get_eye_position();
        let direction = body
            .transform
            .rotation
            .into_view_center()
            .cast()
            .unwrap();
        let pos = chunk_map.cast_ray(origin, direction);

        if pos != self.prev_target {
            let _ = self.target_tx.try_send(pos);
            self.prev_target = pos;
        }

        let Some(pos) = pos else { return; };

        if !self.is_left_hand_active {
            return;
        }

        self.is_left_hand_active = false;

        chunk_map.set_cube(pos, None);
    }

    fn apply_translation(
        &mut self,
        _: &mut ChunkMap,
        body: &mut EntityBody,
        _: &EntityAbilities,
    ) {
        let (straight, side) = body.transform.rotation.into_view_directions();
        let (straight, side) = (straight.cast::<f32>().unwrap(), side.cast::<f32>().unwrap());
        let up = Vec3::Y;

        let mut velocity = Vec3::ZERO;
        velocity += straight * self.forces.x;
        velocity += side * self.forces.z;
        velocity += up * self.forces.y;

        if velocity != Vec3::ZERO {
            velocity = velocity.normalize() * 0.5;
        }

        body.transform.position += velocity;
    }

    fn apply_rotation(&mut self, transform: &mut Transform) {
        transform.rotation = self.rotation;
        transform.rotation.pitch = Rad(transform.rotation.pitch.0.clamp(-FRAC_PI_2, FRAC_PI_2));
    }
}