use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::player::{ActionState, PlayerController};
use crossbeam::channel::{bounded, Receiver, Sender};
use math::vector::{vec2d, vec3i8, Vec2};
use std::f32::consts::FRAC_PI_2;

pub fn client_input_channel() -> (ClientInputSender, ClientInputReceiver) {
    let (movement_command_tx, movement_command_rx) = bounded(1);
    let (mouse_movements_tx, mouse_movements_rx) = bounded(4);
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

pub struct ClientInputSender {
    movement_command: Sender<vec3i8>,
    mouse_movements: Sender<vec2d>,
    action_state: Sender<ActionState>,
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

pub struct ClientInputReceiver {
    movement_command: Receiver<vec3i8>,
    mouse_movements: Receiver<vec2d>,
    action_state: Receiver<ActionState>,
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