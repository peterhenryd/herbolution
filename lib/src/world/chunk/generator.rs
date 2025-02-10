use cached::proc_macro::cached;
use crate::world::chunk;
use crate::world::chunk::{Chunk, ChunkMesh};
use math::vector::{vec2f, vec2i, vec3u8};
use simdnoise::NoiseBuilder;
use crate::world::chunk::material::Material;

#[derive(Debug)]
pub struct ChunkGenerator {
    seed: i32,
}

impl ChunkGenerator {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 80;

    pub fn new(seed: i32) -> Self {
        Self { seed }
    }

    pub fn generate<A: ChunkMesh>(&self, chunk: &mut Chunk<A>) {
        let noise = get_noise(self.seed, chunk.position.xz());
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = noise[x + chunk::LENGTH * z];
                let h = Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.position.y * chunk::LENGTH as i32 + chunk_y as i32;
                    if y < h - 6 {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), Material::Stone);
                    } else if y < h - 1 {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), Material::Dirt);
                    } else if y < h {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), Material::Grass);
                    }
                }
            }
        }
    }
}

#[cached]
fn get_noise(seed: i32, position: vec2i) -> Vec<f32> {
    let vec2f { x, y } = position.cast();
    let (x, y, l) = (x * chunk::LENGTH as f32, y * chunk::LENGTH as f32, chunk::LENGTH);
    NoiseBuilder::ridge_2d_offset(x, l, y, l)
        .with_octaves(2)
        .with_freq(0.005)
        .with_lacunarity(2.0)
        .with_gain(0.5)
        .with_seed(seed)
        .generate_scaled(0.0, 1.0)
}