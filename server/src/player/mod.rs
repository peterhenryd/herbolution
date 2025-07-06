use std::any::Any;
use std::mem::take;
use std::sync::Arc;
use std::time::Duration;

use crate::chunk::map::CubeHit;
use crate::chunk::material::Material;
use crate::entity::behavior::{EntityBehavior, EntityBehaviorType, EntityContext};
use crate::entity::{ActionState, ActionTarget, CubeTarget};
use crate::handle::Particle;
use crate::player::handle::ClientPlayerHandle;
use lib::motile::Motile;
use lib::rotation::Euler;
use lib::spatial::Aabb;
use lib::util::default;
use lib::vector::{Vec2, Vec3};
use lib::world::Health;

pub mod handle;

#[derive(Debug)]
pub struct Player {
    action_state: ActionState,
    handle: ClientPlayerHandle,
    prev_target: Option<ActionTarget>,
    health: Health,
    regeneration: f32,
    dig_speed: f32,
    dig_state: Option<DigState>,
}

#[derive(Debug)]
struct DigState {
    remaining_time: f32,
    material: Arc<Material>,
}

impl Player {
    pub fn new(handle: ClientPlayerHandle) -> Self {
        Self {
            action_state: ActionState::default(),
            handle,
            prev_target: None,
            health: Health::new(100.0),
            regeneration: 1.0,
            dig_speed: 1.0,
            dig_state: None,
        }
    }

    fn handle_interaction(&mut self, ctx: &mut EntityContext) {
        let ray_origin = ctx.entity.body().eye_position();
        let ray_dir = ctx.entity.body().rotation().into_view_center();
        let cube_hit = ctx.chunk_map.cast_ray(ray_origin, ray_dir, 100.0);

        let current_target = cube_hit.as_ref().map(|hit| {
            ActionTarget::Cube(CubeTarget {
                position: hit.position,
                shell_opacity: self
                    .dig_state
                    .as_ref()
                    .map(|x| 0.5 - (x.remaining_time * self.dig_speed) / x.material.toughness * 0.5)
                    .unwrap_or(0.0),
            })
        });

        if current_target != self.prev_target {
            self.handle.transform.set_target(current_target);
        }

        if let (Some(a), Some(b)) = (&current_target, &self.prev_target)
            && !a.stateless_eq(b)
        {
            self.dig_state = None;
        }
        self.prev_target = current_target;

        if let Some(hit) = cube_hit {
            if self.action_state.is_left_hand_active {
                self.process_digging(ctx, hit);
            }

            if self.action_state.is_right_hand_active {
                self.handle_right_hand(hit, ctx);
            }
        }
    }

    fn process_digging(&mut self, ctx: &mut EntityContext, cube_hit: CubeHit) {
        if self.dig_state.is_none() {
            let material = ctx
                .chunk_map
                .get_material(cube_hit.position)
                .unwrap();
            self.dig_state = Some(DigState {
                remaining_time: material.toughness / self.dig_speed,
                material,
            })
        }

        let Some(state) = &mut self.dig_state else {
            return;
        };

        if fastrand::f32() < 0.001 {
            let _ = ctx.handle.particle_tx.try_send(Particle {
                position: cube_hit.contact_point - Vec3::splat(0.5) + Vec3::by_index(|_| fastrand::f64() - 0.5) / 10.0,
                rotation: None,
                motile: Motile {
                    dir: Vec3::by_index(|_| fastrand::f64() - 0.5).normalize(),
                    drive: 4.0,
                    jump: 0.2,
                    ..default()
                },
                lifetime: Duration::SECOND,
                color: state.material.get_color(fastrand::f32()),
            });
        }

        state.remaining_time -= ctx.dt.as_secs_f32();
        let finished = state.remaining_time <= 0.0;

        if finished {
            let material = take(&mut self.dig_state).unwrap().material;

            self.spawn_dig_particles(ctx, cube_hit.position, &material);
            ctx.chunk_map.set_cube(cube_hit.position, None);
            self.handle.transform.set_target(None);
        }
    }

    fn spawn_dig_particles(&self, ctx: &mut EntityContext, cube_pos: Vec3<i32>, material: &Material) {
        for _ in 0..32 {
            let center = cube_pos.cast::<f64>();

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
                .set_cube(position, "herbolution:stone");
        }
    }
}

impl EntityBehavior for Player {
    fn update(&mut self, ctx: &mut EntityContext) {
        self.health += self.regeneration * ctx.dt.as_secs_f32();

        let body = ctx.entity.body_mut();

        if let Some(command) = self.handle.input.next_movement() {
            body.motion = command.cast();
        }

        while let Some(Vec2 { x: dx, y: dy }) = self.handle.input.next_mouse_movement() {
            *body.rotation_mut() -= Euler {
                yaw: dx.to_radians() as f32,
                pitch: dy.to_radians() as f32,
                ..default()
            };
        }

        while let Some(speed_delta) = self.handle.input.next_speed_delta() {
            let speed = &mut body.attrs.acceleration_rate;
            *speed += speed_delta as f64;
            *speed = speed.max(0.0);
        }

        self.handle
            .transform
            .set_position(body.eye_position());
        self.handle
            .transform
            .set_rotation(*body.rotation());

        if let Some(fall_distance) = ctx.entity.body.last_fell.take() {
            dbg!(fall_distance);
            if fall_distance > 3.0 {
                self.health -= (fall_distance as f32 - 3.0) * 2.0;
            }
        }

        self.handle.set_health(self.health);

        if let Some(action_state) = self.handle.input.next_action_state() {
            if !action_state.is_left_hand_active {
                self.dig_state = None;
            }

            self.action_state = action_state;
        }

        self.handle_interaction(ctx);
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
