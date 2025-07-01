use crossbeam::channel::{bounded, Receiver, Sender};
use math::rotation::Euler;
use math::vector::{vec2d, vec3d, vec3i8};

use crate::entity::{ActionState, ActionTarget};

#[derive(Debug)]
pub struct ClientPlayerHandle {
    pub transform: ClientPlayerTransformHandle,
    pub input: ClientPlayerInputHandle,
}

#[derive(Debug)]
pub struct ClientPlayerTransformHandle {
    position: Sender<vec3d>,
    rotation: Sender<Euler<f32>>,
    target: Sender<Option<ActionTarget>>,
}

impl ClientPlayerTransformHandle {
    pub fn set_position(&self, position: vec3d) {
        let _ = self.position.try_send(position);
    }

    pub fn set_rotation(&self, rotation: Euler<f32>) {
        let _ = self.rotation.try_send(rotation);
    }

    pub fn set_target(&self, target: Option<ActionTarget>) {
        let _ = self.target.try_send(target);
    }
}

#[derive(Debug)]
pub struct ClientPlayerInputHandle {
    pub movement: Receiver<vec3i8>,
    pub mouse_movement: Receiver<vec2d>,
    pub action_state: Receiver<ActionState>,
}

impl ClientPlayerInputHandle {
    pub fn next_movement(&self) -> Option<vec3i8> {
        self.movement.try_recv().ok()
    }

    pub fn next_mouse_movement(&self) -> Option<vec2d> {
        self.mouse_movement.try_recv().ok()
    }

    pub fn next_action_state(&self) -> Option<ActionState> {
        self.action_state.try_recv().ok()
    }
}

#[derive(Debug)]
pub struct ServerPlayerHandle {
    pub transform: ServerPlayerTransformHandle,
    pub input: ServerPlayerInputHandle,
}

#[derive(Debug)]
pub struct ServerPlayerTransformHandle {
    position: Receiver<vec3d>,
    rotation: Receiver<Euler<f32>>,
    target: Receiver<Option<ActionTarget>>,
}

impl ServerPlayerTransformHandle {
    pub fn next_position(&self) -> Option<vec3d> {
        self.position.try_recv().ok()
    }

    pub fn next_rotation(&self) -> Option<Euler<f32>> {
        self.rotation.try_recv().ok()
    }

    pub fn next_target(&self) -> Option<Option<ActionTarget>> {
        self.target.try_recv().ok()
    }
}

#[derive(Debug)]
pub struct ServerPlayerInputHandle {
    pub movement: Sender<vec3i8>,
    pub mouse_movement: Sender<vec2d>,
    pub action_state: Sender<ActionState>,
}

impl ServerPlayerInputHandle {
    pub fn set_movement(&self, command: vec3i8) {
        let _ = self.movement.try_send(command);
    }

    pub fn push_mouse_movement(&self, movement: vec2d) {
        let _ = self.mouse_movement.try_send(movement);
    }

    pub fn set_action_state(&self, action_state: ActionState) {
        let _ = self.action_state.try_send(action_state);
    }
}

pub fn create() -> (ClientPlayerHandle, ServerPlayerHandle) {
    let (transform_position_tx, transform_position_rx) = bounded(4);
    let (transform_rotation_tx, transform_rotation_rx) = bounded(4);
    let (transform_target_tx, transform_target_rx) = bounded(4);
    let (input_movement_tx, input_movement_rx) = bounded(4);
    let (input_mouse_movement_tx, input_mouse_movement_rx) = bounded(8);
    let (input_action_state_tx, input_action_state_rx) = bounded(4);

    (
        ClientPlayerHandle {
            transform: ClientPlayerTransformHandle {
                position: transform_position_tx,
                rotation: transform_rotation_tx,
                target: transform_target_tx,
            },
            input: ClientPlayerInputHandle {
                movement: input_movement_rx,
                mouse_movement: input_mouse_movement_rx,
                action_state: input_action_state_rx,
            },
        },
        ServerPlayerHandle {
            transform: ServerPlayerTransformHandle {
                position: transform_position_rx,
                rotation: transform_rotation_rx,
                target: transform_target_rx,
            },
            input: ServerPlayerInputHandle {
                movement: input_movement_tx,
                mouse_movement: input_mouse_movement_tx,
                action_state: input_action_state_tx,
            },
        },
    )
}
