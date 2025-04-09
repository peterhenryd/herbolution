use crossbeam::channel::{bounded, Receiver, Sender};
use math::angle::Rad;
use math::rotation::Euler;
use math::vector::vec3f;
use crate::world::entity::EntityTarget;

pub fn client_output_channel() -> (ClientOutputSender, ClientOutputReceiver) {
    let (camera_pos_tx, camera_pos_rx) = bounded(1);
    let (camera_rotation_tx, camera_rotation_rx) = bounded(1);
    let (entity_target_tx, entity_target_rx) = bounded(1);

    (
        ClientOutputSender {
            camera_pos: camera_pos_tx,
            camera_rotation: camera_rotation_tx,
            entity_target: entity_target_tx,
        },
        ClientOutputReceiver {
            camera_pos: camera_pos_rx,
            camera_rotation: camera_rotation_rx,
            entity_target: entity_target_rx,
        }
    )
}

#[derive(Debug, Clone)]
pub struct ClientOutputSender {
    camera_pos: Sender<vec3f>,
    camera_rotation: Sender<Euler<Rad<f32>>>,
    entity_target: Sender<Option<EntityTarget>>
}

impl ClientOutputSender {
    pub fn send_camera_pos(&self, pos: vec3f) {
        let _ = self.camera_pos.try_send(pos);
    }

    pub fn send_camera_rotation(&self, rotation: Euler<Rad<f32>>) {
        let _ = self.camera_rotation.try_send(rotation);
    }

    pub fn send_target(&self, target: Option<EntityTarget>) {
        let _ = self.entity_target.try_send(target);
    }
}

pub struct ClientOutputReceiver {
    camera_pos: Receiver<vec3f>,
    camera_rotation: Receiver<Euler<Rad<f32>>>,
    entity_target: Receiver<Option<EntityTarget>>
}

impl ClientOutputReceiver {
    pub fn recv_camera_pos(&self) -> Option<vec3f> {
        self.camera_pos.try_recv().ok()
    }

    pub fn recv_camera_rotation(&self) -> Option<Euler<Rad<f32>>> {
        self.camera_rotation.try_recv().ok()
    }

    pub fn recv_target(&self) -> Option<Option<EntityTarget>> {
        self.entity_target.try_recv().ok()
    }
}