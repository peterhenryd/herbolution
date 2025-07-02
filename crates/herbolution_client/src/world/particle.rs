use crate::app::Update;
use engine::sculptor::{Chisel, GrowBuffer3d, Instance3d};
use game::handle::{GameHandle, Particle};
use gpu::{BufferUsage, GrowBuffer, Handle};
use math::rotation::Quat;
use math::vector::{vec3d, Vec3};
use std::time::Duration;

#[derive(Debug)]
pub struct Particles {
    vec: Vec<Particle>,
    buffer: GrowBuffer3d,
    instances: Vec<Instance3d>,
}

impl Particles {
    pub fn create(gpu: &Handle) -> Self {
        Self {
            vec: Vec::new(),
            buffer: GrowBuffer::empty(gpu, BufferUsage::VERTEX | BufferUsage::COPY_DST),
            instances: Vec::new(),
        }
    }

    pub fn update(&mut self, handle: &GameHandle, ctx: &mut Update, camera_position: vec3d) {
        self.instances.clear();
        for particle in &mut self.vec {
            let instance = update_particle(particle, ctx, camera_position);
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

pub fn update_particle(particle: &mut Particle, ctx: &mut Update, mut camera_position: vec3d) -> Instance3d {
    particle.lifetime = particle.lifetime.saturating_sub(ctx.dt);

    particle
        .motile
        .simulate(&mut particle.position, ctx.dt);

    Instance3d::new(
        particle.position,
        particle.rotation.unwrap_or_else(|| {
            camera_position.y = particle.position.y;
            let dir = (particle.position - camera_position)
                .normalize()
                .cast();
            Quat::look_to(dir, -Vec3::Y)
        }),
        Vec3::splat(0.1),
        particle.color,
        1,
    )
}
