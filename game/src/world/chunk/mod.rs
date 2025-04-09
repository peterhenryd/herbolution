use crate::world::chunk::cube::{Cube, CubePos};
use crate::world::chunk::material::{Material, OptionMaterialExt};
use crossbeam::channel::Sender;
use hashbrown::HashMap;
use lib::geometry::cuboid::face::{Face, Faces};
use math::vector::{vec3i, vec3u5, Vec3};
use std::ops::{BitAnd, Not};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task;

pub mod cube;
pub mod generator;
pub mod map;
pub mod material;
pub mod channel;

pub const LENGTH: usize = 32;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk {
    pub pos: vec3i,
    mesh: RwLock<Mesh>,
    sender: Sender<ChunkUpdate>,
}

impl Chunk {
    pub fn new(mesh: Mesh, sender: Sender<ChunkUpdate>) -> Self {
        Self {
            pos: mesh.pos,
            mesh: RwLock::new(mesh),
            sender,
        }
    }

    fn send_overwrite_update(self: &Arc<Self>) {
        let Ok(mesh) = self.mesh.try_read() else { return };

        if mesh.dirtied_pos.is_empty() {
            return;
        }

        let chunk = self.clone();
        task::spawn(async move {
            let dirtied_pos = chunk.mesh.write().await
                .dirtied_pos
                .drain(..)
                .collect::<Vec<_>>();
            let mesh = chunk.mesh.read().await;

            let mut overwrites = HashMap::new();
            for pos in dirtied_pos {
                let index = linearize(pos);
                let cube = mesh.data[index];
                overwrites.insert(pos, cube);
            }
            let _ = chunk.sender.send(ChunkUpdate { overwrites });
        });
    }

    pub fn tick(self: &Arc<Self>) {
        self.send_overwrite_update();
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub pos: vec3i,
    data: Box<[Cube<Option<Material>>; SIZE]>,
    dirtied_pos: Vec<vec3u5>,
}

impl Mesh {
    pub fn new(pos: vec3i) -> Self {
        Self {
            pos,
            data: Box::new([Cube::new(None); SIZE]),
            dirtied_pos: vec![],
        }
    }

    pub fn get(&self, pos: vec3u5) -> Option<Material> {
        self.data[linearize(pos)].material
    }

    pub fn cull_shared_faces(&mut self, other: &mut Mesh) {
        let Some(shared_face) = Face::from_vec3(self.pos - other.pos) else {
            return;
        };

        let this_matrix = shared_face.inverse().sized_boundary_slice(LENGTH as u8);
        let that_matrix = shared_face.sized_boundary_slice(LENGTH as u8);

        for (x1, x2) in this_matrix.x.zip(that_matrix.x) {
            for (y1, y2) in this_matrix.y.clone().zip(that_matrix.y.clone()) {
                for (z1, z2) in this_matrix.z.clone().zip(that_matrix.z.clone()) {
                    let pos1 = vec3u5::new(x1, y1, z1);
                    let pos2 = vec3u5::new(x2, y2, z2);
                    let this = &mut self.data[linearize(pos1)];
                    let that = &mut other.data[linearize(pos2)];

                    if this.material.is_face_culled() && that.material.is_face_culled() {
                        this.remove_faces(Faces::from(shared_face.inverse()));
                        that.remove_faces(Faces::from(shared_face));

                        self.dirtied_pos.push(pos1);
                        other.dirtied_pos.push(pos2);
                    }
                }
            }
        }
    }

    pub fn set(&mut self, pos: vec3u5, new_material: Option<Material>) {
        let i = linearize(pos);
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

        self.dirtied_pos.push(pos);
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, pos: vec3u5) {
        faces
            .map(|f| (f, f.into_vec3()))
            .map(|(f, v)| (f, pos.cast::<i32>().unwrap() + v))
            //.filter(|(_, x)| in_bounds(*x))
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u5::try_new(v.x, v.y, v.z).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.cast().unwrap().linearize(LENGTH);
                self.dirtied_pos.push(v);

                if self.data[index].material.is_face_culled() {
                    self.data[linearize(pos)].remove_faces(Faces::from(f));
                }

                self.data[index].remove_faces(Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, pos: vec3u5) -> Vec<(Face, Vec3<i32>)> {
        let pos = pos.cast::<i32>().unwrap();

        let (in_chunk, out_chunk) = faces.not()
            .map(|f| (f, f.into_vec3() + pos))
            .partition::<Vec<_>, _>(|&(_, pos)| in_bounds(pos));

        for (f, Vec3 { x, y, z }) in in_chunk {
            let pos = vec3u5::new(x as u8, y as u8, z as u8);
            let index = linearize(pos);

            self.dirtied_pos.push(pos);
            self.data[index].insert_faces(Faces::from(f.inverse()));
        }

        out_chunk
    }
}

fn in_bounds(pos: vec3i) -> bool {
    pos.x >= 0 && pos.x < LENGTH as i32
        && pos.y >= 0 && pos.y < LENGTH as i32
        && pos.z >= 0 && pos.z < LENGTH as i32
}

pub fn linearize(pos: vec3u5) -> usize {
    pos.cast().unwrap().linearize(LENGTH)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPos {
    pub chunk: vec3i,
    pub local: vec3u5,
}

impl From<CubePos> for ChunkLocalPos {
    fn from(pos: CubePos) -> Self {
        ChunkLocalPos {
            chunk: pos.0 >> 5,
            local: pos.0.bitand(LENGTH as i32 - 1).into()
        }
    }
}

pub struct ChunkUpdate {
    pub overwrites: HashMap<vec3u5, Cube<Option<Material>>>,
}