use crate::chunk;
use crate::chunk::channel::ServerChunkChannel;
use crate::chunk::cube::CubePos;
use crate::chunk::generator::GenerationParams;
use crate::chunk::material::Material;
use crate::chunk::provider::ChunkProvider;
use crate::chunk::{linearize, Chunk, ChunkLocalPos};
use crossbeam::channel::bounded;
use lib::geometry::cuboid::face::{Face, Faces};
use lib::geometry::cuboid::Cuboid;
use line_drawing::{VoxelOrigin, WalkVoxels};
use math::vector::{vec3f, vec3i, vec3u5, Vec3};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task;

#[derive(Debug)]
pub struct ChunkMap {
    map: HashMap<vec3i, Chunk>,
    provider: ChunkProvider,
    channel: ServerChunkChannel,
}

impl ChunkMap {
    pub fn new(seed: i32, channel: ServerChunkChannel, dir_path: PathBuf) -> Self {
        let map = HashMap::new();
        let generation_params = Arc::new(GenerationParams::new(seed));
        let provider = ChunkProvider::new(dir_path, generation_params);

        Self { map, provider, channel }
    }

    pub fn get_near_colliders(&self, cuboid: Cuboid<f32>, colliders: &mut Vec<Cuboid<f32>>) {
        let min = cuboid.min.floor().cast().unwrap() - 1;
        let max = cuboid.max.ceil().cast().unwrap() + 1;

        colliders.clear();
        for x in min.x..max.x {
            for y in min.y..max.y {
                for z in min.z..max.z {
                    if let Some(material) = self.get_cube_material(CubePos(Vec3::new(x, y, z))) {
                        if material.can_collide() {
                            colliders.push(Cuboid::new(
                                Vec3::new(x as f32, y as f32, z as f32),
                                Vec3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0),
                            ));
                        }
                    }
                }
            }
        }
    }

    pub fn get_chunk(&self, pos: vec3i) -> Option<&Chunk> {
        self.map.get(&pos)
    }

    pub fn get_chunk_mut(&mut self, pos: vec3i) -> Option<&mut Chunk> {
        self.map.get_mut(&pos)
    }

    pub fn unload_chunk(&mut self, pos: vec3i) {
        self.map.remove(&pos);
        self.channel.send_unload(pos);
    }

    pub fn load_chunk(&mut self, pos: vec3i) {
        if self.map.contains_key(&pos) {
            return;
        }

        self.provider.request(pos);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> {
        self.map.values()
    }

    pub fn set_cube(&mut self, pos: impl Into<CubePos>, material: Option<Material>) {
        let pos = ChunkLocalPos::from(pos.into());
        let edges = [
            (Vec3::new(-1, 0, 0), Faces::RIGHT, pos.local.x() == 0),
            (Vec3::new(1, 0, 0), Faces::LEFT, pos.local.x() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, -1, 0), Faces::TOP, pos.local.y() == 0),
            (Vec3::new(0, 1, 0), Faces::BOTTOM, pos.local.y() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, 0, -1), Faces::FRONT, pos.local.z() == 0),
            (Vec3::new(0, 0, 1), Faces::BACK, pos.local.z() == chunk::LENGTH as u8 - 1),
        ];

        for (offset, face, condition) in edges {
            if !condition {
                continue;
            }

            let neighbor_chunk_coord = pos.chunk + offset;
            let neighbor_local_pos = match offset {
                Vec3 { x: -1, y: 0, z: 0 } => vec3u5::new(chunk::LENGTH as u8 -1, pos.local.y(), pos.local.z()),
                Vec3 { x: 1, y: 0, z: 0 } => vec3u5::new(0, pos.local.y(), pos.local.z()),
                Vec3 { x: 0, y: -1, z: 0 } => vec3u5::new(pos.local.x(), chunk::LENGTH as u8 -1, pos.local.z()),
                Vec3 { x: 0, y: 1, z: 0 } => vec3u5::new(pos.local.x(), 0, pos.local.z()),
                Vec3 { x: 0, y: 0, z: -1 } => vec3u5::new(pos.local.x(), pos.local.y(), chunk::LENGTH as u8 -1),
                Vec3 { x: 0, y: 0, z: 1 } => vec3u5::new(pos.local.x(), pos.local.y(), 0),
                _ => unreachable!(),
            };
            let Some(chunk) = self.get_chunk(neighbor_chunk_coord) else { continue };
            let index = linearize(neighbor_local_pos);
            let Ok(mut mesh) = chunk.mesh.try_write() else { continue };
            mesh.data[index].insert_faces(face);
            mesh.dirtied_pos.push(neighbor_local_pos);
        }

        let Some(chunk) = self.get_chunk(pos.chunk) else { return };
        if let Ok(mut mesh) = chunk.mesh.try_write() {
            mesh.set(pos.local, material);
        }
    }

    pub fn get_cube_material(&self, pos: impl Into<CubePos>) -> Option<Material> {
        let pos = ChunkLocalPos::from(pos.into());
        self.get_chunk(pos.chunk)?.mesh.try_read().ok()?.get(pos.local)
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

    fn load_generated(&mut self) {
        for mesh in self.provider.dequeue() {
            let (sender, receiver) = bounded(4);
            self.channel.send_load(mesh.pos, receiver);

            let chunk = Chunk::new(mesh, sender);
            for offset in Face::entries().map(Face::into_vec3) {
                let mesh = chunk.mesh.clone();
                let Some(other) = self.get_chunk(chunk.pos + offset) else { continue };
                let other_mesh = other.mesh.clone();

                // TODO: given that both chunk meshes are referenced mutably, there is currently no
                // performance gain from making this task async.
                // The solution is to adapt the signature to be Mesh::cull_shared_faces(&mut self, &Mesh),
                // to allow for all adjacent chunks to be processed in parallel, however, this requires
                // rewriting the culling function to process the cull-ee and cull-er separately.
                task::spawn(async move {
                    mesh.write().await.cull_shared_faces(&mut *other_mesh.write().await);
                });
            }

            self.map.insert(chunk.pos, chunk);
        }
    }

    pub fn tick(&mut self) {
        self.load_generated();

        for chunk in self.map.values_mut() {
            chunk.tick(self.provider.dir_path.clone());
        }
    }
}