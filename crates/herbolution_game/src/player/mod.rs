pub mod handle;

use herbolution_lib::aabb::Aabb;
use math::vec::{Vec2, Vec3};
use std::any::Any;
use std::f32::consts::FRAC_PI_2;

use crate::entity::behavior::{EntityBehavior, EntityBehaviorType, EntityContext};
use crate::entity::{ActionState, ActionTarget};
use crate::player::handle::ClientPlayerHandle;

#[derive(Debug)]
pub struct Player {
    action_state: ActionState,
    handle: ClientPlayerHandle,
    prev_target: Option<ActionTarget>,
}

impl Player {
    pub fn new(handle: ClientPlayerHandle) -> Self {
        Self {
            action_state: ActionState::default(),
            handle,
            prev_target: None,
        }
    }

    fn update_actions(&mut self, ctx: &mut EntityContext) {
        let origin = ctx.data.body.eye_pos();
        let direction = ctx.data.body.rotation.into_view_center();
        let ray = ctx.chunk_map.cast_ray(origin, direction, 5.0);
        let position = ray.map(|(x, _)| x);
        let target = position.map(ActionTarget::Cube);

        if target != self.prev_target {
            self.handle.transform.set_target(target);
            self.prev_target = target;
        }

        let Some(position) = position else {
            return;
        };

        if self.action_state.is_left_hand_active {
            self.action_state.is_left_hand_active = false;

            ctx.chunk_map.set_cube(position, None);

            let _ = &ctx.handle.particle_tx;
            /*
            TODO
            for i in 0..3 {
                let _ = ctx.handle.particle_tx.try_send(Particle {
                    position: Vec3::new(i % 2, i / 2, i % 2)
                        .cast::<f64>()
                        .unwrap()
                        + position.cast().unwrap(),
                    motile: Motile::default(),
                    lifetime: Duration::SECOND,
                    color: Rgba::new(1.0, 0.0, 0.0, 1.0),
                });
            }
             */
        }

        if self.action_state.is_right_hand_active {
            self.action_state.is_right_hand_active = false;

            let Some((_, face)) = ray else { return };

            let position = position + face.to_normal();
            let collider = Aabb::new(
                Vec3::new(position.x as f32, position.y as f32, position.z as f32),
                Vec3::new(position.x as f32 + 1.0, position.y as f32 + 1.0, position.z as f32 + 1.0),
            );
            if !collider
                .cast()
                .unwrap()
                .intersects(&ctx.data.body.bounds())
            {
                ctx.chunk_map
                    .set_cube(position, ("herbolution", "stone"));
            }
        }
    }
}

impl EntityBehavior for Player {
    fn update(&mut self, ctx: &mut EntityContext) {
        let body = &mut ctx.data.body;

        if let Some(command) = self.handle.input.next_movement() {
            body.apply_motion_command(command);
        }

        while let Some(Vec2 { x: dx, y: dy }) = self.handle.input.next_mouse_movement() {
            // TODO: Implement mouse sensitivity
            body.rotation.yaw -= dx.to_radians() as f32;
            body.rotation.pitch -= dy.to_radians() as f32;
        }

        body.rotation.pitch = body
            .rotation
            .pitch
            .clamp(-FRAC_PI_2 + f32::EPSILON, FRAC_PI_2 - f32::EPSILON)
            .into();

        self.handle.transform.set_position(body.eye_pos());
        self.handle.transform.set_rotation(body.rotation);

        if let Some(action_state) = self.handle.input.next_action_state() {
            if self.prev_target.is_some() {
                self.action_state = action_state;
            }
        }

        self.update_actions(ctx);
    }

    fn from_mut(behavior: &mut EntityBehaviorType) -> Option<&mut Self>
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
