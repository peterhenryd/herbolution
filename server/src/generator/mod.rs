use std::fmt::Debug;
use std::sync::Arc;

use crossbeam_channel::{unbounded, Receiver, Sender, TryIter};
use lib::point::ChunkPt;
use lib::util::ProgressiveMeasurement;
use lib::vector::{vec2f, vec3u5};
use lib::world::{CHUNK_AREA, CHUNK_LENGTH};
use simd_noise::noise::{FbmNoise, Noise, NoiseDim, NoiseTransform, OctaveNoise};

use crate::chunk::material::Palette;
use crate::chunk::mesh::CubeMesh;

pub static CHUNK_GENERATION_TIME: ProgressiveMeasurement = ProgressiveMeasurement::new();

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

#[derive(Debug)]
pub struct GenerationParams {
    seed: i64,
    global_palette: Arc<Palette>,
}

impl GenerationParams {
    pub fn new(seed: i64, global_palette: Arc<Palette>) -> Self {
        Self { seed, global_palette }
    }

    pub fn generate(&self, chunk: &mut CubeMesh) {
        let stopwatch = CHUNK_GENERATION_TIME.start_measuring();

        let stone = chunk
            .palette
            .insert(self.global_palette.get("herbolution:stone"));
        let dirt = chunk
            .palette
            .insert(self.global_palette.get("herbolution:dirt"));
        let grass = chunk
            .palette
            .insert(self.global_palette.get("herbolution:grass"));

        let chunk_position = chunk.position.0.xz().cast() * CHUNK_LENGTH as f32;

        let noise = self.get_noise(chunk_position);
        for x in 0..CHUNK_LENGTH {
            for z in 0..CHUNK_LENGTH {
                let h = (noise[x + z * CHUNK_LENGTH] * CHUNK_LENGTH as f32) as i32;

                for chunk_y in 0..CHUNK_LENGTH {
                    let y = chunk.position.0.y * CHUNK_LENGTH as i32 + chunk_y as i32;
                    let position = vec3u5::new(x as u8, chunk_y as u8, z as u8);

                    if y < h - 6 {
                        chunk.set(position, Some(stone));
                    } else if y < h - 1 {
                        chunk.set(position, Some(dirt));
                    } else if y < h {
                        chunk.set(position, Some(grass));
                    }
                }
            }
        }

        stopwatch.stop();
    }

    #[inline]
    fn get_noise(&self, position: vec2f) -> [f32; CHUNK_AREA] {
        let transform: NoiseTransform<{ NoiseDim::new_2d(CHUNK_LENGTH, CHUNK_LENGTH) }> = NoiseTransform::from_seed(self.seed)
            .with_x(position.x)
            .with_y(position.y);

        FbmNoise::from(transform)
            .with_seed(self.seed)
            .with_freq([0.001; 2])
            .with_octaves(6)
            .with_lacunarity(2.0)
            .with_gain(0.6)
            .generate()
            .0
    }
}
