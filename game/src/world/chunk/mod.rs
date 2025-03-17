use crate::world::chunk::cube::{Cube, CubePosition};
use crate::world::chunk::material::{Material, OptionMaterialExt};
use kanal::Sender;
use lib::geometry::cuboid::face::{Face, Faces};
use math::vector::{vec3i, vec3u5, Vec3};
use std::ops::{BitAnd, Not};
use tinyvec::TinyVec;

pub mod cube;
pub mod generator;
pub mod map;
pub mod material;
pub mod position;

pub const LENGTH: usize = 32;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk {
    pub position: vec3i,
    data: Box<[Cube<Option<Material>>; SIZE]>,
    dirtied_positions: Vec<vec3u5>,
    sender: Sender<ChunkUpdate>,
}

impl Chunk {
    pub fn new(position: vec3i, sender: Sender<ChunkUpdate>) -> Self {
        Self {
            position,
            data: Box::new([Cube::new(None); SIZE]),
            dirtied_positions: vec![],
            sender,
        }
    }

    pub fn get(&self, position: vec3u5) -> Option<Material> {
        self.data[linearize(position)].material
    }

    // The basic idea here is to find the 32x32x1 slice of cubes on which faces are shared between two
    // chunks, and then to iterate over each cube and its corresponding cube, and cull their
    // shared faces if possible.
    pub fn cull_shared_faces(&mut self, other: &mut Chunk) {
        // The difference in position between the two chunks, this is a unit component vector.
        let dp = self.position - other.position;
        // Get the face that is shared between the two chunks.
        let Some(shared_face) = Face::from_vec3i(dp) else {
            return;
        };

        // Get the slices of cubes that are shared between the two chunks.
        let this_matrix = shared_face.inverse().sized_boundary_slice(LENGTH as u8);
        let that_matrix = shared_face.sized_boundary_slice(LENGTH as u8);

        for (x1, x2) in this_matrix.x.zip(that_matrix.x) {
            for (y1, y2) in this_matrix.y.clone().zip(that_matrix.y.clone()) {
                for (z1, z2) in this_matrix.z.clone().zip(that_matrix.z.clone()) {
                    let position1 = vec3u5::new(x1, y1, z1);
                    let position2 = vec3u5::new(x2, y2, z2);
                    let this = &mut self.data[linearize(position1)];
                    let that = &mut other.data[linearize(position2)];

                    if this.material.is_face_culled() && that.material.is_face_culled() {
                        this.remove_faces(Faces::from(shared_face.inverse()));
                        that.remove_faces(Faces::from(shared_face));

                        self.dirtied_positions.push(position1);
                        other.dirtied_positions.push(position2);
                    }
                }
            }
        }
    }

    pub fn set(&mut self, position: vec3u5, new_material: Option<Material>) {
        let i = linearize(position);
        let old_material = self.data[i].material;

        self.data[i].material = new_material;

        if !old_material.is_face_culled() && new_material.is_face_culled() {
            let missing = self.data[i].faces().not();
            self.data[i].set_opaque(Faces::all());
            self.remove_neighboring_faces(missing, position);
        }

        if old_material.is_face_culled() && !new_material.is_face_culled() {
            let present = self.data[i].faces();
            self.data[i].set_opaque(Faces::empty());
            self.add_neighboring_faces(present, position);
        }

        self.dirtied_positions.push(position);
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, position: vec3u5) {
        let i = linearize(position);
        faces
            .map(|f| (f, f.into_vec3()))
            .map(|(f, v)| (f, position.cast::<i32>().unwrap() + v))
            //.filter(|(_, x)| in_bounds(*x))
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u5::try_new(v.x, v.y, v.z).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.cast().unwrap().linearize(LENGTH);
                self.dirtied_positions.push(v);

                if self.data[index].material.is_face_culled() {
                    self.data[i].remove_faces(Faces::from(f));
                }

                self.data[index].remove_faces(Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, position: vec3u5) -> Vec<(Face, Vec3<i32>)> {
        let position = position.cast::<i32>().unwrap();

        let (in_chunk, out_chunk) = faces.not()
            .map(|f| (f, f.into_vec3() + position))
            .partition::<Vec<_>, _>(|&(_, position)| in_bounds(position));

        for (f, Vec3 { x, y, z }) in in_chunk {
            let position = vec3u5::new(x as u8, y as u8, z as u8);
            let index = linearize(position);

            self.dirtied_positions.push(position);
            self.data[index].insert_faces(Faces::from(f.inverse()));
        }

        out_chunk
    }

    fn send_update(&mut self) {
        if self.dirtied_positions.is_empty() {
            return;
        }

        let mut cubes = TinyVec::new();
        for position in self.dirtied_positions.drain(..) {
            cubes.push((position, self.data[linearize(position)]));
        }

        if let Err(e) = self.sender.try_send(ChunkUpdate { cubes }) {
            eprintln!("Failed to send chunk update: {:?}", e);
        }
    }

    pub fn tick(&mut self) {
        self.send_update();
    }
}

fn in_bounds(position: vec3i) -> bool {
    position.x >= 0 && position.x < LENGTH as i32
        && position.y >= 0 && position.y < LENGTH as i32
        && position.z >= 0 && position.z < LENGTH as i32
}

fn linearize(position: vec3u5) -> usize {
    position.cast().unwrap().linearize(LENGTH)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPosition {
    pub chunk: vec3i,
    pub local: vec3u5,
}

impl From<CubePosition> for ChunkLocalPosition {
    fn from(pos: CubePosition) -> Self {
        ChunkLocalPosition {
            chunk: pos.0 >> 5,
            local: pos.0.bitand(LENGTH as i32 - 1).into()
        }
    }
}

pub struct ChunkUpdate {
    pub cubes: TinyVec<[(vec3u5, Cube<Option<Material>>); 256]>,
}