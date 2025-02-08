use hashbrown::HashMap;
use math::vector::vec3i;
use crate::engine::gpu::Gpu;
use crate::world::chunk::Chunk;
use crate::world::chunk::generator::ChunkGenerator;

#[derive(Debug)]
pub struct ChunkMap {
    gpu: Gpu,
    map: HashMap<vec3i, Chunk>,
    generator: ChunkGenerator
}

impl ChunkMap {
    pub fn new(gpu: Gpu, seed: i32) -> Self {
        Self {
            gpu,
            map: HashMap::new(),
            generator: ChunkGenerator::new(seed)
        }
    }

    pub fn get_chunk(&self, position: vec3i) -> Option<&Chunk> {
        self.map.get(&position)
    }

    pub fn chunk(&mut self, position: vec3i) -> &mut Chunk {
        if !self.map.contains_key(&position) {
            self.load_chunk(position);
        }

        self.map.get_mut(&position).unwrap()
    }

    pub fn load_chunk(&mut self, position: vec3i) {
        if self.map.contains_key(&position) {
            return;
        }

        let mut chunk = Chunk::new(&self.gpu, position);
        self.generator.generate(&mut chunk);

        self.map.insert(position, chunk);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> {
        self.map.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
        self.map.values_mut()
    }
}