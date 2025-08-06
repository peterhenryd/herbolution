use std::ops::Deref;
use std::sync::Arc;

use kiln::Env3d;
use prims::camera::{Camera, Perspective, View};
use prims::vector::{Vec3, Vec4, vec3f};
use server::entity::ActionState;
use server::player::{PlayerInputDelta, PlayerInputState, PlayerState, ServerPlayerHandle};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use crate::app::Update;
use crate::world::frustum::Frustum;

#[derive(Debug)]
pub struct Player {
    pub(crate) state: PlayerState,
    pub(crate) frustum: Frustum,
    perspective: Perspective,
    pub(crate) handle: Option<ServerPlayerHandle>,
}

impl Player {
    pub fn new(aspect: f32) -> Self {
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.1, 500.0);
        let camera = Camera::new(Vec3::ZERO, View::rotatable(), perspective);

        Self {
            state: PlayerState::default(),
            frustum: Frustum::new(camera.view_proj),
            perspective,
            handle: None,
        }
    }

    pub fn update(&mut self, ctx: &mut Update) {
        let Some(handle) = &self.handle else { return };

        let guard = handle.state.load();
        let Some(state) = guard.as_ref() else {
            return;
        };

        self.state = state.deref().clone();

        let camera = self.camera();
        ctx.app.core3d.write_env(
            &ctx.app.gpu,
            &Env3d {
                view_proj: camera.view_proj,
                camera_pos: self.eye_position().extend(0.0),
            },
        );
        self.frustum = Frustum::new(camera.view_proj);

        /*
        ctx.video
            .sculptor
            .sets()
            .write_from(self.sky_box_id, cube(state.position.cast(), self.sky_box_color))
            .expect("Failed to update sky box");

        let sets = ctx.video.sculptor.sets();
        match self.state.target {
            Some(ActionTarget::Cube(target)) => {
                self.set_targeted_cube(sets, target)
                    .expect("Failed to set targeted cube");
            }
            Some(ActionTarget::Entity(_)) => {}
            None => {
                sets.get_mut(self.targeted_cube_wireframe_id)
                    .truncate(0);
                sets.get_mut(self.targeted_cube_shell_id)
                    .truncate(0);
            }
        }

        let video_camera = self.video_camera();

        ctx.video.sculptor.update_camera(&video_camera);
        self.frustum = Frustum::new(video_camera.view_proj);

         */
    }
    /*

    fn set_targeted_cube(&self, sets: &mut Sets<Instance3d>, target: CubeTarget) -> Result<(), ()> {
        let position = target.position.cast();
        if self.state.shell_opacity == 0.0 {
            sets.write(self.targeted_cube_shell_id, &[])?;
            sets.write_from(self.targeted_cube_wireframe_id, cube(position, Rgba::BLACK))?;
        } else {
            sets.write_from(self.targeted_cube_shell_id, cube(position, Rgba::new(1.0, 1.0, 1.0, self.state.shell_opacity)))?;
            sets.write(self.targeted_cube_wireframe_id, &[])?;
        }
        Ok(())
    }

     */

    pub fn update_input(&mut self, ctx: &Update) {
        let Some(handle) = &self.handle else {
            return;
        };

        let mut action_state = ActionState::default();

        let is_lmb_active = ctx
            .app
            .input
            .is_mouse_button_active(MouseButton::Left);
        action_state.is_left_hand_active = is_lmb_active;
        action_state.is_right_hand_active = ctx
            .app
            .input
            .is_mouse_button_active(MouseButton::Right)
            || (is_lmb_active && ctx.app.input.is_left_control_active());

        let mut forces = Vec3::ZERO;
        if ctx.app.input.is_key_active(KeyCode::KeyW) {
            forces.x += 1.0;
        }
        if ctx.app.input.is_key_active(KeyCode::KeyS) {
            forces.x -= 1.0;
        }
        if ctx.app.input.is_key_active(KeyCode::KeyA) {
            forces.z += 1.0;
        }
        if ctx.app.input.is_key_active(KeyCode::KeyD) {
            forces.z -= 1.0;
        }
        if ctx.app.input.is_key_active(KeyCode::Space) {
            forces.y += 1.0;
        }
        if ctx.app.input.is_key_active(KeyCode::ShiftLeft) {
            forces.y -= 1.0;
        }

        handle
            .input_state
            .store(Some(Arc::new(PlayerInputState {
                relative_motion: forces,
                action_state,
                toggle_gravity: ctx.input.key_events.contains(&KeyCode::KeyG),
            })));

        let movement = ctx.input.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            let _ = handle
                .input_delta
                .try_send(PlayerInputDelta::MouseMovement(movement));
        }

        if ctx.input.mouse_scroll != 0.0 {
            let _ = handle
                .input_delta
                .try_send(PlayerInputDelta::MouseScroll(ctx.input.mouse_scroll));
        }
    }

    fn eye_position(&self) -> vec3f {
        self.state.pos.cast::<f32>() + self.state.eye_offset
    }

    pub fn camera(&self) -> Camera {
        Camera::new(self.eye_position(), View::Rotate { rotation: self.state.rotation }, self.perspective)
    }
}
