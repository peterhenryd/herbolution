use crate::world::chunk;
use crate::world::chunk::material::Material;
use math::vector::{vec2i, vec3i, vec3u5};
use std::ops::Mul;
use std::sync::Arc;
use cached::proc_macro::cached;
use crossbeam::channel::{bounded, Receiver, Sender, TryIter};
use pollster::FutureExt;
use rayon::{ThreadPool, ThreadPoolBuilder};
use simdnoise::NoiseBuilder;
use crate::world::chunk::material;
use crate::world::chunk::mesh::Mesh;

#[derive(Debug)]
pub struct Channel {
    thread_pool: ThreadPool,
    sender: Sender<Mesh>,
    receiver: Receiver<Mesh>,
    generator: Arc<Generator>,
    materials: material::Registry,
}

impl Channel {
    pub fn new(generator: Arc<Generator>, materials: material::Registry) -> Self {
        let (sender, receiver) = bounded(64);

        Self {
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap(),
            sender,
            receiver,
            generator,
            materials,
        }
    }

    pub fn request_chunk(&self, position: vec3i) {
        let sender = self.sender.clone();
        let generator = self.generator.clone();
        let materials = self.materials.clone();

        self.thread_pool.spawn(move || {
            let mut mesh = Mesh::new(position);

            if let Err(e) = generator.generate(&mut mesh, &materials).block_on() {
                eprintln!("Failed to generate chunk at {position}: {e:?}");
                return;
            }

            sender.send(mesh).unwrap();
        });
    }

    pub fn dequeue(&self) -> TryIter<'_, Mesh> {
        self.receiver.try_iter()
    }
}

#[derive(Debug)]
pub struct Generator {
    seed: i32,
}

impl Generator {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 96;

    pub fn new(seed: i32) -> Self {
        Self { seed }
    }

    pub async fn generate(&self, mesh: &mut Mesh, materials: &material::Registry) -> Result<(), Error> {
        let Some(stone) = materials.get("stone").await else { return Err(Error::MissingMaterial) };
        let Some(dirt) = materials.get("dirt").await else { return Err(Error::MissingMaterial) };
        let Some(grass) = materials.get("grass").await else { return Err(Error::MissingMaterial) };

        let noise = generate_noise(mesh.position.xz(), self.seed);
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = noise[x + z * chunk::LENGTH];
                let h =
                    Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = mesh.position.y * chunk::LENGTH as i32 + chunk_y as i32;
                    if y < h - 6 {
                        mesh.set(
                            vec3u5::new(x as u8, chunk_y as u8, z as u8),
                            Some(&stone),
                        );
                    } else if y < h - 1 {
                        mesh.set(vec3u5::new(x as u8, chunk_y as u8, z as u8), Some(&dirt));
                    } else if y < h {
                        mesh.set(
                            vec3u5::new(x as u8, chunk_y as u8, z as u8),
                            Some(&grass),
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    MissingMaterial,
}

#[cached]
fn generate_noise(position: vec2i, seed: i32) -> Vec<f32> {
    let offset = position.mul(chunk::LENGTH as i32).cast().unwrap();
    let noise_type = NoiseBuilder::fbm_2d_offset(offset.x, chunk::LENGTH, offset.y, chunk::LENGTH)
        .with_seed(seed)
        .with_freq(0.1)
        .wrap();
    unsafe { simdnoise::sse2::get_2d_noise(&noise_type).0 }
}
