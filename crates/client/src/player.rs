use crate::chunk::Frustum;
use engine::input::{Input, InputFrame};
use engine::video::mem::instance::InstanceSetId;
use engine::video::r3d::{Instance3d, Instance3dPayload, Renderer3d, Sets3d};
use engine::{video, Engine};
use game::channel::ClientChannel;
use game::client::{client_input_channel, ClientInputSender, ClientOutputReceiver};
use game::entity::logic::player::ActionState;
use game::entity::EntityTarget;
use lib::geo::face::Face;
use math::color::{ColorConsts, Rgba};
use math::proj::Perspective;
use math::rotation::Euler;
use math::size::Size2;
use math::vector::{vec3d, vec3i, vec3i8, Vec3};
use std::time::Duration;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

pub struct Player {
    tx: ClientInputSender,
    pub(crate) rx: Option<ClientOutputReceiver>,
    prev_target: Option<vec3i>,
    pub(crate) camera: Camera,
    //prev_position: vec3f,
    //position: vec3f,
    //view_bob: f32,
    pub(crate) targeted_cube_wireframe_id: InstanceSetId,
    sky_box_color: Rgba<f32>,
    pub(crate) sky_box_id: InstanceSetId,
}

impl Player {
    pub fn create(channel: &ClientChannel, resolution: Size2<u32>, sky_box_color: Rgba<f32>, engine: &mut Engine) -> Self {
        let (input_sender, input_receiver) = client_input_channel();
        channel.send_client_input(input_receiver);

        Self {
            tx: input_sender,
            rx: None,
            prev_target: None,
            //prev_position: Vec3::ZERO,
            //position: Vec3::ZERO,
            //view_bob: 0.0,
            camera: Camera::new(resolution, &mut engine.video.r3d),
            targeted_cube_wireframe_id: engine.video.r3d.sets().insert_with_capacity(6),
            sky_box_color,
            sky_box_id: engine.video.r3d.sets().insert_raw(cube(Vec3::ZERO, sky_box_color))
        }
    }

    pub(crate) fn update(&mut self, _: Duration, engine: &mut Engine, input_frame: &InputFrame, is_focused: bool) {
        let Some(output_receiver) = &mut self.rx else { return };

        let mut update_camera = false;
        if let Some(x) = output_receiver.recv_camera_pos() {
            //self.prev_position = self.position;
            //self.position = position;
            self.camera.video.position = x;
            update_camera = true;
            
            engine.video.r3d.sets().write_raw(self.sky_box_id, cube(x.cast().unwrap(), self.sky_box_color))
                .expect("Failed to update sky box");
        }

        /*if (self.prev_position - self.position).length_squared() > 0.5 * dt.as_secs_f32() {
            self.view_bob += dt.as_secs_f32() * 10.0;
        } else {
            self.view_bob = 0.0;
        }

        camera_position.y += self.view_bob.sin() * 0.1;

         */


        if let Some(x) = output_receiver.recv_camera_rotation() {
            self.camera.video.rotation = x;
            update_camera = true;
        }
        
        if update_camera {
            self.camera.update(engine);
        }

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
    
    fn set_targeted_cube(&mut self, sets: &mut Sets3d, position: Option<vec3i>) {
        match position {
            None => sets.write(self.targeted_cube_wireframe_id, []),
            Some(x) => sets.write_raw(self.targeted_cube_wireframe_id, cube(x.cast().unwrap(), Rgba::BLACK)),
        }.expect("Failed to clear targeted cube wireframe instances");
    }

    fn dequeue_input(&mut self, frame: &InputFrame, input: &Input) {
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
            self.tx.set_action_state(action_state);
        }

        // Keyboard
        let mut forces = vec3i8::ZERO;
        if input.is_key_active(KeyCode::KeyW) { forces.x += 1; }
        if input.is_key_active(KeyCode::KeyS) { forces.x -= 1; }
        if input.is_key_active(KeyCode::KeyA) { forces.z += 1; }
        if input.is_key_active(KeyCode::KeyD) { forces.z -= 1; }
        if input.is_key_active(KeyCode::Space) { forces.y += 1; }
        if input.is_key_active(KeyCode::ShiftLeft) { forces.y -= 1; }

        self.tx.set_movement_command(forces);

        // Mouse movement
        let movement = frame.mouse_movement;
        if movement.x != 0.0 || movement.y != 0.0 {
            self.tx.add_mouse_movement(movement);
        }
    }
}

fn cube(position: vec3d, color: Rgba<f32>) -> impl IntoIterator<Item = Instance3dPayload> {
    Face::entries()
        .map(Face::to_rotation)
        .map(move |rotation| Instance3d {
            position,
            rotation,
            color,
            ..Default::default()
        }.payload())
}

pub struct Camera {
    pub(crate) video: video::Camera<Perspective>,
    pub(crate) frustum: Frustum,
    pub(crate) chunk_position: vec3i,
}

impl Camera {
    pub fn new(resolution: Size2<u32>, r3d: &mut Renderer3d) -> Self {
        let aspect = resolution.cast::<f32>().unwrap().aspect();
        let perspective = Perspective::new(70f32.to_radians(), aspect, 0.001, 500.0);
        let video = video::Camera::new(Vec3::ZERO, Euler::IDENTITY, perspective);
        r3d.update_camera(&video);
        
        Self {
            frustum: Frustum::new(video.view_proj()),
            video,
            chunk_position: vec3i::ZERO,
        }
    }
    
    fn update(&mut self, engine: &mut Engine) {
        engine.video.r3d.update_camera(&self.video);
        self.frustum = Frustum::new(self.video.view_proj());
        self.chunk_position = self.video.position.cast().unwrap() / game::chunk::LENGTH as i32;
    }
}