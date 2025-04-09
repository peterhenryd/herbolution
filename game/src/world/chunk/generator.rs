use crate::world::chunk;
use crate::world::chunk::material::Material;
use crate::world::chunk::Mesh;
use cached::proc_macro::cached;
use crossbeam::channel::{bounded, Receiver, Sender, TryIter};
use math::vector::{vec2i, vec3i, vec3u5};
use rayon::{ThreadPool, ThreadPoolBuilder};
use simdnoise::NoiseBuilder;
use std::ops::Mul;
use std::sync::Arc;


#[derive(Debug)]
pub struct Channel {
    thread_pool: ThreadPool,
    sender: Sender<Mesh>,
    receiver: Receiver<Mesh>,
    generator: Arc<Generator>,
}

impl Channel {
    pub fn new(generator: Arc<Generator>) -> Self {
        let (sender, receiver) = bounded(64);

        Self {
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(8)
                .build()
                .unwrap(),
            sender,
            receiver,
            generator,
        }
    }

    pub fn request_chunk(&self, pos: vec3i) {
        let sender = self.sender.clone();
        let generator = self.generator.clone();

        self.thread_pool.spawn(move || {
            let mut mesh = Mesh::new(pos);

            generator.generate(&mut mesh);
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

    pub fn generate(&self, chunk: &mut Mesh) {
        let noise = generate_noise(chunk.pos.xz(), self.seed);
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = noise[x + z * chunk::LENGTH];
                let h =
                    Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.pos.y * chunk::LENGTH as i32 + chunk_y as i32;
                    if y < h - 6 {
                        chunk.set(
                            vec3u5::new(x as u8, chunk_y as u8, z as u8),
                            Some(Material::Stone),
                        );
                    } else if y < h - 1 {
                        chunk.set(vec3u5::new(x as u8, chunk_y as u8, z as u8), Some(Material::Dirt));
                    } else if y < h {
                        chunk.set(
                            vec3u5::new(x as u8, chunk_y as u8, z as u8),
                            Some(Material::Grass),
                        );
                    }
                }
            }
        }
    }
}

#[cached]
fn generate_noise(pos: vec2i, seed: i32) -> Vec<f32> {
    let offset = pos.mul(chunk::LENGTH as i32).cast().unwrap();
    let noise_type = NoiseBuilder::fbm_2d_offset(offset.x, chunk::LENGTH, offset.y, chunk::LENGTH)
        .with_seed(seed)
        .with_freq(0.1)
        .wrap();
    unsafe { simdnoise::sse2::get_2d_noise(&noise_type).0 }
}
