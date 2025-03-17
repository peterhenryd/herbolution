use crate::world::chunk::map::ChunkMap;
use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::EntityLogic;
use crate::world::entity::{EntityAbilities, EntityData};
use crate::{ActionImpulse, ActionState, Hand};
use kanal::Sender;
use math::angle::Rad;
use math::rotation::Euler;
use math::transform::Transform;
use math::vector::vec3i;
use std::f32::consts::FRAC_PI_2;

#[derive(Debug)]
pub struct PlayerLogic {
    pub controller: PlayerController,
}

impl EntityLogic for PlayerLogic {
    fn tick(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap) {
        self.controller.apply(chunk_map, &mut data.body, &data.abilities);
    }

    fn on_action_impulse(&mut self, data: &mut EntityData, impulse: ActionImpulse) {
        match impulse {
            ActionImpulse::Move { forces } => {
                data.body.forces = forces;
            }
            ActionImpulse::Rotate { delta_rotation } => {
                self.controller.rotation -= delta_rotation;
                self.controller.rotation.pitch = self.controller.rotation.pitch.0.clamp(-FRAC_PI_2, FRAC_PI_2).into();
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
    rotation: Euler<Rad<f32>>,
    is_left_hand_active: bool,
    target_tx: Sender<Option<vec3i>>,
    prev_target: Option<vec3i>,
}

impl PlayerController {
    pub fn new(target_tx: Sender<Option<vec3i>>) -> Self {
        Self {
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
        _: &EntityAbilities,
    ) {
        self.apply_target(body, chunk_map);
        self.apply_rotation(&mut body.transform);
    }

    fn apply_target(&mut self, body: &EntityBody, chunk_map: &mut ChunkMap) {
        let origin = body.get_eye_position();
        let direction = body
            .transform
            .rotation
            .into_view_center();
        let position = chunk_map.cast_ray(origin, direction, 5.0);

        if position != self.prev_target {
            let _ = self.target_tx.try_send(position);
            self.prev_target = position;
        }

        let Some(position) = position else { return; };

        if !self.is_left_hand_active {
            return;
        }

        self.is_left_hand_active = false;

        chunk_map.set_cube(position, None);

    }

    fn apply_rotation(&mut self, transform: &mut Transform) {
        transform.rotation = self.rotation;
    }
}