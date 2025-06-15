use std::time::Duration;

use engine::Engine;
use engine::input::{Frame, Input};
use engine::video::gpu::SetId;
use engine::video::gpu::camera::View;
use engine::video::{gpu, v3d};
use game::channel::ClientChannel;
use game::client::{ClientInputSender, ClientOutputReceiver, client_input_channel};
use game::entity::EntityTarget;
use game::entity::logic::player::ActionState;
use lib::geo::face::Face;
use math::color::{ColorConsts, Rgba};
use math::proj::Perspective;
use math::size::Size2;
use math::vector::{Vec3, vec3d, vec3i, vec3i8};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use crate::world::frustum::Frustum;

/// The render-side representation of the player within the world.
#[derive(Debug)]
pub struct Player {
    /// The channel used to send input commands to the logic-side player.
    tx: ClientInputSender,
    /// The channel used to receive updates from the logic-side player.
    pub(crate) rx: Option<ClientOutputReceiver>,
    /// The previous entity or cube the player was looking at.
    prev_target: Option<vec3i>,
    /// The camera, positioned at the player's eye.
    pub(crate) camera: Camera,
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
    /// Creates a new instance, while also sending a signal to the logic-side game to create the player entity.
    pub fn create(channel: &ClientChannel, sky_box_color: Rgba<f32>, engine: &mut Engine) -> Self {
        let (input_sender, input_receiver) = client_input_channel();
        channel.send_client_input(input_receiver);

        Self {
            tx: input_sender,
            rx: None,
            prev_target: None,
            //prev_position: Vec3::ZERO,
            //position: Vec3::ZERO,
            //view_bob: 0.0,
            camera: Camera::new(engine.video.resolution(), &mut engine.video.r3d),
            targeted_cube_wireframe_id: engine
                .video
                .r3d
                .sets()
                .insert_with_capacity(6),
            sky_box_color,
            sky_box_id: engine
                .video
                .r3d
                .sets()
                .insert_raw(cube(Vec3::ZERO, sky_box_color)),
        }
    }

    /// Updates the player state.
    pub fn update(&mut self, _: Duration, engine: &mut Engine, input_frame: &Frame, is_focused: bool) {
        // If the logic-side game has not processed the player creation yet, skip the update.
        let Some(output_receiver) = &mut self.rx else { return };

        // Receive the latest camera position and rotation from the logic-side player.
        let mut update_camera = false;
        if let Some(x) = output_receiver.recv_camera_pos() {
            //self.prev_position = self.position;
            //self.position = position;

            // TODO: receive the player position instead of the camera position, and setup a different channel for updating the relative eye position.

            self.camera.video.position = x;
            update_camera = true;

            engine
                .video
                .r3d
                .sets()
                .write_raw(self.sky_box_id, cube(x.cast().unwrap(), self.sky_box_color))
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

        if let Some(x) = output_receiver.recv_camera_rotation() {
            *self
                .camera
                .video
                .view
                .rotation()
                .unwrap() = x;
            update_camera = true;
        }

        if update_camera {
            self.camera.update(engine);
        }

        // Overwrite the targeted cube wireframe with the latest target from the logic-side player.
        match output_receiver.recv_target() {
            Some(Some(EntityTarget::Cube(pos))) => {
                if self.prev_target != Some(pos) {
                    self.set_targeted_cube(engine.video.r3d.sets(), Some(pos));
                    self.prev_target = Some(pos);
                }
            }
            Some(Some(EntityTarget::Entity(_))) => {}
            Some(None) => {
                if self.prev_target.is_some() {
                    self.set_targeted_cube(engine.video.r3d.sets(), None);
                    self.prev_target = None;
                }
            }
            None => {}
        }

        if is_focused {
            self.dequeue_input(input_frame, &engine.input);
        }
    }

    fn set_targeted_cube(&mut self, sets: &mut v3d::Sets, position: Option<vec3i>) {
        match position {
            None => sets.write(self.targeted_cube_wireframe_id, []),
            Some(x) => sets.write_raw(self.targeted_cube_wireframe_id, cube(x.cast().unwrap(), Rgba::BLACK)),
        }
        .expect("Failed to clear targeted cube wireframe instances");
    }

    fn dequeue_input(&mut self, frame: &Frame, input: &Input) {
        // Process mouse button input
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
            self.tx.set_action_state(action_state);
        }

        // Process keyboard input
        let mut forces = vec3i8::ZERO;
        if input.is_key_active(KeyCode::KeyW) {
            forces.x += 1;
        }
        if input.is_key_active(KeyCode::KeyS) {
            forces.x -= 1;
        }
        if input.is_key_active(KeyCode::KeyA) {
            forces.z += 1;
        }
        if input.is_key_active(KeyCode::KeyD) {
            forces.z -= 1;
        }
        if input.is_key_active(KeyCode::Space) {
            forces.y += 1;
        }
        if input.is_key_active(KeyCode::ShiftLeft) {
            forces.y -= 1;
        }

        self.tx.set_movement_command(forces);

        // Process mouse movement
        let movement = frame.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            self.tx.add_mouse_movement(movement);
        }
    }
}

fn cube(position: vec3d, color: Rgba<f32>) -> impl IntoIterator<Item = v3d::InstancePayload> {
    Face::entries()
        .map(Face::to_rotation)
        .map(move |rotation| {
            v3d::Instance {
                position,
                rotation,
                color,
                ..Default::default()
            }
            .payload()
        })
}

/// The camera with additional information used for culling and camera-relative rendering.
#[derive(Debug)]
pub struct Camera {
    pub(crate) video: gpu::Camera<Perspective>,
    pub(crate) frustum: Frustum,
    pub(crate) chunk_position: vec3i,
}

impl Camera {
    /// Creates a new instance.
    pub fn new(resolution: Size2<u32>, r3d: &mut v3d::Renderer) -> Self {
        let aspect = resolution
            .cast::<f32>()
            .unwrap()
            .aspect();
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.001, 500.0);
        let video = gpu::Camera::new(Vec3::ZERO, View::rotatable(), perspective);
        r3d.update_camera(&video);

        Self {
            frustum: Frustum::new(video.view_proj()),
            video,
            chunk_position: vec3i::ZERO,
        }
    }

    /// Submits the camera to the video state and calculates a new frustum and chunk position.
    fn update(&mut self, engine: &mut Engine) {
        engine
            .video
            .r3d
            .update_camera(&self.video);
        self.frustum = Frustum::new(self.video.view_proj());
        self.chunk_position = self.video.position.cast().unwrap() / game::chunk::LENGTH as i32;
    }
}
