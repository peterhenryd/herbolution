use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::sync::Arc;

use lib::aabb::Aabb3;
use lib::collections::mailbox::Mailbox;
use lib::point::{ChunkCubePt, ChunkPt, CubePt};
use lib::spatial::{CubeFace, CubeFaces};
use lib::task::THREAD_POOL;
use lib::util::{GroupKey, GroupKeyBuf};
use lib::vector::{vec3d, vec3f, vec3i, vec3u5, Vec3};
use lib::world::CHUNK_LENGTH;
use line_drawing::{VoxelOrigin, WalkVoxels};

use crate::chunk::handle::ChunkLoad;
use crate::chunk::material::{Material, PaletteMaterialId};
use crate::chunk::provider::ChunkProvider;
use crate::chunk::{handle, Chunk};
use crate::handle::ClientHandle;

#[derive(Debug)]
pub struct ChunkMap {
    map: HashMap<ChunkPt, Chunk>,
    provider: ChunkProvider,
    unloader: Mailbox<ChunkPt>,
}

impl ChunkMap {
    pub fn new(seed: i64, dir_path: PathBuf) -> Self {
        Self {
            map: HashMap::new(),
            provider: ChunkProvider::new(dir_path, seed),
            unloader: Mailbox::default(),
        }
    }

    pub fn get_near_colliders(&self, aabb: Aabb3<f64>, colliders: &mut Vec<Aabb3<f64>>) {
        let min = aabb.min.floor().cast() - 1;
        let max = aabb.max.ceil().cast() + 1;

        colliders.clear();
        for x in min.x..max.x {
            for y in min.y..max.y {
                for z in min.z..max.z {
                    let position = Vec3::new(x, y, z);
                    if !self.has_collider(CubePt(position)) {
                        continue;
                    }

                    colliders.push(Aabb3::cube(position.cast()));
                }
            }
        }
    }

    pub fn get_chunk(&self, position: ChunkPt) -> Option<&Chunk> {
        self.map.get(&position)
    }

    pub fn get_chunk_mut(&mut self, position: ChunkPt) -> Option<&mut Chunk> {
        self.map.get_mut(&position)
    }

    pub fn queue_unload(&self, position: ChunkPt) {
        let _ = self.unloader.push(position);
    }

    pub fn queue_load(&self, position: ChunkPt) {
        if self.map.contains_key(&position) {
            return;
        }

        self.provider.request(position);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> {
        self.map.values()
    }

    pub fn set_cube<'a>(&mut self, position: impl Into<CubePt>, material_ref: impl MaterialRef) {
        let ChunkCubePt { chunk, local } = position.into().into();
        let material_key = material_ref.as_key_ref();

        let edges = [
            (Vec3::new(-1, 0, 0), CubeFaces::EAST, local.x() == 0),
            (Vec3::new(1, 0, 0), CubeFaces::WEST, local.x() == CHUNK_LENGTH as u8 - 1),
            (Vec3::new(0, -1, 0), CubeFaces::UP, local.y() == 0),
            (Vec3::new(0, 1, 0), CubeFaces::DOWN, local.y() == CHUNK_LENGTH as u8 - 1),
            (Vec3::new(0, 0, -1), CubeFaces::NORTH, local.z() == 0),
            (Vec3::new(0, 0, 1), CubeFaces::SOUTH, local.z() == CHUNK_LENGTH as u8 - 1),
        ];

        for (offset, face, condition) in edges {
            if !condition {
                continue;
            }

            let neighbor_chunk_coord = ChunkPt(chunk.0 + offset);
            let neighbor_local_pos = match offset {
                Vec3 { x: -1, y: 0, z: 0 } => vec3u5::new(CHUNK_LENGTH as u8 - 1, local.y(), local.z()),
                Vec3 { x: 1, y: 0, z: 0 } => vec3u5::new(0, local.y(), local.z()),
                Vec3 { x: 0, y: -1, z: 0 } => vec3u5::new(local.x(), CHUNK_LENGTH as u8 - 1, local.z()),
                Vec3 { x: 0, y: 1, z: 0 } => vec3u5::new(local.x(), 0, local.z()),
                Vec3 { x: 0, y: 0, z: -1 } => vec3u5::new(local.x(), local.y(), CHUNK_LENGTH as u8 - 1),
                Vec3 { x: 0, y: 0, z: 1 } => vec3u5::new(local.x(), local.y(), 0),
                _ => unreachable!(),
            };
            let Some(chunk) = self.get_chunk(neighbor_chunk_coord) else { continue };
            let index = neighbor_local_pos.linearize();

            let mut mesh = chunk.mesh.write();
            mesh.data[index].flags.insert_faces(face);

            let Vec3 { x, y, z } = local.try_cast::<i32>().unwrap().add(offset);
            if x == 0 || x == 15 && material_key.is_none() {
                mesh.exposed_faces.set(
                    CubeFace::from_normal(Vec3::new(x / 15 * 2 - 1, 0, 0))
                        .unwrap()
                        .into(),
                    true,
                );
            }
            if y == 0 || y == 15 && material_key.is_none() {
                mesh.exposed_faces.set(
                    CubeFace::from_normal(Vec3::new(0, y / 15 * 2 - 1, 0))
                        .unwrap()
                        .into(),
                    true,
                );
            }
            if z == 0 || z == 15 && material_key.is_none() {
                mesh.exposed_faces.set(
                    CubeFace::from_normal(Vec3::new(0, 0, z / 15 * 2 - 1))
                        .unwrap()
                        .into(),
                    true,
                );
            }

            mesh.updated_positions.push(neighbor_local_pos);
        }

        let Some(chunk) = self.get_chunk(chunk) else { return };
        let mut mesh = chunk.mesh.write();

        let material = material_key
            .map(|x| mesh.palette.get_id_by_key(x))
            .flatten();
        mesh.set(local, material);
    }

