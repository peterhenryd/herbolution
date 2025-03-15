use crate::world::chunk::cube::{Cube, CubePosition};
use crate::world::chunk::material::{Material, OptionMaterialExt};
use hashbrown::HashMap;
use lib::geometry::cuboid::face::{Face, Faces};
use math::vector::{vec3i, vec3u5, Vec3};
use std::ops::{Not, Range};
use tokio::sync::mpsc::Sender;

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
        let this_matrix = facial_chunk_boundary_slice(shared_face.inverse());
        let that_matrix = facial_chunk_boundary_slice(shared_face);

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

    fn add_neighboring_faces(&mut self, faces: Faces, position: vec3u5) {
        let position = position.cast::<i32>().unwrap();
        faces
            // Get the faces that are not present on the cube.
            .not()
            // Get the corresponding cube offset vectors for each face
            .map(|f| (f, f.into_vec3() + position))
            // Filter positions that exist outside the current chunk
            // TODO: these should be returned to the caller so they can be used to cull faces on
            // neighboring chunks.
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u5::try_new(v.x, v.y, v.z).map(|x| (f, x)))
            // Add the inverse of the removed face to the neighboring cube.
            .for_each(|(f, v)| {
                let i = v.cast().unwrap().linearize(LENGTH);
                self.dirtied_positions.push(v);
                self.data[i].insert_faces(Faces::from(f.inverse()));
            });
    }

    fn send_update(&mut self) {
        if self.dirtied_positions.is_empty() {
            return;
        }

        let mut cubes = HashMap::with_capacity(self.dirtied_positions.len());
        for i in self.dirtied_positions.drain(..) {
            cubes.insert(i, self.data[linearize(i)]);
        }

        if let Err(e) = self.sender.try_send(ChunkUpdate { cubes }) {
            eprintln!("Failed to send chunk update: {:?}", e);
        }
    }

    pub fn tick(&mut self) {
        self.send_update();
    }
}

fn facial_chunk_boundary_slice(face: Face) -> Vec3<Range<u8>> {
    // Recreate the commented section except don't linearize
    match face {
        Face::Top => Vec3::new(
            0..LENGTH as u8,
            (LENGTH as u8 - 1)..LENGTH as u8,
            0..LENGTH as u8,
        ),
        Face::Bottom => Vec3::new(
            0..LENGTH as u8,
            0..1,
            0..LENGTH as u8,
        ),
        Face::Left => Vec3::new(
            0..1,
            0..LENGTH as u8,
            0..LENGTH as u8,
        ),
        Face::Right => Vec3::new(
            (LENGTH as u8 - 1)..LENGTH as u8,
            0..LENGTH as u8,
            0..LENGTH as u8,
        ),
        Face::Front => Vec3::new(
            0..LENGTH as u8,
            0..LENGTH as u8,
            (LENGTH as u8 - 1)..LENGTH as u8,
        ),
        Face::Back => Vec3::new(
            0..LENGTH as u8,
            0..LENGTH as u8,
            0..1,
        ),
    }
    
        /*
    let full_x = (0..LENGTH).step_by(1);
    let full_y = (0..LENGTH.pow(2)).step_by(LENGTH);
    let full_z = (0..LENGTH.pow(3)).step_by(LENGTH.pow(2));

    match face {
        Face::Top => Vec3::new(
            full_x.clone(),
            ((LENGTH - 1) * LENGTH..LENGTH * LENGTH).step_by(LENGTH),
            full_z.clone(),
        ),
        Face::Bottom => Vec3::new(full_x.clone(), (0..LENGTH).step_by(LENGTH), full_z.clone()),
        Face::Left => Vec3::new((0..1).step_by(1), full_y.clone(), full_z.clone()),
        Face::Right => Vec3::new(
            (LENGTH - 1..LENGTH).step_by(1),
            full_y.clone(),
            full_z.clone(),
        ),
        Face::Front => Vec3::new(
            full_x.clone(),
            full_y.clone(),
            ((LENGTH - 1) * LENGTH.pow(2)..LENGTH.pow(3)).step_by(LENGTH.pow(2)),
        ),
        Face::Back => Vec3::new(
            full_x.clone(),
            full_y.clone(),
            (0..LENGTH.pow(2)).step_by(LENGTH.pow(2)),
        ),
    }
    
         */
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
        let chunk_x = pos.0.x.div_euclid(LENGTH as i32);
        let chunk_y = pos.0.y.div_euclid(LENGTH as i32);
        let chunk_z = pos.0.z.div_euclid(LENGTH as i32);

        let local_x = (pos.0.x & (LENGTH as i32 - 1)) as u8;
        let local_y = (pos.0.y & (LENGTH as i32 - 1)) as u8;
        let local_z = (pos.0.z & (LENGTH as i32 - 1)) as u8;

        ChunkLocalPosition {
            chunk: Vec3::new(chunk_x, chunk_y, chunk_z),
            local: vec3u5::new(local_x, local_y, local_z),
        }
    }
}

pub struct ChunkUpdate {
    pub cubes: HashMap<vec3u5, Cube<Option<Material>>>,
}