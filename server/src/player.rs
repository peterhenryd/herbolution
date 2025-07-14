use crate::chunk::map::CubeHit;
use crate::chunk::material::Material;
use crate::entity::behavior::{EntityBehavior, EntityBehaviorType, EntityContext};
use crate::entity::{ActionState, ActionTarget, CubeTarget};
use crate::handle::Particle;
use arc_swap::ArcSwapOption;
use crossbeam_channel::{Receiver, Sender};
use lib::aabb::Aabb3;
use lib::motile::Motile;
use lib::rotation::Euler;
use lib::util::default;
use lib::vector::{vec2d, vec3d, vec3f, Vec2, Vec3};
use lib::world::Health;
use std::any::Any;
use std::mem::take;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct Player {
    action_state: ActionState,
    handle: ClientPlayerHandle,
    target: Option<ActionTarget>,
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
    pub fn new() -> (Self, ServerPlayerHandle) {
        let (handle, server_handle) = create_handles();
        (
            Self {
                action_state: ActionState::default(),
                handle,
                target: None,
                health: Health::new(100.0),
                regeneration: 3.0,
                dig_speed: 1.0,
                dig_state: None,
            },
            server_handle,
        )
    }

    fn process_input(&mut self, ctx: &mut EntityContext) {
        for msg in self.handle.input_delta.try_iter() {
            match msg {
                PlayerInputDelta::MouseMovement(Vec2 { x: dx, y: dy }) => {
                    ctx.entity
                        .body
                        .add_rotational_impulse(-dx.to_radians() as f32, -dy.to_radians() as f32);
                }
                PlayerInputDelta::MouseScroll(speed_delta) => {
                    let speed = &mut ctx.entity.body.attrs.acceleration_rate;
                    *speed += speed_delta as f64;
                    *speed = speed.max(0.0);
                }
            }
        }

        let input_state_guard = self.handle.input_state.load();
        if let Some(input_state) = input_state_guard.as_ref() {
            self.action_state = input_state.action_state;
            ctx.entity.body.motion = input_state.relative_motion;
        }

        ctx.entity.body.motion = ctx
            .entity
            .body
            .motion
            .min_each(1.0)
            .max_each(-1.0);
    }

    fn handle_interaction(&mut self, ctx: &mut EntityContext) {
        let ray_origin = ctx.entity.body().eye_position();
        let ray_dir = ctx.entity.body().rotation().into_view_center();
        let cube_hit = ctx.chunk_map.cast_ray(ray_origin, ray_dir, 100.0);

        let current_target = cube_hit
            .as_ref()
            .map(|hit| ActionTarget::Cube(CubeTarget { position: hit.position }));

        if self.target != current_target {
            self.dig_state = None;
            self.target = current_target;
        }

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
        let position = cube_hit.position + cube_hit.face.normal();
        let collider = Aabb3::new(
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

    fn sync_state(&mut self, ctx: &mut EntityContext) {
        self.handle
            .state
            .store(Some(Arc::new(PlayerState {
                position: ctx.entity.body.position,
                rotation: ctx.entity.body.rotation,
                eye_offset: ctx.entity.body.bounds.eye_offset,
                health: self.health,
                target: self.target,
                shell_opacity: self
                    .dig_state
                    .as_ref()
                    .map(|x| 0.5 - (x.remaining_time * self.dig_speed) / x.material.toughness * 0.5)
                    .unwrap_or(0.0),
            })));
    }
}

impl EntityBehavior for Player {
    fn update(&mut self, ctx: &mut EntityContext) {
        self.health += self.regeneration * ctx.dt.as_secs_f32();

        if let Some(fall_distance) = ctx.entity.body.last_fell.take() {
            if fall_distance > 3.0 {
                self.health -= (fall_distance - 3.0) * 2.0;
            }
        }

        self.process_input(ctx);
        self.handle_interaction(ctx);
        self.sync_state(ctx);
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

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub position: vec3d,
    pub rotation: Euler<f32>,
    pub eye_offset: vec3f,
    pub health: Health,
    pub target: Option<ActionTarget>,
    pub shell_opacity: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Euler::IDENTITY,
            eye_offset: Vec3::ZERO,
            health: Health::new(100.0),
            target: None,
            shell_opacity: 0.0,
        }
    }
}

pub enum PlayerInputDelta {
    MouseMovement(vec2d),
    MouseScroll(f32),
}

#[derive(Debug)]
pub struct PlayerInputState {
    pub relative_motion: vec3f,
    pub action_state: ActionState,
}

#[derive(Debug)]
pub struct ClientPlayerHandle {
    pub state: Arc<ArcSwapOption<PlayerState>>,
    pub input_state: Arc<ArcSwapOption<PlayerInputState>>,
    pub input_delta: Receiver<PlayerInputDelta>,
}

#[derive(Debug)]
pub struct ServerPlayerHandle {
    pub state: Arc<ArcSwapOption<PlayerState>>,
    pub input_state: Arc<ArcSwapOption<PlayerInputState>>,
    pub input_delta: Sender<PlayerInputDelta>,
}

fn create_handles() -> (ClientPlayerHandle, ServerPlayerHandle) {
    let state = Arc::new(ArcSwapOption::new(None));
    let input_state = Arc::new(ArcSwapOption::new(None));
    let (input_delta_tx, input_delta_rx) = crossbeam_channel::bounded(16);

    (
        ClientPlayerHandle {
            state: state.clone(),
            input_state: input_state.clone(),
            input_delta: input_delta_rx,
        },
        ServerPlayerHandle {
            state,
            input_state,
            input_delta: input_delta_tx,
        },
    )
}
