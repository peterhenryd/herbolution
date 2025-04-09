use crate::world::chunk::map::ChunkMap;
use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::EntityLogic;
use crate::world::entity::{EntityAbilities, EntityData, EntityTarget};
use crate::client::output::ClientOutputSender;
use crate::world::chunk::material::Material;

#[derive(Debug)]
pub struct PlayerLogic {
    pub controller: PlayerController,
}

impl PlayerLogic {
    pub fn new(output_sender: ClientOutputSender) -> Self {
        Self {
            controller: PlayerController::new(output_sender)
        }
    }
}

impl EntityLogic for PlayerLogic {
    fn tick(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap) {
        self.controller.tick(chunk_map, &mut data.body, &data.abilities);
    }
}

#[derive(Debug)]
pub struct PlayerController {
    action_state: ActionState,
    output_sender: ClientOutputSender,
    prev_target: Option<EntityTarget>,
}

impl PlayerController {
    pub fn new(output_sender: ClientOutputSender) -> Self {
        Self {
            action_state: ActionState::default(),
            output_sender,
            prev_target: None,
        }
    }

    pub fn set_action_state(&mut self, action_state: ActionState) {
        if self.prev_target.is_some() {
            self.action_state = action_state;
        }
    }

    pub fn tick(
        &mut self,
        chunk_map: &mut ChunkMap,
        body: &mut EntityBody,
        _: &EntityAbilities,
    ) {
        self.apply_target(body, chunk_map);
    }

    fn apply_target(&mut self, body: &EntityBody, chunk_map: &mut ChunkMap) {
        let origin = body.eye_pos();
        let direction = body.rotation.into_view_center();
        let ray = chunk_map.cast_ray(origin, direction, 5.0);
        let pos = ray.map(|(x, _)| x);
        let target = pos.map(EntityTarget::Cube);

        if target != self.prev_target {
            self.output_sender.send_target(target);
            self.prev_target = target;
        }

        let Some(pos) = pos else { return; };

        if self.action_state.is_left_hand_active {
            self.action_state.is_left_hand_active = false;

            chunk_map.set_cube(pos, None);
        }

        if self.action_state.is_right_hand_active {
            self.action_state.is_right_hand_active = false;

            let Some((_, face)) = ray else { return };
            chunk_map.set_cube(pos + face.into_vec3(), Some(Material::Stone));
        }
    }
}

#[derive(Debug, Default)]
pub struct ActionState {
    pub is_left_hand_active: bool,
    pub is_right_hand_active: bool,
}