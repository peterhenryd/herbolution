use crate::world::chunk;
use crate::world::chunk::cube::CubePosition;
use crate::world::chunk::generator::ChunkGenerator;
use crate::world::chunk::material::Material;
use crate::world::chunk::{linearize, Chunk, ChunkLocalPosition};
use crate::Response;
use crossbeam::channel::{bounded, Sender};
use lib::geometry::cuboid::face::{Face, Faces};
use lib::geometry::cuboid::Cuboid;
use line_drawing::{VoxelOrigin, WalkVoxels};
use math::vector::{vec3f, vec3i, vec3u5, Vec3};
use std::collections::HashMap;

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

    pub fn get_near_colliders(&self, cuboid: Cuboid<f32>, colliders: &mut Vec<Cuboid<f32>>) {
        let min = cuboid.min.floor().cast().unwrap() - 1;
        let max = cuboid.max.ceil().cast().unwrap() + 1;

        colliders.clear();
        for x in min.x..max.x {
            for y in min.y..max.y {
                for z in min.z..max.z {
                    if let Some(material) = self.get_cube_material(CubePosition(Vec3::new(x, y, z))) {
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

    pub fn unload_chunk(&mut self, position: vec3i) {
        self.map.remove(&position);
        let _ = self.sender.try_send(Response::UnloadChunk { position });
    }

    pub fn load_chunk(&mut self, position: vec3i) {
        if self.map.contains_key(&position) {
            return;
        }

        let (sender, receiver) = bounded(4);
        let mut chunk = Chunk::new(position, sender);
        if let Err(e) = self.sender.try_send(Response::LoadChunk { position, receiver }) {
            eprintln!("Failed to send load chunk request: {:?}", e);
        }
        self.generator.generate(&mut chunk);

        for offset in Face::entries().map(Face::into_vec3) {
            let Some(other) = self.get_chunk_mut(position + offset) else { continue };
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
        let edges = [
            (Vec3::new(-1, 0, 0), Faces::RIGHT,  position.local.x() == 0),
            (Vec3::new(1, 0, 0),  Faces::LEFT,   position.local.x() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, -1, 0), Faces::TOP,    position.local.y() == 0),
            (Vec3::new(0, 1, 0),  Faces::BOTTOM, position.local.y() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, 0, -1), Faces::FRONT,  position.local.z() == 0),
            (Vec3::new(0, 0, 1),  Faces::BACK,   position.local.z() == chunk::LENGTH as u8 - 1),
        ];

        for (offset, face, condition) in edges {
            if !condition {
                continue;
            }

            let neighbor_chunk_coord = position.chunk + offset;
            let neighbor_local_pos = match offset {
                Vec3 { x: -1, y: 0, z: 0 } => vec3u5::new(chunk::LENGTH as u8 -1, position.local.y(), position.local.z()),
                Vec3 { x: 1, y: 0, z: 0 } => vec3u5::new(0, position.local.y(), position.local.z()),
                Vec3 { x: 0, y: -1, z: 0 } => vec3u5::new(position.local.x(), chunk::LENGTH as u8 -1, position.local.z()),
                Vec3 { x: 0, y: 1, z: 0 } => vec3u5::new(position.local.x(), 0, position.local.z()),
                Vec3 { x: 0, y: 0, z: -1 } => vec3u5::new(position.local.x(), position.local.y(), chunk::LENGTH as u8 -1),
                Vec3 { x: 0, y: 0, z: 1 } => vec3u5::new(position.local.x(), position.local.y(), 0),
                _ => unreachable!(),
            };
            let chunk = self.chunk(neighbor_chunk_coord);
            let index = linearize(neighbor_local_pos);
            chunk.data[index].insert_faces(face);
            chunk.dirtied_positions.push(neighbor_local_pos);
        }

        self.chunk(position.chunk).set(position.local, material);
    }

    pub fn get_cube_material(&self, position: impl Into<CubePosition>) -> Option<Material> {
        let position = ChunkLocalPosition::from(position.into());
        self.get_chunk(position.chunk).map(|x| x.get(position.local)).flatten()
    }

    pub fn cube_material(&mut self, position: impl Into<CubePosition>) -> Option<Material> {
        let position = ChunkLocalPosition::from(position.into());
        self.chunk(position.chunk).get(position.local)
    }

    pub fn cast_ray(&mut self, mut origin: vec3f, direction: vec3f, range: f32) -> Option<(vec3i, Face)> {
        origin += 0.5;
        let end = origin + direction * range;

        let origin = origin.into_array().into();
        let dest = end.into_array().into();
        for (prev, curr) in WalkVoxels::new(origin, dest, &VoxelOrigin::Corner).steps() {
            let pos = Vec3::from(curr);
            let norm = Vec3::from(prev) - pos;

            if let Some(material) = self.get_cube_material(pos) {
                if material.can_collide() {
                    return Some((pos, Face::from(norm)));
                }
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