use kanal::{bounded, Sender};
use num::traits::ConstZero;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use engine::gpu::handle::Handle;
use engine::input::{Input, InputFrame};
use engine::renderer_3d::Renderer3D;
use game::{ActionImpulse, ActionState, GameHandle, Hand, PlayerHandle};
use math::angle::Rad;
use math::rotation::Euler;
use math::transform::Transform;
use math::vector::{vec3f, vec3i, Vec3};

pub struct SessionPlayer {
    action_sender: Sender<ActionImpulse>,
    pub(super) handle: Option<PlayerHandle>,
    prev_forces: vec3f,
    prev_delta_rotation: Euler<Rad<f32>>,
    prev_target: Option<vec3i>,
}

impl SessionPlayer {
    pub fn new(handle: &GameHandle) -> Self {
        let (action_sender, action_receiver) = bounded(256);
        handle.request_player(action_receiver);

        Self {
            action_sender,
            handle: None,
            prev_forces: Vec3::ZERO,
            prev_delta_rotation: Euler::IDENTITY,
            prev_target: None,
        }
    }

    fn send_action(&self, action_impulse: ActionImpulse) {
        if let Err(e) = self.action_sender.try_send(action_impulse) {
            eprintln!("Failed to send player action impulse: {}", e);
        }
    }

    pub(super) fn update(&mut self, handle: &Handle, render: &mut Renderer3D, frame_input: &InputFrame, input: &Input, is_focused: bool) {
        if !is_focused {
            if self.prev_forces != Vec3::ZERO {
                self.prev_forces = Vec3::ZERO;
                self.send_action(ActionImpulse::Move { forces: Vec3::ZERO });
            }
            return;
        }

        let Some(player_handle) = &mut self.handle else { return };
        player_handle.update();

        if player_handle.target != self.prev_target {
            render.set_highlighted_tile(&handle, player_handle.target);
            self.prev_target = player_handle.target;
        }

        render.camera.transform.position = player_handle.transform.position + Vec3::new(0., 0.9, 0.);
        render.camera.transform.rotation = player_handle.transform.rotation;

        for click_event in &frame_input.click_events {
            if click_event.button != MouseButton::Left {
                continue;
            }

            self.send_action(ActionImpulse::Interact { hand: Hand::Left, state: ActionState::Once });
        }

        let mut forces = Vec3::ZERO;
        if input.is_key_active(KeyCode::KeyW) { forces.x += 1.0; }
        if input.is_key_active(KeyCode::KeyS) { forces.x -= 1.0; }
        if input.is_key_active(KeyCode::KeyA) { forces.z += 1.0; }
        if input.is_key_active(KeyCode::KeyD) { forces.z -= 1.0; }
        if input.is_key_active(KeyCode::Space) { forces.y += 1.0; }
        if input.is_key_active(KeyCode::ShiftLeft) { forces.y -= 1.0; }

        if forces != self.prev_forces {
            self.send_action(ActionImpulse::Move { forces });
            self.prev_forces = forces;
        }

        let mouse_movement = frame_input.mouse_movement;
        let delta_rotation = Euler::new(
            Rad(mouse_movement.x.to_radians() as f32),
            Rad(mouse_movement.y.to_radians() as f32),
            Rad(0.0)
        );

        if self.prev_delta_rotation != delta_rotation {
            self.send_action(ActionImpulse::Rotate { delta_rotation });
            self.prev_delta_rotation = delta_rotation;
        }
    }

    pub fn transform(&self) -> Transform {
        self.handle.as_ref().map(|x| x.transform.clone()).unwrap_or_default()
    }
}