    pub fn get_material(&self, position: impl Into<CubePt>) -> Option<Arc<Material>> {
        let ChunkCubePt { chunk, local } = position.into().into();

        let mesh = self.get_chunk(chunk)?.mesh.read();
        let id = mesh.get(local)?;

        mesh.palette.get_by_id(id).cloned()
    }

    pub fn get_material_id(&self, position: impl Into<CubePt>) -> Option<PaletteMaterialId> {
        let ChunkCubePt { chunk, local } = position.into().into();
        self.get_chunk(chunk)?.mesh.read().get(local)
    }

    pub fn has_collider(&self, position: impl Into<CubePt>) -> bool {
        let ChunkCubePt { chunk, local } = position.into().into();
        let Some(chunk) = self.get_chunk(chunk) else {
            return false;
        };

        let mesh = chunk.mesh.read();
        mesh.get(local)
            .map(|id| mesh.palette.get_by_id(id))
            .flatten()
            .map(|material| material.has_collider)
            .unwrap_or(false)
    }

    pub fn cast_ray(&mut self, origin: vec3d, dir: vec3f, range: f32) -> Option<CubeHit> {
        let start = origin + 0.5;
        let end = origin + dir.cast() * range as f64;

        let voxel_walker = WalkVoxels::new(start.into(), end.into(), &VoxelOrigin::Corner);
        for (prev, curr) in voxel_walker.steps() {
            let position = Vec3::from(curr);
            let normal = Vec3::from(prev) - position;

            if self.has_collider(position) {
                let position = position.cast();
                return Some(CubeHit {
                    position: position.cast(),
                    face: CubeFace::from_normal(normal).unwrap(),
                    contact_point: Aabb3::cube(position).intersect_ray(start, dir.cast()),
                });
            }
        }

        None
    }

    fn load_provided(&mut self, handle: &ClientHandle) {
        for mesh in self.provider.dequeue() {
            let position = mesh.position;
            let (game_handle, client_handle) = handle::create(position);
            let chunk = Chunk::new(mesh, client_handle);

            handle
                .chunks
                .load(ChunkLoad { position, handle: game_handle });

            for face in CubeFace::values() {
                let Some(other) = self.get_chunk(position + face.normal()) else {
                    continue;
                };

                let mesh = chunk.mesh.clone();
                let adj_mesh = other.mesh.clone();
                THREAD_POOL.spawn(move || {
                    mesh.write()
                        .cull_shared_faces(&mut *adj_mesh.write());
                });
            }

            self.map.insert(chunk.position, chunk);
        }
    }

    fn unload_requested(&mut self, handle: &ClientHandle) {
        for chunk_position in &self.unloader {
            self.map.remove(&chunk_position);
            handle.chunks.unload(chunk_position);
        }
    }

    pub fn update(&mut self, handle: &ClientHandle) {
        self.load_provided(handle);
        self.unload_requested(handle);

        for chunk in self.map.values() {
            chunk.update(self.provider.dir_path.clone());
        }
    }
}

pub trait MaterialRef {
    fn as_key_ref(&self) -> Option<&str>;
}

impl MaterialRef for GroupKey {
    fn as_key_ref(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl MaterialRef for GroupKeyBuf {
    fn as_key_ref(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl MaterialRef for &str {
    fn as_key_ref(&self) -> Option<&str> {
        Some(self)
    }
}

impl MaterialRef for Option<&str> {
    fn as_key_ref(&self) -> Option<&str> {
        *self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CubeHit {
    pub position: vec3i,
    pub face: CubeFace,
    pub contact_point: vec3d,
}
