use crate::chunk;
use crate::chunk::material::Material;
use crate::chunk::CubeMesh;
use crossbeam::channel::{unbounded, Receiver, Sender, TryIter};
use math::vector::{vec3i, vec3u4};
use noise::{NoiseFn, Simplex};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;

#[derive(Debug)]
pub struct ChunkGenerator {
    thread_pool: ThreadPool,
    sender: Sender<CubeMesh>,
    receiver: Receiver<CubeMesh>,
    params: Arc<GenerationParams>,
}

impl ChunkGenerator {
    pub fn new(generator: Arc<GenerationParams>) -> Self {
        let (sender, receiver) = unbounded();
        
        Self {
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .build()
                .unwrap(),
            sender,
            receiver,
            params: generator,
        }
    }

    pub fn request(&self, pos: vec3i) {
        let sender = self.sender.clone();
        let params = self.params.clone();

        self.thread_pool.spawn(move || {
            let mut mesh = CubeMesh::new(pos);
            
            params.generate(&mut mesh);
            sender.send(mesh).unwrap();
        });
    }

    pub fn dequeue(&self) -> TryIter<'_, CubeMesh> {
        self.receiver.try_iter()
    }
}

#[derive(Debug)]
pub struct GenerationParams {
    noise: Simplex,
}

impl GenerationParams {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 112;

    pub fn new(seed: i64) -> Self {
        Self {
            noise: Simplex::new(seed as u32)
        }
    }

    pub fn generate(&self, chunk: &mut CubeMesh) {
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = {
                    let x = chunk.pos.x * chunk::LENGTH as i32 + x as i32;
                    let z = chunk.pos.z * chunk::LENGTH as i32 + z as i32;
                    self.noise.get([x as f64 / 32.0, z as f64 / 32.0]) as f32 / 4.0
                };
                let h =
                    Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.pos.y * chunk::LENGTH as i32 + chunk_y as i32;
                    if y < h - 6 {
                        chunk.set(
                            vec3u4::new(x as u8, chunk_y as u8, z as u8),
                            Some(Material::Stone),
                        );
                    } else if y < h - 1 {
                        chunk.set(vec3u4::new(x as u8, chunk_y as u8, z as u8), Some(Material::Dirt));
                    } else if y < h {
                        chunk.set(
                            vec3u4::new(x as u8, chunk_y as u8, z as u8),
                            Some(Material::Grass),
                        );
                    }
                }
            }
        }
    }
}