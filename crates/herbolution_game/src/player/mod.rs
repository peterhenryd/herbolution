pub mod handle;

use crate::chunk::map::CubeHit;
use crate::entity::behavior::{EntityBehavior, EntityBehaviorType, EntityContext};
use crate::entity::{ActionState, ActionTarget};
use crate::handle::Particle;
use crate::player::handle::ClientPlayerHandle;
use herbolution_math::spatial::aabb::Aabb;
use lib::motile::Motile;
use lib::util::default;
use math::rotation::Euler;
use math::vector::{vec3i, Vec2, Vec3};
use std::any::Any;
use std::time::Duration;

#[derive(Debug)]
pub struct Player {
    action_state: ActionState,
    handle: ClientPlayerHandle,
}

impl Player {
    pub fn new(handle: ClientPlayerHandle) -> Self {
        Self {
            action_state: ActionState::default(),
            handle,
        }
    }

    fn handle_actions(&mut self, ctx: &mut EntityContext) {
        let origin = ctx.entity.body().eye_position();
        let direction = ctx.entity.body().rotation().into_view_center();

        let Some(cube_hit) = ctx.chunk_map.cast_ray(origin, direction, 5.0) else {
            self.handle.transform.set_target(None);
            return;
        };

        self.handle
            .transform
            .set_target(ActionTarget::Cube(cube_hit.position));

        if self.action_state.is_left_hand_active {
            self.handle_left_hand(cube_hit.position, ctx);
            self.action_state.is_left_hand_active = false;
        }

        if self.action_state.is_right_hand_active {
            self.handle_right_hand(cube_hit, ctx);
            self.action_state.is_right_hand_active = false;
        }
    }

    fn handle_left_hand(&mut self, position: vec3i, ctx: &mut EntityContext) {
        let material = ctx.chunk_map.get_material(position).unwrap();
        ctx.chunk_map.set_cube(position, None);

        for _ in 0..32 {
            let center = position.cast::<f64>();

            let mut offset = Vec3::by_index(|_| fastrand::f64());
            offset[fastrand::usize(0..=2)] = fastrand::f64().round();

            let position = center + offset - 0.5;
            let final_position = position + offset / 2.0;
            let dir = (final_position - center).normalize();

            let _ = ctx.handle.particle_tx.try_send(Particle {
                position,
                rotation: None,
                motile: Motile {
                    dir,
                    drive: 12.0,
                    jump: 0.2,
                    ..default()
                },
                lifetime: Duration::SECOND,
                color: material.get_color(fastrand::f32()),
            });
        }
    }

    fn handle_right_hand(&mut self, cube_hit: CubeHit, ctx: &mut EntityContext) {
        let position = cube_hit.position + cube_hit.face.to_normal();
        let collider = Aabb::new(
            Vec3::new(position.x as f32, position.y as f32, position.z as f32),
            Vec3::new(position.x as f32 + 1.0, position.y as f32 + 1.0, position.z as f32 + 1.0),
        );
        if !collider
            .try_cast()
            .unwrap()
            .intersects(&ctx.entity.body().bounds())
        {
            ctx.chunk_map
                .set_cube(position, ("herbolution", "stone"));
        }
    }
}

impl EntityBehavior for Player {
    fn update(&mut self, ctx: &mut EntityContext) {
        let body = ctx.entity.body_mut();

        if let Some(command) = self.handle.input.next_movement() {
            body.apply_motion_command(command);
        }

        while let Some(Vec2 { x: dx, y: dy }) = self.handle.input.next_mouse_movement() {
            *body.rotation_mut() -= Euler {
                yaw: dx.to_radians() as f32,
                pitch: dy.to_radians() as f32,
                ..default()
            };
        }

        self.handle
            .transform
            .set_position(body.eye_position());
        self.handle
            .transform
            .set_rotation(*body.rotation());

        if let Some(action_state) = self.handle.input.next_action_state() {
            self.action_state = action_state;
        }

        self.handle_actions(ctx);
    }

    fn select_from(behavior: &mut EntityBehaviorType) -> Option<&mut Self>
    where
        Self: Sized,
    {
        match behavior {
            EntityBehaviorType::Player(x) => Some(x),
            EntityBehaviorType::ChunkLoader(_) => None,
            EntityBehaviorType::Dyn(x) => (x.as_mut() as &mut dyn Any).downcast_mut(),
            _ => None,
        }
    }
}
