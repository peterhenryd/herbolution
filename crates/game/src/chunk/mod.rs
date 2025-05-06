use crate::chunk::channel::ChunkUpdate;
use crate::chunk::cube::{Cube, CubePosition};
use crate::chunk::material::{Material, OptionMaterialExt};
use crossbeam::channel::Sender;
use hashbrown::HashMap;
use lib::geometry::cuboid::face::{Face, Faces};
use math::vector::{vec3i, vec4u4};
use std::ops::{BitAnd, Not};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{RwLock, TryLockError};
use tokio::task;

pub mod cube;
pub mod generator;
pub mod map;
pub mod material;
pub mod channel;
pub mod provider;

pub const LENGTH: usize = 16;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk {
    pub pos: vec3i,
    mesh: Arc<RwLock<CubeMesh>>,
    sender: Sender<ChunkUpdate>,
    //save_counter: u64,
}

#[derive(Debug, Clone)]
pub struct CubeMesh {
    pub pos: vec3i,
    data: Box<[Cube<Option<Material>>]>,
    updated_positions: Vec<vec4u4>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPos {
    pub chunk: vec3i,
    pub local: vec4u4,
}

pub struct CubeGrid {
    data: Box<[Option<Material>]>,
}

impl Chunk {
    pub fn new(mesh: CubeMesh, sender: Sender<ChunkUpdate>) -> Self {
        Self {
            pos: mesh.pos,
            mesh: Arc::new(RwLock::new(mesh)),
            sender,
            //save_counter: 0,
        }
    }

    fn send_overwrite_update(&self) {
        let Ok(mesh) = self.mesh.try_read() else { return };

        if mesh.updated_positions.is_empty() {
            return;
        }

        drop(mesh);

        let mesh = self.mesh.clone();
        let sender = self.sender.clone();

        task::spawn_blocking(move || {
            let data = mesh.try_read()?.data.clone();
            let dirtied_pos = mesh.try_write()?
                .updated_positions
                .drain(..)
                .collect::<Vec<_>>();

            let mut overwrites = HashMap::new();
            for pos in dirtied_pos {
                let index = pos.linearize();
                let cube = data[index];
                overwrites.insert(pos, cube);
            }
            let _ = sender.send(ChunkUpdate { overwrites });

            Ok::<_, TryLockError>(())
        });
    }

    /*fn save_chunk(&mut self, path: PathBuf) {
        if self.save_counter < TICKS_PER_SECOND * 15 {
            self.save_counter += 1;
            return;
        }

        self.save_counter = 0;

        let pos = self.pos;
        let mesh = self.mesh.clone();
        task::spawn(async move {
            let path = path.join(display::Join::new(&pos, ".").to_string());
            let material_mesh = CubeGrid::from_mesh(&*mesh.read().await);

            write(path, &material_mesh.encode()).await.unwrap();
        });
    }
     */

    pub fn tick(&self, _: PathBuf) {
        self.send_overwrite_update();
        //self.save_chunk(path);
    }
}

impl CubeMesh {
    pub fn new(pos: vec3i) -> Self {
        Self {
            pos,
            data: Box::new([Cube::new(None); SIZE]),
            updated_positions: vec![],
        }
    }

    pub fn get(&self, pos: vec4u4) -> Option<Material> {
        self.data[pos.linearize()].material
    }

    pub fn cull_shared_faces(&mut self, other: &mut CubeMesh) {
        let Some(shared_face) = Face::from_vec3(other.pos - self.pos) else {
            return;
        };

        let this_matrix = shared_face.sized_boundary_slice(LENGTH as u8);
        let that_matrix = shared_face.inverse().sized_boundary_slice(LENGTH as u8);

        for (x1, x2) in this_matrix.x.zip(that_matrix.x) {
            for (y1, y2) in this_matrix.y.clone().zip(that_matrix.y.clone()) {
                for (z1, z2) in this_matrix.z.clone().zip(that_matrix.z.clone()) {
                    let pos1 = vec4u4::new(x1, y1, z1, 0);
                    let pos2 = vec4u4::new(x2, y2, z2, 0);
                    let this = &mut self.data[pos1.linearize()];
                    let that = &mut other.data[pos2.linearize()];

                    if this.material.is_face_culled() && that.material.is_face_culled() {
                        this.remove_faces(Faces::from(shared_face));
                        that.remove_faces(Faces::from(shared_face.inverse()));

                        self.updated_positions.push(pos1);
                        other.updated_positions.push(pos2);
                    }
                }
            }
        }
    }

