mod camera;

use crate::app::Update;
pub use camera::PlayerCamera;
use engine::sculptor::Instance3d;
use engine::{sculptor, Engine};
use game::entity::{ActionState, ActionTarget};
use game::player::handle::ServerPlayerHandle;
use gpu::SetId;
use herbolution_math::spatial::face::Face;
use math::color::{ColorConsts, Rgba};
use math::vector::{vec3d, vec3i, vec3i8, Vec3};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

/// The render-side representation of the player within the world.
#[derive(Debug)]
pub struct Player {
    /// The channel used to receive updates from the behavior-side player.
    pub(crate) handle: Option<ServerPlayerHandle>,
    /// The previous entity or cube the player was looking at.
    prev_target: Option<vec3i>,
    /// The camera, positioned at the player's eye.
    pub(crate) camera: PlayerCamera,
    //prev_position: vec3f,
    //position: vec3f,
    //view_bob: f32,
    /// The identifier for the GPU buffer containing the quads to be rendered as a wireframe when the player is targeting a cube.
    pub(crate) targeted_cube_wireframe_id: SetId,
    /// The color of the sky box.
    sky_box_color: Rgba<f32>,
    /// The identifier for the GPU buffer containing the quads of the sky box, which is rendered as a cube around the player.
    pub(crate) sky_box_id: SetId,
}

impl Player {
    /// Creates a new instance, while also sending a signal to the behavior-side herbolution_game to create the player entity.
    pub fn create(sky_box_color: Rgba<f32>, engine: &mut Engine) -> Self {
        Self {
            handle: None,
            prev_target: None,
            //prev_position: Vec3::ZERO,
            //position: Vec3::ZERO,
            //view_bob: 0.0,
            camera: PlayerCamera::new(engine.video.resolution(), &mut engine.video.sculptor),
            targeted_cube_wireframe_id: engine
                .video
                .sculptor
                .sets()
                .insert_with_capacity(6),
            sky_box_color,
            sky_box_id: engine
                .video
                .sculptor
                .sets()
                .insert_from(cube(Vec3::ZERO, sky_box_color)),
        }
    }

    /// Updates the player state.
    pub fn update(&mut self, ctx: &mut Update) {
        // If the behavior-side herbolution_game has not processed the player creation yet, skip the update.
        let Some(handle) = &mut self.handle else { return };

        // Receive the latest camera position and rotation from the behavior-side player.
        let mut update_camera = false;
        if let Some(x) = handle.transform.next_position() {
            //self.prev_position = self.position;
            //self.position = position;

            // TODO: receive the player position instead of the camera position, and setup a different channel for updating the relative eye position.

            self.camera.video.position = x;
            update_camera = true;

            ctx.engine
                .video
                .sculptor
                .sets()
                .write_from(self.sky_box_id, cube(x.cast(), self.sky_box_color))
                .expect("Failed to update sky box");
        }

        /*
        if (self.prev_position - self.position).length_squared() > 0.5 * dt.as_secs_f32() {
            self.view_bob += dt.as_secs_f32() * 10.0;
        } else {
            self.view_bob = 0.0;
        }

        camera_position.y += self.view_bob.sin() * 0.1;
         */

        if let Some(x) = handle.transform.next_rotation() {
            *self.camera.video.view.rotation().unwrap() = x;
            update_camera = true;
        }

        if update_camera {
            self.camera.update(ctx.engine);
        }

        let sets = ctx.engine.video.sculptor.sets();
        // Overwrite the targeted cube wireframe with the latest target from the behavior-side player.
        match handle.transform.next_target() {
            Some(Some(ActionTarget::Cube(position))) => {
                if self.prev_target != Some(position) {
                    self.set_targeted_cube(sets, Some(position));
                    self.prev_target = Some(position);
                }
            }
            Some(Some(ActionTarget::Entity(_))) => {}
            Some(None) => {
                sets.get_mut(self.targeted_cube_wireframe_id)
                    .shorten_to(0);
                self.prev_target = None;
            }
            _ => {}
        }
    }

    fn set_targeted_cube(&mut self, sets: &mut sculptor::Sets, position: Option<vec3i>) {
        match position {
            None => sets.write(self.targeted_cube_wireframe_id, &[]),
            Some(x) => sets.write_from(self.targeted_cube_wireframe_id, cube(x.cast(), Rgba::BLACK)),
        }
        .expect("Failed to clear targeted cube wireframe instances");
    }

    pub fn update_input(&mut self, ctx: &Update) {
        let Some(handle) = &self.handle else {
            return;
        };

        // Process mouse button input
        let mut action_state = ActionState::default();

        let is_lmb_active = ctx
            .engine
            .input
            .is_mouse_button_active(MouseButton::Left);
        action_state.is_left_hand_active = is_lmb_active;
        action_state.is_right_hand_active = ctx
            .engine
            .input
            .is_mouse_button_active(MouseButton::Right)
            || (is_lmb_active && ctx.engine.input.is_left_control_active());

        handle.input.set_action_state(action_state);

        // Process keyboard input
        let mut forces = vec3i8::ZERO;
        if ctx.engine.input.is_key_active(KeyCode::KeyW) {
            forces.x += 1;
        }
        if ctx.engine.input.is_key_active(KeyCode::KeyS) {
            forces.x -= 1;
        }
        if ctx.engine.input.is_key_active(KeyCode::KeyA) {
            forces.z += 1;
        }
        if ctx.engine.input.is_key_active(KeyCode::KeyD) {
            forces.z -= 1;
        }
        if ctx.engine.input.is_key_active(KeyCode::Space) {
            forces.y += 1;
        }
        if ctx.engine.input.is_key_active(KeyCode::ShiftLeft) {
            forces.y -= 1;
        }

        handle.input.set_movement(forces);

        // Process mouse movement
        let movement = ctx.input.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            handle.input.push_mouse_movement(movement);
        }
    }

    pub fn position(&self) -> vec3d {
        self.camera.video.position
    }
}

fn cube(position: vec3d, color: Rgba<f32>) -> impl IntoIterator<Item = Instance3d> {
    Face::values()
        .map(Face::to_rotation)
        .map(move |rotation| Instance3d::new(position, rotation, Vec3::splat(1.0), color, 1))
}
