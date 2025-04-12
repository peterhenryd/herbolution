use std::f32::consts::FRAC_PI_2;
use crossbeam::channel::{bounded, Receiver, Sender};
use math::angle::Rad;
use math::rotation::Euler;
use math::vector::{vec2d, vec3f, vec3i8, Vec2};
use crate::entity::body::EntityBody;
use crate::entity::EntityTarget;
use crate::entity::logic::player::{ActionState, PlayerController};
use crate::entity::set::EntityId;

#[derive(Debug)]
pub struct Client {
    pub entity_id: EntityId,
    pub input_receiver: ClientInputReceiver,
    pub output_sender: ClientOutputSender,
}

#[derive(Debug)]
pub struct ClientInputSender {
    movement_command: Sender<vec3i8>,
    mouse_movements: Sender<vec2d>,
    action_state: Sender<ActionState>,
}

#[derive(Debug)]
pub struct ClientInputReceiver {
    movement_command: Receiver<vec3i8>,
    mouse_movements: Receiver<vec2d>,
    action_state: Receiver<ActionState>,
}

#[derive(Debug, Clone)]
pub struct ClientOutputSender {
    camera_pos: Sender<vec3f>,
    camera_rotation: Sender<Euler<Rad<f32>>>,
    entity_target: Sender<Option<EntityTarget>>
}

#[derive(Debug)]
pub struct ClientOutputReceiver {
    camera_pos: Receiver<vec3f>,
    camera_rotation: Receiver<Euler<Rad<f32>>>,
    entity_target: Receiver<Option<EntityTarget>>
}

impl ClientInputSender {
    pub fn set_movement_command(&self, command: vec3i8) {
        let _ = self.movement_command.try_send(command);
    }

    pub fn add_mouse_movement(&mut self, movement: vec2d) {
        let _ = self.mouse_movements.try_send(movement);
    }

    pub fn set_action_state(&self, action_state: ActionState) {
        let _ = self.action_state.try_send(action_state);
    }
}

impl ClientInputReceiver {
    pub fn dequeue_onto_body(&mut self, body: &mut EntityBody) {
        if let Ok(command) = self.movement_command.try_recv() {
            body.apply_motion_command(command);
        }

        while let Ok(Vec2 { x: dx, y: dy }) = self.mouse_movements.try_recv() {
            // TODO: Implement mouse sensitivity
            body.rotation.yaw -= dx.to_radians() as f32;
            body.rotation.pitch -= dy.to_radians() as f32;
        }
        body.rotation.pitch = body.rotation.pitch.0.clamp(-FRAC_PI_2 + f32::EPSILON, FRAC_PI_2 - f32::EPSILON).into();
    }

    pub fn dequeue_onto_controller(&mut self, controller: &mut PlayerController) {
        if let Ok(action_state) = self.action_state.try_recv() {
            controller.set_action_state(action_state);
        }
    }
}

impl ClientOutputSender {
    pub fn send_camera_pos(&self, pos: vec3f) {
        let _ = self.camera_pos.try_send(pos);
    }

    pub fn send_camera_rotation(&self, rotation: Euler<Rad<f32>>) {
        let _ = self.camera_rotation.try_send(rotation);
    }

    pub fn send_target(&self, target: Option<EntityTarget>) {
        let _ = self.entity_target.try_send(target);
    }
}

impl ClientOutputReceiver {
    pub fn recv_camera_pos(&self) -> Option<vec3f> {
        self.camera_pos.try_recv().ok()
    }

    pub fn recv_camera_rotation(&self) -> Option<Euler<Rad<f32>>> {
        self.camera_rotation.try_recv().ok()
    }

    pub fn recv_target(&self) -> Option<Option<EntityTarget>> {
        self.entity_target.try_recv().ok()
    }
}

pub fn client_input_channel() -> (ClientInputSender, ClientInputReceiver) {
    let (movement_command_tx, movement_command_rx) = bounded(1);
    let (mouse_movements_tx, mouse_movements_rx) = bounded(8);
    let (action_state_tx, action_state_rx) = bounded(1);

    (
        ClientInputSender {
            movement_command: movement_command_tx,
            mouse_movements: mouse_movements_tx,
            action_state: action_state_tx,
        },
        ClientInputReceiver {
            movement_command: movement_command_rx,
            mouse_movements: mouse_movements_rx,
            action_state: action_state_rx,
        }
    )
}

pub fn client_output_channel() -> (ClientOutputSender, ClientOutputReceiver) {
    let (camera_pos_tx, camera_pos_rx) = bounded(1);
    let (camera_rotation_tx, camera_rotation_rx) = bounded(1);
    let (entity_target_tx, entity_target_rx) = bounded(1);

    (
        ClientOutputSender {
            camera_pos: camera_pos_tx,
            camera_rotation: camera_rotation_tx,
            entity_target: entity_target_tx,
        },
        ClientOutputReceiver {
            camera_pos: camera_pos_rx,
            camera_rotation: camera_rotation_rx,
            entity_target: entity_target_rx,
        }
    )
}