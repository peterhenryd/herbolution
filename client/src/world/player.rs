use lib::color::{ColorConsts, Rgba};
use server::entity::{ActionState, ActionTarget, CubeTarget};
use server::player::handle::ServerPlayerHandle;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

/// The video-side representation of the player within the world.
#[derive(Debug)]
pub struct Player {
    /// The channel used to receive updates from the behavior-side player.
    pub(crate) handle: Option<ServerPlayerHandle>,
    /// The previous entity or cube the player was looking at.
    prev_target: Option<CubeTarget>,
    /// The camera, positioned at the player's eye.
    pub(crate) camera: PlayerCamera,
    //prev_position: vec3f,
    //position: vec3f,
    //view_bob: f32,
    /// The identifier for the GPU buffer containing the quads to be rendered as a wireframe when the player is targeting a cube.
    pub(crate) targeted_cube_wireframe_id: SetId,
    pub(crate) targeted_cube_shell_id: SetId,
    /// The color of the sky box.
    sky_box_color: Rgba<f32>,
    /// The identifier for the GPU buffer containing the quads of the sky box, which is rendered as a cube around the player.
    pub(crate) sky_box_id: SetId,
    pub(crate) health: Health,
}

impl Player {
    /// Creates a new instance, while also sending a signal to the behavior-side server to create the player entity.
    pub fn create(sky_box_color: Rgba<f32>, video: &mut Video) -> Self {
        Self {
            handle: None,
            prev_target: None,
            //prev_position: Vec3::ZERO,
            //position: Vec3::ZERO,
            //view_bob: 0.0,
            camera: PlayerCamera::new(video.resolution(), &mut video.sculptor),
            targeted_cube_wireframe_id: video.sculptor.sets().insert_with_capacity(6),
            targeted_cube_shell_id: video.sculptor.sets().insert_with_capacity(6),
            sky_box_color,
            sky_box_id: video
                .sculptor
                .sets()
                .insert_from(cube(Vec3::ZERO, sky_box_color)),
            health: Health::default(),
        }
    }

    /// Updates the player state.
    pub fn update(&mut self, ctx: &mut Update) {
        // If the behavior-side server has not processed the player creation yet, skip the update.
        let Some(handle) = &mut self.handle else { return };

        // Receive the latest camera position and rotation from the behavior-side player.
        let mut update_camera = false;
        if let Some(x) = handle.transform.next_position() {
            //self.prev_position = self.position;
            //self.position = position;

            // TODO: receive the player position instead of the camera position, and setup a different channel for updating the relative eye position.

            self.camera.position = x;
            update_camera = true;

            ctx.video
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

        if let Some(health) = handle.next_health() {
            self.health = health;
        }

        if let Some(x) = handle.transform.next_rotation() {
            self.camera.rotation = x;
            update_camera = true;
        }

        if update_camera {
            self.camera.update(ctx.video);
        }

        let sets = ctx.video.sculptor.sets();
        // Overwrite the targeted cube wireframe with the latest target from the server-side player.
        match handle.transform.next_target() {
            Some(Some(ActionTarget::Cube(target))) => {
                self.set_targeted_cube(sets, target)
                    .expect("Failed to set targeted cube");
                self.prev_target = Some(target);
            }
            Some(Some(ActionTarget::Entity(_))) => {}
            Some(None) => {
                sets.get_mut(self.targeted_cube_wireframe_id)
                    .set_len(0);
                sets.get_mut(self.targeted_cube_shell_id)
                    .set_len(0);
                self.prev_target = None;
            }
            _ => {}
        }
    }

    fn set_targeted_cube(&mut self, sets: &mut Sets<Instance3d>, target: CubeTarget) -> Result<(), ()> {
        let position = target.position.cast();
        if target.shell_opacity == 0.0 {
            sets.write(self.targeted_cube_shell_id, &[])?;
            sets.write_from(self.targeted_cube_wireframe_id, cube(position, Rgba::BLACK))?;
        } else {
            sets.write_from(self.targeted_cube_shell_id, cube(position, Rgba::new(1.0, 1.0, 1.0, target.shell_opacity)))?;
            sets.write(self.targeted_cube_wireframe_id, &[])?;
        }
        Ok(())
    }

    pub fn update_input(&mut self, ctx: &Update) {
        let Some(handle) = &self.handle else {
            return;
        };

        // Process mouse button input
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

        handle.input.set_action_state(action_state);

        // Process keyboard input
        let mut forces = vec3i8::ZERO;
        if ctx.store.input.is_key_active(KeyCode::KeyW) {
            forces.x += 1;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyS) {
            forces.x -= 1;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyA) {
            forces.z += 1;
        }
        if ctx.store.input.is_key_active(KeyCode::KeyD) {
            forces.z -= 1;
        }
        if ctx.store.input.is_key_active(KeyCode::Space) {
            forces.y += 1;
        }
        if ctx.store.input.is_key_active(KeyCode::ShiftLeft) {
            forces.y -= 1;
        }

        handle.input.set_movement(forces);

        // Process mouse movement
        let movement = ctx.input.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            handle.input.push_mouse_movement(movement);
        }

        handle
            .input
            .push_speed_delta(ctx.input.mouse_scroll);
    }
}

fn cube(position: vec3d, color: Rgba<f32>) -> impl IntoIterator<Item = Instance3d> {
    Face::values()
        .map(Face::to_rotation)
        .map(move |rotation| Instance3d::new(position, rotation, Vec3::splat(1.0), color, 1))
}

use lib::matrix::Mat4;
use lib::proj::Perspective;
use lib::rotation::Euler;
use lib::size::size2u;
use lib::spatial::Face;
use lib::vector::{Vec3, vec3d, vec3i, vec3i8};
use lib::world::{CHUNK_LENGTH, Health};

use crate::app::Update;
use crate::video::Video;
use crate::video::camera::{VideoCamera, View};
use crate::video::resource::{SetId, Sets};
use crate::video::world::{Instance3d, Sculptor};
use crate::world::frustum::Frustum;

/// The camera with additional information used for culling and camera-relative rendering.
#[derive(Debug)]
pub struct PlayerCamera {
    pub(crate) position: vec3d,
    rotation: Euler<f32>,
    pub(crate) frustum: Frustum,
    pub(crate) chunk_position: vec3i,
    perspective: Perspective,
}

impl PlayerCamera {
    /// Creates a new instance.
    pub fn new(resolution: size2u, sculptor: &mut Sculptor) -> Self {
        let aspect = resolution.cast::<f32>().unwrap().aspect();
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.001, 500.0);

        sculptor.update_camera(&VideoCamera::new(Vec3::ZERO, View::rotatable(), perspective));

        Self {
            position: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            frustum: Frustum::new(Mat4::IDENTITY),
            chunk_position: vec3i::ZERO,
            perspective,
        }
    }

    #[inline]
    fn to_video_camera(&self) -> VideoCamera {
        VideoCamera::new(self.position, View::Rotate { rotation: self.rotation }, self.perspective)
    }

    /// Submits the camera to the video state and calculates a new frustum and chunk position.
    pub fn update(&mut self, video: &mut Video) {
        let video_camera = self.to_video_camera();

        video.sculptor.update_camera(&video_camera);
        self.frustum = Frustum::new(video_camera.view_proj);

        self.chunk_position = self.position.cast() / CHUNK_LENGTH as i32;
    }
}
