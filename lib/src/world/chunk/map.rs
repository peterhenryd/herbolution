use hashbrown::HashMap;
use math::vector::vec3i;
use crate::engine::geometry::cube::Faces;
use crate::engine::gpu::Gpu;
use crate::world::chunk::{Chunk, InstanceMesh};
use crate::world::chunk::generator::ChunkGenerator;
use crate::world::chunk::material::Material;
use crate::world::position::{ChunkLocalPosition, CubePosition};

#[derive(Debug)]
pub struct ChunkMap {
    gpu: Gpu,
    map: HashMap<vec3i, Chunk<InstanceMesh>>,
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

    pub fn get_chunk(&self, position: vec3i) -> Option<&Chunk<InstanceMesh>> {
        self.map.get(&position)
    }

    pub fn get_chunk_mut(&mut self, position: vec3i) -> Option<&mut Chunk<InstanceMesh>> {
        self.map.get_mut(&position)
    }

    pub fn chunk(&mut self, position: vec3i) -> &mut Chunk<InstanceMesh> {
        if !self.map.contains_key(&position) {
            self.load_chunk(position);
        }

        self.map.get_mut(&position).unwrap()
    }

    pub fn load_chunk(&mut self, position: vec3i) {
        if self.map.contains_key(&position) {
            return;
        }

        let mut chunk = Chunk::new(position);
        self.generator.generate(&mut chunk);

        for p in Faces::all().map(|x| x.into_vec3i()) {
            let Some(other) = self.get_chunk_mut(position + p) else { continue };
            chunk.cull_shared_faces(other);
        }

        let chunk = chunk.into_meshed(&self.gpu);

        self.map.insert(position, chunk);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk<InstanceMesh>> {
        self.map.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Chunk<InstanceMesh>> {
        self.map.values_mut()
    }

    pub fn set_cube(&mut self, position: impl Into<CubePosition>, material: Material) {
        let position = ChunkLocalPosition::from(position.into());
        self.chunk(position.chunk).set(position.local, material);
    }
}