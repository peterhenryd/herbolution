use crate::channel::ClientboundChunks;
use crate::world::chunk;
use crate::world::chunk::cube::CubePosition;
use crate::world::chunk::material::Material;
use crate::world::chunk::mesh::Mesh;
use crate::world::chunk::{generator, linearize, material, Chunk, ChunkLocalPosition};
use crossbeam::channel::bounded;
use lib::geometry::cuboid::face::{Face, Faces};
use lib::geometry::cuboid::Cuboid;
use line_drawing::{VoxelOrigin, WalkVoxels};
use math::vector::{vec3f, vec3i, vec3u5, Vec3};
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use std::collections::HashMap;
use std::sync::Arc;
use crate::world::chunk::generator::Generator;

#[derive(Debug)]
pub struct Map {
    map: HashMap<vec3i, Chunk>,
    generator: generator::Channel,
    clientbound: ClientboundChunks,
    materials: material::Registry,
}

impl Map {
    pub fn new(seed: i32, clientbound: ClientboundChunks) -> Self {
        let materials = material::Registry::new();
        Self {
            map: HashMap::new(),
            generator: generator::Channel::new(Arc::new(Generator::new(seed)), materials.clone()),
            clientbound,
            materials,
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
                        if material.has_collider {
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

    pub fn unload_chunk(&mut self, position: vec3i) {
        self.map.remove(&position);
        self.clientbound.send_unload_chunk(position);
    }

    pub fn load_chunk(&mut self, position: vec3i) {
        if self.map.contains_key(&position) {
            return;
        }

        self.generator.request_chunk(position);
    }

    fn wrap_mesh(&mut self, mesh: Mesh) -> Chunk {
        let (sender, receiver) = bounded(4);

        let chunk = Chunk::new(mesh, sender);
        self.clientbound.send_load_chunk(chunk.mesh.position, receiver);

        chunk
    }

    fn submit_queued(&mut self, mut chunk: Chunk) {
        for offset in Face::entries().map(Face::into_vec3) {
            let Some(other) = self.get_chunk_mut(chunk.mesh.position + offset) else { continue };
            chunk.mesh.cull_shared_faces(&mut other.mesh);
        }

        self.map.insert(chunk.mesh.position, chunk);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Chunk> {
        self.map.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
        self.map.values_mut()
    }

    pub fn set_cube(&mut self, position: impl Into<CubePosition>, material: Option<&Arc<Material>>) {
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
            let Some(chunk) = self.get_chunk_mut(neighbor_chunk_coord) else { continue };
            let index = linearize(neighbor_local_pos);
            chunk.mesh.data.insert_faces_at(index, face);
            chunk.mesh.updated_pos.push(neighbor_local_pos);
        }

        if let Some(chunk) = self.get_chunk_mut(position.chunk) {
            chunk.mesh.data.set_material(linearize(position.local), material, &mut chunk.mesh.materials);
        }
    }

    pub fn get_cube_material(&self, position: impl Into<CubePosition>) -> Option<&Arc<Material>> {
        let position = ChunkLocalPosition::from(position.into());
        self.get_chunk(position.chunk).map(|x| x.mesh.get(position.local)).flatten()
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
                if material.has_collider {
                    return Some((pos, Face::from(norm)));
                }
            }
        }

        None
    }

    pub fn tick(&mut self) {
        for mesh in self.generator.dequeue().collect::<Vec<_>>() {
            let chunk = self.wrap_mesh(mesh);
            self.submit_queued(chunk);
        }

        self.map.par_iter_mut()
            .for_each(|(_, chunk)| {
                chunk.send_overwrites();
            });

        for chunk in self.map.values_mut() {
            chunk.tick();
        }
    }
}