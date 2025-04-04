use crate::world::chunk;
use crate::world::chunk::material::Material;
use crate::world::chunk::Chunk;
use math::vector::{vec2i, vec3u5};
use std::ops::Mul;
use hashbrown::HashMap;
use simdnoise::NoiseBuilder;

#[derive(Debug)]
pub struct ChunkGenerator {
    seed: i32,
    noise_cache: HashMap<vec2i, Vec<f32>>,
}

impl ChunkGenerator {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 96;

    pub fn new(seed: i32) -> Self {
        Self { seed, noise_cache: HashMap::new() }
    }

    pub fn generate(&mut self, chunk: &mut Chunk) {
        let noise = self.get_noise(chunk.position.xz());
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = noise[x + z * chunk::LENGTH];
                let h =
                    Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.position.y * chunk::LENGTH as i32 + chunk_y as i32;
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

    fn get_noise(&mut self, position: vec2i) -> &[f32] {
        if !self.noise_cache.contains_key(&position) {
            self.noise_cache.insert(position, generate_noise(position, self.seed));
        }

        self.noise_cache.get(&position).unwrap()
    }
}

fn generate_noise(position: vec2i, seed: i32) -> Vec<f32> {
    let offset = position.mul(chunk::LENGTH as i32).cast().unwrap();
    let noise_type = NoiseBuilder::fbm_2d_offset(offset.x, chunk::LENGTH, offset.y, chunk::LENGTH)
        .with_seed(seed)
        .with_freq(0.1)
        .wrap();
    unsafe { simdnoise::sse2::get_2d_noise(&noise_type).0 }
}
