use std::time::Duration;

use lib::rotation::Quat;
use lib::vector::{vec3d, Vec3, Vec4};
use server::handle::{GameHandle, Particle};
use wgpu::BufferUsages;

use crate::app::Update;
use crate::video::gpu;
use crate::video::resource::GrowBuffer;
use crate::video::world::chisel::Chisel;
use crate::video::world::Instance3d;

#[derive(Debug)]
pub struct Particles {
    vec: Vec<Particle>,
    buffer: GrowBuffer<Instance3d>,
    instances: Vec<Instance3d>,
}

impl Particles {
    pub fn create(gpu: &gpu::Handle) -> Self {
        Self {
            vec: Vec::new(),
            buffer: GrowBuffer::empty(gpu, BufferUsages::VERTEX | BufferUsages::COPY_DST),
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
            .write(&ctx.video.handle, &self.instances);

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
        Vec4::ZERO,
    )
}
