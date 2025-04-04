use engine::gpu::handle::Handle;
use engine::input::{Input, InputFrame};
use engine::renderer_3d::Renderer3D;
use game::client::input::{client_input_channel, ClientInputSender};
use game::handle::GameHandle;
use game::world::entity::logic::player::ActionState;
use math::vector::{vec3i, vec3i8};
use num::traits::ConstZero;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use game::client::output::ClientOutputReceiver;
use game::world::entity::EntityTarget;

pub struct SessionPlayer {
    input_sender: ClientInputSender,
    pub(super) output_receiver: Option<ClientOutputReceiver>,
    prev_target: Option<vec3i>,
}

impl SessionPlayer {
    pub fn new(handle: &GameHandle) -> Self {
        let (input_sender, input_receiver) = client_input_channel();
        handle.request_player(input_receiver);

        Self {
            input_sender,
            output_receiver: None,
            prev_target: None,
        }
    }


    pub(super) fn update(&mut self, handle: &Handle, renderer: &mut Renderer3D, input: (&InputFrame, &Input), is_focused: bool) {
        let Some(output_receiver) = &mut self.output_receiver else { return };

        if let Some(position) = output_receiver.receive_camera_position() {
            renderer.camera.position = position;
        }

        if let Some(rotation) = output_receiver.receive_camera_rotation() {
            renderer.camera.rotation = rotation;
        }

        match output_receiver.receive_target() {
            Some(Some(EntityTarget::Cube(position))) => {
                if self.prev_target != Some(position) {
                    renderer.set_highlighted_tile(&handle, Some(position));
                    self.prev_target = Some(position);
                }
            }
            Some(Some(EntityTarget::Entity(_))) => {}
            Some(None) => {
                if self.prev_target.is_some() {
                    renderer.set_highlighted_tile(&handle, None);
                    self.prev_target = None;
                }
            }
            None => {}
        }

        if is_focused {
            self.dequeue_input(input);
        }
    }

    fn dequeue_input(&mut self, (frame, input): (&InputFrame, &Input)) {
        // Mouse buttons
        let mut action_state = ActionState::default();
        for click_event in &frame.click_events {
            match click_event.button {
                MouseButton::Left => action_state.is_left_hand_active = true,
                MouseButton::Right => action_state.is_right_hand_active = true,
                _ => {}
            }
        }

        if action_state.is_left_hand_active || action_state.is_right_hand_active {
            self.input_sender.set_action_state(action_state);
        }

        // Keyboard
        let mut forces = vec3i8::ZERO;
        if input.is_key_active(KeyCode::KeyW) { forces.x += 1; }
        if input.is_key_active(KeyCode::KeyS) { forces.x -= 1; }
        if input.is_key_active(KeyCode::KeyA) { forces.z += 1; }
        if input.is_key_active(KeyCode::KeyD) { forces.z -= 1; }
        if input.is_key_active(KeyCode::Space) { forces.y += 1; }
        if input.is_key_active(KeyCode::ShiftLeft) { forces.y -= 1; }

        self.input_sender.set_movement_command(forces);

        // Mouse movement
        let movement = frame.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            self.input_sender.add_mouse_movement(movement);
        }
    }
}