use crate::gpu::Gpu;
use crate::input::{Input, InputFrame};
use crate::state3d::State3d;
use game::channel::ClientChannel;
use game::client::{client_input_channel, ClientInputSender, ClientOutputReceiver};
use game::entity::logic::player::ActionState;
use game::entity::EntityTarget;
use math::vector::{vec3i, vec3i8};
use std::time::Duration;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

pub struct SessionPlayer {
    input_sender: ClientInputSender,
    pub(super) output_receiver: Option<ClientOutputReceiver>,
    prev_target: Option<vec3i>,
    //prev_position: vec3f,
    //position: vec3f,
    //view_bob: f32,
}

impl SessionPlayer {
    pub fn create(channel: &ClientChannel) -> Self {
        let (input_sender, input_receiver) = client_input_channel();
        channel.send_client_input(input_receiver);

        Self {
            input_sender,
            output_receiver: None,
            prev_target: None,
            //prev_position: Vec3::ZERO,
            //position: Vec3::ZERO,
            //view_bob: 0.0,
        }
    }


    pub(super) fn update(&mut self, _: Duration, gpu: &Gpu, state: &mut State3d, input: (&InputFrame, &Input), is_focused: bool) {
        let Some(output_receiver) = &mut self.output_receiver else { return };

        if let Some(position) = output_receiver.recv_camera_pos() {
            //self.prev_position = self.position;
            //self.position = position;
            state.camera.position = position;
        }

        /*if (self.prev_position - self.position).length_squared() > 0.5 * dt.as_secs_f32() {
            self.view_bob += dt.as_secs_f32() * 10.0;
        } else {
            self.view_bob = 0.0;
        }

        camera_position.y += self.view_bob.sin() * 0.1;

         */


        if let Some(rot) = output_receiver.recv_camera_rotation() {
            state.camera.rot = rot;
        }

        match output_receiver.recv_target() {
            Some(Some(EntityTarget::Cube(pos))) => {
                if self.prev_target != Some(pos) {
                    state.set_highlighted_tile(gpu, Some(pos));
                    self.prev_target = Some(pos);
                }
            }
            Some(Some(EntityTarget::Entity(_))) => {}
            Some(None) => {
                if self.prev_target.is_some() {
                    state.set_highlighted_tile(gpu, None);
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
                MouseButton::Left if input.is_left_control_active() => action_state.is_right_hand_active = true,
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