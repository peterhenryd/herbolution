use crate::chunk;
use crate::chunk::channel::ServerChunkChannel;
use crate::chunk::cube::CubePosition;
use crate::chunk::generator::GenerationParams;
use crate::chunk::material::Material;
use crate::chunk::provider::ChunkProvider;
use crate::chunk::{Chunk, ChunkLocalPos};
use crossbeam::channel::bounded;
use lib::geo::face::{Face, Faces};
use lib::geo::cuboid::Cuboid;
use line_drawing::{VoxelOrigin, WalkVoxels};
use math::vector::{vec3d, vec3f, vec3i, vec3u4, Vec3};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug)]
pub struct ChunkMap {
    map: HashMap<vec3i, Chunk>,
    provider: ChunkProvider,
    channel: ServerChunkChannel,
    thread_pool: Rc<ThreadPool>,
}

impl ChunkMap {
    pub fn new(seed: i64, channel: ServerChunkChannel, dir_path: PathBuf) -> Self {
        let map = HashMap::new();
        let generation_params = Arc::new(GenerationParams::new(seed));
        let provider = ChunkProvider::new(dir_path, generation_params);
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build()
            .unwrap();

        Self { map, provider, channel, thread_pool: Rc::new(thread_pool) }
    }

    pub fn get_near_colliders(&self, cuboid: Cuboid<f64>, colliders: &mut Vec<Cuboid<f64>>) {
        let min = cuboid.min.floor().cast().unwrap() - 1;
        let max = cuboid.max.ceil().cast().unwrap() + 1;

        colliders.clear();
        for x in min.x..max.x {
            for y in min.y..max.y {
                for z in min.z..max.z {
                    let Some(material) = self.get_cube_material(CubePosition(Vec3::new(x, y, z))) else { continue };

                    if !material.can_collide() {
                        continue;
                    }

                    colliders.push(Cuboid::new(
                        Vec3::new(x as f64, y as f64, z as f64),
                        Vec3::new(x as f64 + 1.0, y as f64 + 1.0, z as f64 + 1.0),
                    ));
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

    pub fn set_cube(&mut self, pos: impl Into<CubePosition>, material: Option<Material>) {
        let pos = ChunkLocalPos::from(pos.into());
        let edges = [
            (Vec3::new(-1, 0, 0), Faces::EAST, pos.local.x() == 0),
            (Vec3::new(1, 0, 0), Faces::WEST, pos.local.x() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, -1, 0), Faces::UP, pos.local.y() == 0),
            (Vec3::new(0, 1, 0), Faces::DOWN, pos.local.y() == chunk::LENGTH as u8 - 1),
            (Vec3::new(0, 0, -1), Faces::NORTH, pos.local.z() == 0),
            (Vec3::new(0, 0, 1), Faces::SOUTH, pos.local.z() == chunk::LENGTH as u8 - 1),
        ];

        for (offset, face, condition) in edges {
            if !condition {
                continue;
            }

            let neighbor_chunk_coord = pos.chunk + offset;
            let neighbor_local_pos = match offset {
                Vec3 { x: -1, y: 0, z: 0 } => vec3u4::new(chunk::LENGTH as u8 -1, pos.local.y(), pos.local.z()),
                Vec3 { x: 1, y: 0, z: 0 } => vec3u4::new(0, pos.local.y(), pos.local.z()),
                Vec3 { x: 0, y: -1, z: 0 } => vec3u4::new(pos.local.x(), chunk::LENGTH as u8 -1, pos.local.z()),
                Vec3 { x: 0, y: 1, z: 0 } => vec3u4::new(pos.local.x(), 0, pos.local.z()),
                Vec3 { x: 0, y: 0, z: -1 } => vec3u4::new(pos.local.x(), pos.local.y(), chunk::LENGTH as u8 -1),
                Vec3 { x: 0, y: 0, z: 1 } => vec3u4::new(pos.local.x(), pos.local.y(), 0),
                _ => unreachable!(),
            };
            let Some(chunk) = self.get_chunk(neighbor_chunk_coord) else { continue };
            let index = neighbor_local_pos.linearize();

            let mut mesh = chunk.mesh.write();
            mesh.data[index].insert_faces(face);
            
            /*
            let Vec3 { x, y, z } = pos.local.cast::<i32>().unwrap().add(offset);
            if x == 0 || x == 15 && material.is_none() {
                mesh.exposed_faces.set(Face::from_vec3(Vec3::new(x / 15 * 2 - 1, 0, 0)).unwrap().into(), true);
            }
            if y == 0 || y == 15 && material.is_none() {
                mesh.exposed_faces.set(Face::from_vec3(Vec3::new(0, y / 15 * 2 - 1, 0)).unwrap().into(), true);
            }
            if z == 0 || z == 15 && material.is_none() {
                mesh.exposed_faces.set(Face::from_vec3(Vec3::new(0, 0, z / 15 * 2 - 1)).unwrap().into(), true);
            }
            
             */
            
            mesh.updated_positions.push(neighbor_local_pos);
        }

        let Some(chunk) = self.get_chunk(pos.chunk) else { return };
        chunk.mesh.write().set(pos.local, material);
    }

    pub fn get_cube_material(&self, pos: impl Into<CubePosition>) -> Option<Material> {
        let pos = ChunkLocalPos::from(pos.into());
        self.get_chunk(pos.chunk)?.mesh.read().get(pos.local)
    }

    pub fn cast_ray(&mut self, mut origin: vec3d, direction: vec3f, range: f32) -> Option<(vec3i, Face)> {
        origin += 0.5;
        let end = origin + direction.cast().unwrap() * range as f64;

        let origin = origin.to_tuple();
        let dest = end.to_tuple();
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
            let position = mesh.pos;
            let render_flag = Arc::new(AtomicBool::new(true));
            let (sender, receiver) = bounded(4);
            let chunk = Chunk::new(mesh, sender, self.thread_pool.clone(), render_flag.clone());
            self.channel.send_load(position, receiver, render_flag);
            
            for offset in Face::entries().map(Face::to_normal) {
                let temp_mesh = chunk.mesh.clone();
                let Some(other) = self.get_chunk(position + offset) else { continue };
                let other_mesh = other.mesh.clone();

                // TODO: given that both chunk meshes are referenced mutably, there is currently no
                // performance gain from making this task async.
                // The solution is to adapt the signature to be Mesh::cull_shared_faces(&mut self, &Mesh),
                // to allow for all adjacent chunks to be processed in parallel, however, this requires
                // rewriting the culling function to process the cull-ee and cull-er separately.
                self.thread_pool.spawn(move || {
                    temp_mesh.write().cull_shared_faces(&mut *other_mesh.write());
                });
            }

            self.map.insert(position, chunk);
        }
    }

    pub fn tick(&mut self) {
        self.load_generated();

        for chunk in self.map.values() {
            chunk.tick(self.provider.dir_path.clone());
        }
    }
}
