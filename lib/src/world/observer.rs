use crate::engine::uniform::Uniform;
use crate::world::camera::proj::perspective::Perspective;
use crate::world::camera::Camera;
use crate::world::entity::set::{EntityId, EntitySet};

pub struct Observer {
    pub(crate) entity_id: EntityId,
    pub abilities: Abilities,
}

impl Observer {
    pub fn update(&mut self, set: &mut EntitySet, camera: &mut Uniform<Camera<Perspective>>) {
        let entity = set.get_mut(self.entity_id).unwrap();

        if entity.physics.transform.position != camera.transform.position
            || entity.physics.transform.rotation != camera.transform.rotation
        {
            camera.edit(|c| c.transform = entity.physics.transform);
        }
    }
}

pub struct Abilities {
    // TODO: make this granular
    pub accepts_input: bool,
}