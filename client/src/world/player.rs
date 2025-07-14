use crate::app::Update;
use crate::video::camera::{VideoCamera, View};
use crate::video::resource::{SetId, Sets};
use crate::video::world::Instance3d;
use crate::video::Video;
use crate::world::frustum::Frustum;
use lib::color::{ColorConsts, Rgba};
use lib::proj::Perspective;
use lib::spatial::CubeFace;
use lib::vector::{vec3f, Vec3, Vec4};
use server::entity::{ActionState, ActionTarget, CubeTarget};
use server::player::{PlayerInput, PlayerState, ServerPlayerHandle};
use std::ops::Deref;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub struct Player {
    pub(crate) state: PlayerState,
    pub(crate) frustum: Frustum,
    perspective: Perspective,
    pub(crate) handle: Option<ServerPlayerHandle>,
    pub(crate) targeted_cube_wireframe_id: SetId,
    pub(crate) targeted_cube_shell_id: SetId,
    sky_box_color: Rgba<f32>,
    pub(crate) sky_box_id: SetId,
}

impl Player {
    pub fn create(sky_box_color: Rgba<f32>, video: &mut Video) -> Self {
        let aspect = video.resolution().cast::<f32>().aspect();
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.1, 500.0);

        let video_camera = VideoCamera::new(Vec3::ZERO, View::rotatable(), perspective);
        video.sculptor.update_camera(&video_camera);

        Self {
            state: PlayerState::default(),
            frustum: Frustum::new(video_camera.view_proj),
            perspective,
            handle: None,
            targeted_cube_wireframe_id: video.sculptor.sets().insert_with_capacity(6),
            targeted_cube_shell_id: video.sculptor.sets().insert_with_capacity(6),
            sky_box_color,
            sky_box_id: video
                .sculptor
                .sets()
                .insert_from(cube(Vec3::ZERO, sky_box_color)),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) {
        let Some(handle) = &self.handle else { return };

        let guard = handle.state.load();
        let Some(state) = guard.as_ref() else {
            return;
        };

        self.state = state.deref().clone();

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
    }

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

    pub fn update_input(&mut self, ctx: &Update) {
        let Some(handle) = &self.handle else {
            return;
        };

        let mut action_state = ActionState::default();

        let is_lmb_active = ctx
            .store
            .input
            .is_mouse_button_active(MouseButton::Left);
        action_state.is_left_hand_active = is_lmb_active;
        action_state.is_right_hand_active = ctx
            .store
            .input
            .is_mouse_button_active(MouseButton::Right)
            || (is_lmb_active && ctx.store.input.is_left_control_active());

        let _ = handle
            .input_tx
            .try_send(PlayerInput::ActionState(action_state));

        let mut forces = Vec3::ZERO;
        if ctx.store.input.is_key_active(KeyCode::KeyW) {
            forces.x += 1.0;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyS) {
            forces.x -= 1.0;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyA) {
            forces.z += 1.0;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyD) {
            forces.z -= 1.0;
        }
        if ctx.store.input.is_key_active(KeyCode::Space) {
            forces.y += 1.0;
        }
        if ctx.store.input.is_key_active(KeyCode::ShiftLeft) {
            forces.y -= 1.0;
        }

        let _ = handle
            .input_tx
            .try_send(PlayerInput::MotionUnit(forces));

        let movement = ctx.input.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            let _ = handle
                .input_tx
                .try_send(PlayerInput::MouseMovement(movement));
        }

        if ctx.input.mouse_scroll != 0.0 {
            let _ = handle
                .input_tx
                .try_send(PlayerInput::SpeedDelta(ctx.input.mouse_scroll * 2.0));
        }
    }

    fn eye_position(&self) -> vec3f {
        self.state.position.cast::<f32>() + self.state.eye_offset
    }

    fn video_camera(&self) -> VideoCamera {
        VideoCamera::new(self.eye_position(), View::Rotate { rotation: self.state.rotation }, self.perspective)
    }
}

fn cube(position: vec3f, color: Rgba<f32>) -> impl IntoIterator<Item = Instance3d> {
    CubeFace::values()
        .map(CubeFace::rotation)
        .map(move |rotation| Instance3d::new(position, rotation, Vec3::splat(1.0), color, 1, Vec4::ZERO))
}
