use crate::engine::geometry::cuboid::{Cuboid, Faces};
use crate::engine::gpu::Gpu;
use crate::world::chunk::generator::ChunkGenerator;
use crate::world::chunk::material::Material;
use crate::world::chunk::{Chunk, ChunkMesh, InstanceMesh};
use crate::world::position::{ChunkLocalPosition, CubePosition};
use hashbrown::HashMap;
use math::vector::{vec3, vec3d, vec3f, vec3i};

#[derive(Debug)]
pub struct ChunkMap {
    gpu: Gpu,
    map: HashMap<vec3i, Chunk<InstanceMesh>>,
    generator: ChunkGenerator,
}

impl ChunkMap {
    pub fn new(gpu: Gpu, seed: i32) -> Self {
        Self {
            gpu,
            map: HashMap::new(),
            generator: ChunkGenerator::new(seed),
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
            other.cull_shared_faces(&mut chunk);
        }

        self.map.insert(position, chunk.into_meshed(&self.gpu));
    }

    pub fn iter(&self) -> impl Iterator<Item=&Chunk<InstanceMesh>> {
        self.map.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Chunk<InstanceMesh>> {
        self.map.values_mut()
    }

    pub fn set_cube(&mut self, position: impl Into<CubePosition>, material: Material) {
        let position = ChunkLocalPosition::from(position.into());
        self.chunk(position.chunk).set(position.local, material);

        if position.local.x == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(-1, 0, 0));
            let index = chunk.get_index(vec3::new(31, position.local.y, position.local.z));
            chunk.data[index].faces.insert(Faces::RIGHT);
            chunk.mesh.schedule_update();
        } else if position.local.x == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(1, 0, 0));
            let index = chunk.get_index(vec3::new(0, position.local.y, position.local.z));
            chunk.data[index].faces.insert(Faces::LEFT);
            chunk.mesh.schedule_update();
        } else if position.local.y == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, -1, 0));
            let index = chunk.get_index(vec3::new(position.local.x, 31, position.local.z));
            chunk.data[index].faces.insert(Faces::TOP);
            chunk.mesh.schedule_update();
        } else if position.local.y == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 1, 0));
            let index = chunk.get_index(vec3::new(position.local.x, 0, position.local.z));
            chunk.data[index].faces.insert(Faces::BOTTOM);
            chunk.mesh.schedule_update();
        } else if position.local.z == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 0, -1));
            let index = chunk.get_index(vec3::new(position.local.x, position.local.y, 31));
            chunk.data[index].faces.insert(Faces::FRONT);
            chunk.mesh.schedule_update();
        } else if position.local.z == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 0, 1));
            let index = chunk.get_index(vec3::new(position.local.x, position.local.y, 0));
            chunk.data[index].faces.insert(Faces::BACK);
            chunk.mesh.schedule_update();
        }
    }

    pub fn get_cube(&mut self, position: impl Into<CubePosition>) -> Option<Material> {
        let position = ChunkLocalPosition::from(position.into());
        self.chunk(position.chunk).get(position.local)
    }

    pub fn cast_ray(&mut self, origin: vec3f, direction: vec3d) -> Option<vec3i> {
        let mut position = origin;
        let step = direction.normalize().cast() * 0.1;
        let mut distance = 0.0;

        while distance < 10.0 {
            let x = position.x.floor() as i32;
            let y = position.y.floor() as i32;
            let z = position.z.floor() as i32;

            if let Some(_) = self.get_cube(CubePosition(vec3::new(x, y, z))) {
                return Some(vec3::new(x, y, z).cast());
            }

            position += step;
            distance += step.length();
        }

        None
    }

    pub fn check_collision(&mut self, cuboid: Cuboid<f32>) -> bool {
        let min = vec3::new(
            cuboid.min.x.floor(),
            cuboid.min.y.floor(),
            cuboid.min.z.floor(),
        );
        let max = vec3::new(
            cuboid.max.x.ceil(),
            cuboid.max.y.ceil(),
            cuboid.max.z.ceil(),
        );

        for x in min.x as i32..max.x as i32 {
            for y in min.y as i32..max.y as i32 {
                for z in min.z as i32..max.z as i32 {
                    if let Some(material) =
                        self.get_cube(CubePosition(vec3::new(x, y, z)))
                    {
                        if material.can_collide() {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}