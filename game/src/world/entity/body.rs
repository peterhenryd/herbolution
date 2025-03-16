use crate::world::chunk::map::ChunkMap;
use crate::world::entity::EntityAbilities;
use math::transform::Transform;
use math::vector::vec3f;
use std::time::Duration;
use lib::geometry::cuboid::Cuboid;

#[derive(Debug, Clone)]
pub struct EntityBody {
    pub transform: Transform,
    pub bounding_box: Cuboid<f32>,
    pub eye_offset: vec3f,
    pub gravity: EntityGravity,
}

impl EntityBody {
    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap, abilities: EntityAbilities) {
        if abilities.is_affected_by_gravity {
            self.gravity.update(dt);
            self.apply_gravity(dt, chunk_map);
        }
    }

    fn apply_gravity(&mut self, dt: Duration, chunk_map: &mut ChunkMap) {
        let dy = self.gravity.get_dy(dt);
        let new_position = self.transform.position + vec3f::new(0.0, dy, 0.0);
        let new_bounding_box = self.bounding_box + new_position;

        for collider in chunk_map.get_near_colliders(new_bounding_box) {
            if new_bounding_box.intersects(&collider) {
                return;
            }
        }

        self.transform.position = new_position;
    }

    pub fn bounds(&self) -> Cuboid<f32> {
        self.bounding_box + self.transform.position
    }

    pub fn get_eye_position(&self) -> vec3f {
        self.transform.position + self.eye_offset
    }
}

#[derive(Debug, Clone)]
pub struct EntityGravity {
    pub(crate) fall_acceleration: f32,
    pub(crate) fall_speed: f32,
    pub(crate) max_fall_speed: f32,
}

impl EntityGravity {
    pub fn update(&mut self, dt: Duration) {
        self.fall_speed += self.fall_acceleration * dt.as_secs_f32();
        self.fall_speed = self.fall_speed.min(self.max_fall_speed);
    }

    pub fn get_dy(&self, dt: Duration) -> f32 {
        self.fall_speed * dt.as_secs_f32()
    }
}
