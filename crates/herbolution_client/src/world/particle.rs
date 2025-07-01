use crate::app::Update;
use engine::sculptor::{Chisel, GrowBuffer3d, Instance3d, Instance3dPayload};
use game::handle::{GameHandle, Particle};
use gpu::buffer::Usage;
use gpu::{GrowBuffer, Handle};
use lib::default;
use math::rotation::Quat;
use std::time::Duration;

#[derive(Debug)]
pub struct Particles {
    vec: Vec<Particle>,
    buffer: GrowBuffer3d,
    instances: Vec<Instance3dPayload>,
}

impl Particles {
    pub fn create(gpu: &Handle) -> Self {
        Self {
            vec: Vec::new(),
            buffer: GrowBuffer::empty(gpu, Usage::VERTEX | Usage::COPY_DST),
            instances: Vec::new(),
        }
    }

    pub fn update(&mut self, handle: &GameHandle, ctx: &mut Update) {
        self.instances.clear();
        for particle in &mut self.vec {
            let instance = update_particle(particle, ctx);
            self.instances.push(instance);
        }
        self.buffer
            .write(&ctx.engine.video.handle, &self.instances);

        self.vec
            .retain(|particle| particle.lifetime > Duration::ZERO);

        self.vec.extend(handle.particle_rx.try_iter())
    }

    pub fn render(&self, chisel: &mut Chisel) {
        chisel.render_each(&self.buffer);
    }
}

pub fn update_particle(particle: &mut Particle, ctx: &mut Update) -> Instance3dPayload {
    particle.lifetime = particle.lifetime.saturating_sub(ctx.dt);

    particle
        .motile
        .simulate(&mut particle.position, ctx.dt);

    Instance3d {
        position: particle.position,
        rotation: Quat::IDENTITY,
        color: particle.color,
        ..default()
    }
    .payload()
}
