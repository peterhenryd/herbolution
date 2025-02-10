use crate::world::chunk;
use crate::world::chunk::material::Material;
use crate::world::chunk::{Chunk, ChunkMesh};
use cached::proc_macro::cached;
use math::vector::{vec2, vec2i, vec3u8};
use std::hash::Hash;
use noise::{NoiseFn, PerlinSurflet, Seedable};

#[derive(Debug)]
pub struct ChunkGenerator {
    seed: u32,
    noise: PerlinSurflet,
}

impl ChunkGenerator {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 80;

    pub fn new(seed: u32) -> Self {
        Self { seed, noise: PerlinSurflet::new(seed) }
    }

    pub fn generate<A: ChunkMesh>(&self, chunk: &mut Chunk<A>) {
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let abs_x = chunk.position.x * chunk::LENGTH as i32 + x as i32;
                let abs_z = chunk.position.z * chunk::LENGTH as i32 + z as i32;
                let f = get_noise(Noise(self.noise), vec2::new(abs_x, abs_z));
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

// Wrapper to allow for caching
pub struct Noise(PerlinSurflet);

impl Clone for Noise {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for Noise {}

impl PartialEq for Noise {
    fn eq(&self, other: &Self) -> bool {
        self.0.seed() == other.0.seed()
    }
}

impl Hash for Noise {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.seed().hash(state);
    }
}


#[cached]
fn get_noise(noise: Noise, position: vec2i) -> f32 {
    noise.0.get([position.x as f64 / 32.0, position.y as f64 / 32.0]) as f32
}