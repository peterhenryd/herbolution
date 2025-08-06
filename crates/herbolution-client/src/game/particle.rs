use std::time::Duration;

use prims::rotation::Quat;
use prims::size::Size3;
use prims::vector::{Vec3, vec3d};
use server::handle::{GameHandle, Particle};
use wgpu::BufferUsages;

use crate::app::Update;

#[derive(Debug)]
pub struct Particles {
    vec: Vec<Particle>,
    buffer: kiln::GrowBuffer<kiln::Instance3d>,
    instances: Vec<kiln::Instance3d>,
}

impl Particles {
    pub fn create() -> Self {
        Self {
            vec: Vec::new(),
            buffer: kiln::GrowBuffer::empty(BufferUsages::VERTEX | BufferUsages::COPY_DST),
            instances: Vec::new(),
        }
    }

    pub fn update(&mut self, handle: &GameHandle, ctx: &mut Update, camera_position: vec3d) {
        self.instances.clear();
        for particle in &mut self.vec {
            let instance = update_particle(particle, ctx, camera_position);
            self.instances.push(instance);
        }
        self.buffer.write(ctx.app.gpu, &self.instances);

        self.vec
            .retain(|particle| particle.lifetime > Duration::ZERO);

        self.vec.extend(handle.particle_rx.try_iter())
    }

    pub fn render(&self, frame: &mut kiln::Draw3d<'_, '_>) {
        frame.render(&self.buffer);
    }
}

pub fn update_particle(particle: &mut Particle, ctx: &mut Update, mut camera_position: vec3d) -> kiln::Instance3d {
    particle.lifetime -= ctx.dt;

    particle
        .motile
        .simulate(&mut particle.position, ctx.dt);

    kiln::Instance3d::new(
        particle.position.cast(),
        particle.rotation.unwrap_or_else(|| {
            camera_position.y = particle.position.y;
            let dir = (particle.position - camera_position)
                .normalize()
                .cast();
            Quat::look_to(dir, -Vec3::Y)
        }),
        Size3::splat(0.1),
        particle.color,
        None,
    )
}
