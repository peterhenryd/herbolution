use crate::world::chunk::map::ChunkMap;
use crate::world::entity::EntityAbilities;
use lib::geometry::cuboid::Cuboid;
use math::num::traits::ConstZero;
use math::transform::Transform;
use math::vector::{vec3f, Vec3};

#[derive(Debug, Clone)]
pub struct EntityBody {
    pub transform: Transform,
    pub(crate) forces: vec3f,
    pub bounding_box: Cuboid<f32>,
    pub eye_offset: vec3f,
    pub gravity: EntityGravity,
}

impl EntityBody {
    pub fn update(&mut self, chunk_map: &mut ChunkMap, abilities: EntityAbilities) {
        if abilities.is_affected_by_gravity {
            self.gravity.update();
            self.apply_gravity(chunk_map);
        }

        self.apply_translation(chunk_map, &abilities);
    }

    fn apply_translation(
        &mut self,
        chunk_map: &mut ChunkMap,
        _: &EntityAbilities,
    ) {
        let (straight, side) = self.transform.rotation.into_view_directions();
        let (straight, side) = (straight.cast::<f32>().unwrap(), side.cast::<f32>().unwrap());
        let up = Vec3::Y;

        let mut velocity = Vec3::ZERO;
        velocity += straight * self.forces.x;
        velocity += side * self.forces.z;
        velocity += up * self.forces.y;

        if velocity != Vec3::ZERO {
            velocity = velocity.normalize() * 0.5;
        }

        let bounds = self.bounds();
        for collider in chunk_map.get_near_colliders(bounds) {
            collider.clamp_collision_velocity(&bounds, &mut velocity);
        }

        self.transform.position += velocity;
    }

    fn apply_gravity(&mut self, chunk_map: &mut ChunkMap) {
        let dy = self.gravity.fall_speed;
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
    pub fn update(&mut self) {
        self.fall_speed += self.fall_acceleration;
        self.fall_speed = self.fall_speed.min(self.max_fall_speed);
    }
}
