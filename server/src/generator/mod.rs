use std::array;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender, TryIter, unbounded};
use lib::chunk;
use lib::point::ChunkPt;
use lib::vector::{Vec2, vec2d, vec3u5};
use noise::{Billow, NoiseFn, PerlinSurflet, ScaleBias, Simplex, Turbulence};

use crate::chunk::material::Material;
use crate::chunk::mesh::CubeMesh;

#[derive(Debug)]
pub struct ChunkGenerator {
    sender: Sender<CubeMesh>,
    receiver: Receiver<CubeMesh>,
    params: Arc<GenerationParams>,
}

impl ChunkGenerator {
    pub fn new(generator: Arc<GenerationParams>) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            sender,
            receiver,
            params: generator,
        }
    }

    pub fn request(&self, position: ChunkPt) {
        let sender = self.sender.clone();
        let params = self.params.clone();

        rayon::spawn(move || {
            let mut mesh = CubeMesh::new(position);

            params.generate(&mut mesh);
            sender.send(mesh).unwrap();
        });
    }

    pub fn dequeue(&self) -> TryIter<'_, CubeMesh> {
        self.receiver.try_iter()
    }
}

struct Octave {
    noise: ScaleBias<f64, Turbulence<Simplex, Billow<PerlinSurflet>>, 2>,
    scale: f64,
}

impl Octave {
    fn get(&self, position: vec2d) -> f64 {
        self.noise.get((position / self.scale).into())
    }
}

pub struct GenerationParams {
    octaves: [Octave; 8],
}

impl Debug for GenerationParams {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl GenerationParams {
    pub fn new(seed: i64) -> Self {
        Self {
            octaves: array::from_fn(|i| Octave {
                noise: ScaleBias::new(
                    Turbulence::new(Simplex::new(seed as u32))
                        .set_frequency(0.01 * (i as f64 + 1.0))
                        .set_power(2.0)
                        .set_roughness(4),
                )
                .set_scale((i as f64).powf(2.0 - (8 - i) as f64 / 8.0)),
                scale: i as f64 + 1.0,
            }),
        }
    }

    pub fn generate(&self, chunk: &mut CubeMesh) {
        // TODO: reuse materials
        let stone = chunk.palette.insert(Arc::new(Material::stone()));
        let dirt = chunk.palette.insert(Arc::new(Material::dirt()));
        let grass = chunk.palette.insert(Arc::new(Material::grass()));

        let chunk_position = chunk.position.0.xz().cast::<f64>();
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let position = Vec2::new(x as f64, z as f64) / chunk::LENGTH as f64 + chunk_position;
                let h = self.sample_noise(position) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.position.0.y * chunk::LENGTH as i32 + chunk_y as i32;
                    if y < h - 6 {
                        chunk.set(vec3u5::new(x as u8, chunk_y as u8, z as u8), Some(stone));
                    } else if y < h - 1 {
                        chunk.set(vec3u5::new(x as u8, chunk_y as u8, z as u8), Some(dirt));
                    } else if y < h {
                        chunk.set(vec3u5::new(x as u8, chunk_y as u8, z as u8), Some(grass));
                    }
                }
            }
        }
    }

    fn sample_noise(&self, position: vec2d) -> f64 {
        self.octaves
            .iter()
            .fold(0.0, |acc, noise| acc + noise.get(position.into()))
    }
}