    pub fn set(&mut self, pos: vec4u4, new_material: Option<Material>) {
        let i = pos.linearize();
        let old_material = self.data[i].material;

        self.data[i].material = new_material;

        if !old_material.is_face_culled() && new_material.is_face_culled() {
            let missing = self.data[i].faces().not();
            self.data[i].set_opaque(Faces::all());
            self.remove_neighboring_faces(missing, pos);
        }

        if old_material.is_face_culled() && !new_material.is_face_culled() {
            let present = self.data[i].faces();
            self.data[i].set_opaque(Faces::empty());
            self.add_neighboring_faces(present, pos);
        }

        self.updated_positions.push(pos);
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, pos: vec4u4) {
        faces.var_iter()
            .map(|f| (f, f.into_vec3()))
            .map(|(f, v)| (f, pos.cast::<i32>().unwrap().xyz() + v))
            //.filter(|(_, x)| in_bounds(*x))
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec4u4::try_from(v.extend(0)).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.cast().unwrap().linearize(LENGTH);
                self.updated_positions.push(v);

                if self.data[index].material.is_face_culled() {
                    self.data[pos.linearize()].remove_faces(Faces::from(f));
                }

                self.data[index].remove_faces(Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, pos: vec4u4) {
        let pos = pos.cast::<i32>().unwrap().xyz();

        let in_chunk = faces.not()
            .var_iter()
            .filter_map(|a| (a.into_vec3() + pos)
                .cast::<u8>()
                .map(|v| vec4u4::try_from(v.extend(0)))
                .flatten()
                .map(|b| (a, b))
            );

        for (f, pos) in in_chunk {
            self.updated_positions.push(pos);
            self.data[pos.linearize()].insert_faces(Faces::from(f.inverse()));
        }
    }
}

impl From<CubePosition> for ChunkLocalPos {
    fn from(pos: CubePosition) -> Self {
        ChunkLocalPos {
            chunk: pos.0 >> 4,
            local: pos.0.bitand(LENGTH as i32 - 1).extend(0).into()
        }
    }
}

impl CubeGrid {
    pub fn new() -> Self {
        Self {
            data: Box::new([None; SIZE]),
        }
    }

    pub fn from_mesh(cube_mesh: &CubeMesh) -> Self {
        let mut mesh = Self::new();
        for x in 0..LENGTH {
            for y in 0..LENGTH {
                for z in 0..LENGTH {
                    let pos = vec4u4::new(x as u8, y as u8, z as u8, 0);
                    let index = pos.linearize();
                    mesh.data[index] = cube_mesh.data[index].material;
                }
            }
        }
        mesh
    }

    pub fn to_mesh(&self, pos: vec3i) -> CubeMesh {
        let mut mesh = CubeMesh::new(pos);
        for x in 0..LENGTH {
            for y in 0..LENGTH {
                for z in 0..LENGTH {
                    let pos = vec4u4::new(x as u8, y as u8, z as u8, 0);
                    let index = pos.linearize();
                    mesh.set(pos, self.data[index]);
                }
            }
        }
        mesh
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        let mut count = 0;
        let mut current = None;

        for &given in self.data.iter() {
            if current != given || count == u8::MAX {
                vec.push(count);
                vec.extend(current.map(|x| x as u16).unwrap_or(0).to_le_bytes());
                current = given;
                count = 1;
            } else {
                count += 1;
            }
        }

        vec.push(count);
        vec.extend(current.map(|x| x as u16).unwrap_or(0).to_le_bytes());

        vec
    }

    pub fn decode(bytes: &[u8]) -> Self {
        let mut mesh = Self::new();

        let mut i = 0;
        for &[count, m0, m1] in bytes.array_chunks::<3>() {
            let material = Material::from_id(u16::from_le_bytes([m0, m1]));

            for _ in 0..count {
                mesh.data[i] = material;
                i += 1;
            }
        }

        mesh
    }
}