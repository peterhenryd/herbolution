use crate::world::chunk;
use crate::world::chunk::Chunk;
use hashbrown::HashMap;
use math::vector::{vec2f, vec2i, vec3u8};
use simdnoise::NoiseBuilder;

#[derive(Debug)]
pub struct ChunkGenerator {
    seed: i32,
    chunk_noise_map: HashMap<vec2i, Vec<f32>>,
}

impl ChunkGenerator {
    const MIN_HEIGHT: i32 = 64;
    const MAX_HEIGHT: i32 = 96;

    pub fn new(seed: i32) -> Self {
        Self { seed, chunk_noise_map: HashMap::new() }
    }

    fn get_noise(&mut self, position: vec2i) -> &[f32] {
        let l = chunk::LENGTH;
        if !self.chunk_noise_map.contains_key(&position) {
            let vec2f { x, y } = position.cast();
            let noise = NoiseBuilder::ridge_2d_offset(x * 32.0, l, y * 32.0, l)
                .with_octaves(4)
                .with_freq(0.01)
                .with_lacunarity(2.0)
                .with_gain(0.5)
                .with_seed(self.seed)
                .generate_scaled(0.0, 1.0);
            self.chunk_noise_map.insert(position, noise);
        }

        self.chunk_noise_map.get(&position).unwrap().as_slice()
    }

    pub fn generate(&mut self, chunk: &mut Chunk) {
        let noise = self.get_noise(chunk.position.xz());
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                let f = noise[z * chunk::LENGTH + x];
                let h = Self::MIN_HEIGHT + (f * (Self::MAX_HEIGHT - Self::MIN_HEIGHT) as f32) as i32;

                for chunk_y in 0..chunk::LENGTH {
                    let y = chunk.position.y * 32 + chunk_y as i32;
                    if y < h - 6 {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), chunk::Material::Stone);
                    } else if y < h - 1 {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), chunk::Material::Dirt);
                    } else if y < h {
                        chunk.set(vec3u8::new(x as u8, chunk_y as u8, z as u8), chunk::Material::Grass);
                    }
                }
            }
        }
    }
}