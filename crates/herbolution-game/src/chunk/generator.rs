use crate::game::chunk::material::Material;
use crate::game::chunk::section::CHUNK_SIZE;
use crate::game::chunk::Chunk;
use simdnoise::NoiseBuilder;

pub struct ChunkGenerator {
    seed: i32,
}

impl ChunkGenerator {
    pub fn new(seed: i32) -> Self {
        Self { seed }
    }
}

impl ChunkGenerator {
    const MIN_HEIGHT: f32 = 64.0;
    const MAX_HEIGHT: f32 = 128.0;

    pub fn generate(&self, chunk: &mut Chunk) {
        let noise = NoiseBuilder::ridge_2d_offset(
            chunk.position.x as f32,
            CHUNK_SIZE,
            chunk.position.y as f32,
            CHUNK_SIZE,
        )
            .with_octaves(4)
            .with_freq(0.01)
            .with_lacunarity(2.0)
            .with_gain(0.5)
            .with_seed(self.seed)
            .generate_scaled(0.0, 1.0);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let h_fac = noise[x * CHUNK_SIZE + z];
                let h = (Self::MIN_HEIGHT + h_fac * (Self::MAX_HEIGHT - Self::MIN_HEIGHT)) as usize;

                chunk.set_column(x, z, 0, h - 6, Material::Stone);
                chunk.set_column(x, z, h - 6, h - 1, Material::Dirt);
                chunk.set_column(x, z, h - 1, h, Material::Grass);
            }
        }
    }
}
