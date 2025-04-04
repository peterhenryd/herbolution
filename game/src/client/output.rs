use crossbeam::channel::{bounded, Receiver, Sender};
use math::angle::Rad;
use math::rotation::Euler;
use math::vector::vec3f;
use crate::world::entity::EntityTarget;

pub fn client_output_channel() -> (ClientOutputSender, ClientOutputReceiver) {
    let (camera_position_tx, camera_position_rx) = bounded(1);
    let (camera_rotation_tx, camera_rotation_rx) = bounded(1);
    let (entity_target_tx, entity_target_rx) = bounded(1);

    (
        ClientOutputSender {
            camera_position: camera_position_tx,
            camera_rotation: camera_rotation_tx,
            entity_target: entity_target_tx,
        },
        ClientOutputReceiver {
            camera_position: camera_position_rx,
            camera_rotation: camera_rotation_rx,
            entity_target: entity_target_rx,
        }
    )
}

#[derive(Debug, Clone)]
pub struct ClientOutputSender {
    camera_position: Sender<vec3f>,
    camera_rotation: Sender<Euler<Rad<f32>>>,
    entity_target: Sender<Option<EntityTarget>>
}

impl ClientOutputSender {
    pub fn set_camera_position(&self, position: vec3f) {
        let _ = self.camera_position.try_send(position);
    }

    pub fn set_camera_rotation(&self, rotation: Euler<Rad<f32>>) {
        let _ = self.camera_rotation.try_send(rotation);
    }

    pub fn set_target(&self, target: Option<EntityTarget>) {
        let _ = self.entity_target.try_send(target);
    }
}

pub struct ClientOutputReceiver {
    camera_position: Receiver<vec3f>,
    camera_rotation: Receiver<Euler<Rad<f32>>>,
    entity_target: Receiver<Option<EntityTarget>>
}

impl ClientOutputReceiver {
    pub fn receive_camera_position(&self) -> Option<vec3f> {
        self.camera_position.try_recv().ok()
    }

    pub fn receive_camera_rotation(&self) -> Option<Euler<Rad<f32>>> {
        self.camera_rotation.try_recv().ok()
    }

    pub fn receive_target(&self) -> Option<Option<EntityTarget>> {
        self.entity_target.try_recv().ok()
    }
}