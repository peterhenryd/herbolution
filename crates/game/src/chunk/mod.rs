use crate::chunk::channel::ChunkUpdate;
use crate::chunk::cube::{Cube, CubePosition};
use crate::chunk::material::{Material, OptionMaterialExt};
use crossbeam::channel::Sender;
use hashbrown::HashMap;
use lib::geo::face::{Face, Faces};
use math::vector::{vec3i, vec3u4, Vec3};
use parking_lot::RwLock;
use rayon::ThreadPool;
use std::ops::{BitAnd, Not, Range, Sub};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use math::num::traits::{ConstOne, ConstZero};

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
    thread_pool: Rc<ThreadPool>,
    render_flag: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct CubeMesh {
    pub pos: vec3i,
    data: Box<[Cube<Option<Material>>]>,
    updated_positions: Vec<vec3u4>,
    //exposed_faces: Faces,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPos {
    pub chunk: vec3i,
    pub local: vec3u4,
}

pub struct CubeGrid {
    data: Box<[Option<Material>]>,
}

impl Chunk {
    pub fn new(mesh: CubeMesh, sender: Sender<ChunkUpdate>, thread_pool: Rc<ThreadPool>, render_flag: Arc<AtomicBool>) -> Self {
        Self {
            pos: mesh.pos,
            mesh: Arc::new(RwLock::new(mesh)),
            sender,
            thread_pool,
            render_flag,
        }
    }

    fn send_overwrite_update(&self) {
        let Some(mesh) = self.mesh.try_read() else { return };
        
        //self.render_flag.store(!mesh.exposed_faces.is_empty(), Ordering::Relaxed);

        if mesh.updated_positions.is_empty() {
            return;
        }

        drop(mesh);

        let mesh = self.mesh.clone();
        let sender = self.sender.clone();

        self.thread_pool.spawn(move || {
            let data = mesh.read().data.clone();
            let dirtied_pos = mesh.write().updated_positions
                .drain(..)
                .collect::<Vec<_>>();

            let mut overwrites = HashMap::new();
            for pos in dirtied_pos {
                let index = pos.linearize();
                let cube = data[index];
                overwrites.insert(pos, cube);
            }
            let _ = sender.send(ChunkUpdate { overwrites });
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
            //exposed_faces: Faces::all(),
        }
    }

    pub fn get(&self, pos: vec3u4) -> Option<Material> {
        self.data[pos.linearize()].material
    }

    pub fn cull_shared_faces(&mut self, other: &mut CubeMesh) {
        let Some(shared_face) = Face::from_normal(other.pos - self.pos) else {
            return;
        };
        let other_shared_face = shared_face.inverse();

        fn sized_boundary_slice(face: Face) -> Vec3<Range<u8>> {
            let l = LENGTH as u8;
            match face {
                Face::East => Vec3::new(l - 1..l, 0..l, 0..l),
                Face::West => Vec3::new(0..1, 0..l, 0..l),
                Face::Up => Vec3::new(0..l, l - 1..l, 0..l),
                Face::Down => Vec3::new(0..l, 0..1, 0..l),
                Face::North => Vec3::new(0..l, 0..l, l - 1..l),
                Face::South => Vec3::new(0..l, 0..l, 0..1),
            }
            
            /*
            match face {
                Face::East => Vec3::new(
                    length - T::ONE..length,
                    T::ZERO..length,
                    T::ZERO..length,
                ),
                Face::West => Vec3::new(
                    T::ZERO..T::ONE,
                    T::ZERO..length,
                    T::ZERO..length,
                ),
                Face::Up => Vec3::new(
                    T::ZERO..length,
                    length - T::ONE..length,
                    T::ZERO..length,
                ),
                Face::Down => Vec3::new(
                    T::ZERO..length,
                    T::ZERO..T::ONE,
                    T::ZERO..length,
                ),
                Face::North => Vec3::new(
                    T::ZERO..length,
                    T::ZERO..length,
                    length - T::ONE..length,
                ),
                Face::South => Vec3::new(
                    T::ZERO..length,
                    T::ZERO..length,
                    T::ZERO..T::ONE,
                ),
            }
            
             */
        }

        let matric = sized_boundary_slice(shared_face);
        let other_matric = sized_boundary_slice(other_shared_face);
        
        let mut is_exposed = false;
        for (x1, x2) in matric.x.zip(other_matric.x) {
            for (y1, y2) in matric.y.clone().zip(other_matric.y.clone()) {
                for (z1, z2) in matric.z.clone().zip(other_matric.z.clone()) {
                    let position = vec3u4::new(x1, y1, z1);
                    let other_position = vec3u4::new(x2, y2, z2);
                    let cube = &mut self.data[position.linearize()];
                    let other_cube = &mut other.data[other_position.linearize()];

                    if cube.material.is_face_culled() && other_cube.material.is_face_culled() {
                        cube.remove_faces(Faces::from(shared_face));
                        other_cube.remove_faces(Faces::from(other_shared_face));

                        self.updated_positions.push(position);
                        other.updated_positions.push(other_position);
                    } else {
                        is_exposed = true;
                    }
                }
            }
        }
        
        //self.exposed_faces.set(shared_face.into(), is_exposed);
        //other.exposed_faces.set(other_shared_face.into(), is_exposed);
    }

    pub fn set(&mut self, pos: vec3u4, new_material: Option<Material>) {
        let i = pos.linearize();
        let old_material = self.data[i].material;
        
        if new_material == old_material {
            return;
        }

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
        
        /*let (x, y, z) = (pos.x(), pos.y(), pos.z());
        if x == 0 || x == 15 && new_material.is_none() {
            self.exposed_faces.set(Face::from_vec3(Vec3::new(if x == 0 { -1 } else { 1 }, 0, 0)).unwrap().into(), true);
        }
        if y == 0 || y == 15 && new_material.is_none() {
            self.exposed_faces.set(Face::from_vec3(Vec3::new(0, if y == 0 { -1 } else { 1 }, 0)).unwrap().into(), true);
        }
        if z == 0 || z == 15 && new_material.is_none() {
            self.exposed_faces.set(Face::from_vec3(Vec3::new(0, 0, if y == 0 { -1 } else { 1 })).unwrap().into(), true);
        }
        
         */

        self.updated_positions.push(pos);
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, pos: vec3u4) {
        faces.variant_iter()
            .map(|f| (f, f.to_normal()))
            .map(|(f, v)| (f, pos.cast::<i32>().unwrap() + v))
            //.filter(|(_, x)| in_bounds(*x))
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u4::try_from(v).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.linearize();
                self.updated_positions.push(v);

                if self.data[index].material.is_face_culled() {
                    self.data[pos.linearize()].remove_faces(Faces::from(f));
                }

                self.data[index].remove_faces(Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, pos: vec3u4) {
        let pos = pos.cast::<i32>().unwrap();

        let in_chunk = faces.not()
            .variant_iter()
            .filter_map(|a| (a.to_normal() + pos)
                .cast::<u8>()
                .map(|v| vec3u4::try_from(v))
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
            local: pos.0.bitand(LENGTH as i32 - 1).into()
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
                    let pos = vec3u4::new(x as u8, y as u8, z as u8);
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
                    let pos = vec3u4::new(x as u8, y as u8, z as u8);
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