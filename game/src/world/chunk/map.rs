use crate::world::chunk::cube::CubePosition;
use crate::world::chunk::generator::ChunkGenerator;
use crate::world::chunk::material::Material;
use crate::world::chunk::{linearize, Chunk, ChunkLocalPosition};
use crate::Response;
use lib::geometry::cuboid::face::{Face, Faces};
use lib::geometry::cuboid::Cuboid;
use math::vector::{vec3f, vec3i, vec3u5, Vec3};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub struct ChunkMap {
    map: HashMap<vec3i, Chunk>,
    generator: ChunkGenerator,
    sender: Sender<Response>,
}

impl ChunkMap {
    pub fn new(seed: i32, sender: Sender<Response>) -> Self {
        Self {
            map: HashMap::new(),
            generator: ChunkGenerator::new(seed),
            sender,
        }
    }

    pub fn get_near_colliders(&mut self, cuboid: Cuboid<f32>) -> Vec<Cuboid<f32>> {
        let min = cuboid.min.floor() - cuboid.min.signum();
        let max = cuboid.max.ceil() + cuboid.max.signum();

        let mut colliders = Vec::new();
        for x in min.x as i32..max.x as i32 {
            for y in min.y as i32..max.y as i32 {
                for z in min.z as i32..max.z as i32 {
                    if let Some(material) = self.get_cube(CubePosition(Vec3::new(x, y, z))) {
                        if material.can_collide() {
                            colliders.push(Cuboid::new(
                                Vec3::new(x as f32 - 0.5, y as f32 - 0.5, z as f32 - 0.5),
                                Vec3::new(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5),
                            ));
                        }
                    }
                }
            }
        }

        colliders
    }

    pub fn get_chunk(&self, position: vec3i) -> Option<&Chunk> {
        self.map.get(&position)
    }

    pub fn get_chunk_mut(&mut self, position: vec3i) -> Option<&mut Chunk> {
        self.map.get_mut(&position)
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

        let (sender, receiver) = channel(16);
        let mut chunk = Chunk::new(position, sender);
        if let Err(e) = self.sender.try_send(Response::LoadChunk { position, receiver }) {
            eprintln!("Failed to send load chunk request: {:?}", e);
        }
        self.generator.generate(&mut chunk);

        for p in Faces::all().map(Face::into_vec3) {
            let Some(other) = self.get_chunk_mut(position + p) else {
                continue;
            };
            chunk.cull_shared_faces(other);
        }

        self.map.insert(position, chunk);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> {
        self.map.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
        self.map.values_mut()
    }

    pub fn set_cube(&mut self, position: impl Into<CubePosition>, material: Option<Material>) {
        let position = ChunkLocalPosition::from(position.into());

        if position.local.x() == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(-1, 0, 0));
            let position = vec3u5::new(31, position.local.y(), position.local.z());
            chunk.data[linearize(position)].insert_faces(Faces::RIGHT);
            chunk.dirtied_positions.push(position);
        } else if position.local.x() == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(1, 0, 0));
            let position = vec3u5::new(0, position.local.y(), position.local.z());
            chunk.data[linearize(position)].insert_faces(Faces::LEFT);
            chunk.dirtied_positions.push(position);
        } else if position.local.y() == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, -1, 0));
            let position = vec3u5::new(position.local.x(), 31, position.local.z());
            chunk.data[linearize(position)].insert_faces(Faces::TOP);
            chunk.dirtied_positions.push(position);
        } else if position.local.y() == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 1, 0));
            let position = vec3u5::new(position.local.x(), 0, position.local.z());
            chunk.data[linearize(position)].insert_faces(Faces::BOTTOM);
            chunk.dirtied_positions.push(position);
        } else if position.local.z() == 0 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 0, -1));
            let position = vec3u5::new(position.local.x(), position.local.y(), 31);
            chunk.data[linearize(position)].insert_faces(Faces::FRONT);
            chunk.dirtied_positions.push(position);
        } else if position.local.z() == 31 {
            let chunk = self.chunk(position.chunk + vec3i::new(0, 0, 1));
            let position = vec3u5::new(position.local.x(), position.local.y(), 0);
            chunk.data[linearize(position)].insert_faces(Faces::BACK);
            chunk.dirtied_positions.push(position);
        }

        self.chunk(position.chunk).set(position.local, material);
    }

    pub fn get_cube(&mut self, position: impl Into<CubePosition>) -> Option<Material> {
        let position = ChunkLocalPosition::from(position.into());
        self.chunk(position.chunk).get(position.local)
    }

    pub fn cast_ray(&mut self, origin: vec3f, direction: vec3f) -> Option<vec3i> {
        let mut position = origin;
        let step = direction.normalize() * 0.01;
        let dist_step = step.length();
        let mut distance: f32 = 0.0;

        while distance < 10.0 {
            distance += dist_step;
            position += step;

            let Some(cube) = self.get_cube(position.cast().unwrap()) else { continue };

            if cube.can_collide() {
                return Some(position.cast().unwrap());
            }
        }

        None
    }

    pub fn tick(&mut self) {
        for chunk in self.map.values_mut() {
            chunk.tick();
        }
    }
}