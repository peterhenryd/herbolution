use crate::world::chunk::cube::{Cube, CubePosition};
use crate::world::chunk::material::{Material, OptionMaterialExt};
use hashbrown::HashMap;
use lib::geometry::cuboid::face::{Face, Faces};
use lib::geometry::InstanceShaderPayload;
use math::color::Rgba;
use math::matrix::Mat4;
use math::vector::{vec3i, vec3u5, Vec3};
use std::iter::StepBy;
use std::ops::{Mul, Not, Range};
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
                    let this = &mut self.data[x1 + y1 + z1];
                    let that = &mut other.data[x2 + y2 + z2];

                    self.dirtied_positions.push(vec3u5::new(x1 as u8, (y1  / LENGTH) as u8, (z1 / LENGTH.pow(2)) as u8));
                    other.dirtied_positions.push(vec3u5::new(x2 as u8, (y2 / LENGTH) as u8, (z2 / LENGTH.pow(2)) as u8));

                    if this.material.is_face_culled() && that.material.is_face_culled() {
                        this.remove_faces(Faces::from(shared_face.inverse()));
                        that.remove_faces(Faces::from(shared_face));
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
            .map(|f| (f, f.into_vec3i()))
            .into_iter()
            .map(|(f, v)| (f, position.cast::<i32>().unwrap() + v))
            .filter(|(_, x)| in_bounds(*x))
            .map(|(f, v)| (f, v.cast::<u8>().unwrap()))
            .for_each(|(f, v)| {
                let index = v.cast().unwrap().linearize(LENGTH);
                self.dirtied_positions.push(vec3u5::new(v.x, v.y, v.z));

                if self.data[index].material.is_face_culled() {
                    self.data[i].remove_faces(Faces::from(f));
                }

                self.data[index].remove_faces(Faces::from(f.inverse()));
            });

        self.dirtied_positions.push(position);
    }

    fn add_neighboring_faces(&mut self, faces: Faces, position: vec3u5) {
        faces
            // Get the faces that are not present on the cube.
            .not()
            // Get the corresponding cube offset vectors for each face
            .map(|f| (f, f.into_vec3i()))
            .into_iter()
            // Offset vectors by cube local position
            .map(|(f, v)| (f, position.cast::<i32>().unwrap() + v))
            // Filter positions that exist outside the current chunk
            // TODO: these should be returned to the caller so they can be used to cull faces on
            // neighboring chunks.
            .filter(|&(_, x)| in_bounds(x))
            .map(|(f, v)| (f, v.cast::<u8>().unwrap()))
            // Add the inverse of the removed face to the neighboring cube.
            .for_each(|(f, v)| {
                let i = v.cast().unwrap().linearize(LENGTH);
                self.dirtied_positions.push(vec3u5::new(v.x, v.y, v.z));
                self.data[i].insert_faces(Faces::from(f.inverse()));
            });
    }

    fn add_quads(&self, quads: &mut Vec<InstanceShaderPayload>, position: vec3u5) {
        let i = linearize(position);
        let cube = &self.data[i];
        let Some(material) = cube.material else { return };

        let chunk_position = self.position.mul(LENGTH as i32).cast::<f32>().unwrap();
        let position = position.cast::<f32>().unwrap();
        for rotation in cube.faces().map(|f| f.into_quat()) {
            quads.push(InstanceShaderPayload {
                model_matrix: Mat4::from_translation(position + chunk_position) * Mat4::from(rotation),
                texture_index: material.texture_index(),
                color: Rgba::TRANSPARENT,
            });
        }
    }

    pub fn get_quad_instances(&self) -> Vec<InstanceShaderPayload> {
        let mut vec = vec![];

        for x in 0..LENGTH as u8 {
            for y in 0..LENGTH as u8 {
                for z in 0..LENGTH as u8 {
                    self.add_quads(&mut vec, vec3u5::new(x, y, z));
                }
            }
        }

        vec
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

fn facial_chunk_boundary_slice(face: Face) -> Vec3<StepBy<Range<usize>>> {
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
}

fn in_bounds(Vec3 { x, y, z }: vec3i) -> bool {
    x >= 0 && y >= 0 && z >= 0 && x < LENGTH as i32 && y < LENGTH as i32 && z < LENGTH as i32
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

        let local_x = pos.0.x.rem_euclid(LENGTH as i32) as u8;
        let local_y = pos.0.y.rem_euclid(LENGTH as i32) as u8;
        let local_z = pos.0.z.rem_euclid(LENGTH as i32) as u8;

        ChunkLocalPosition {
            chunk: Vec3::new(chunk_x, chunk_y, chunk_z),
            local: vec3u5::new(local_x, local_y, local_z),
        }
    }
}

pub struct ChunkUpdate {
    pub cubes: HashMap<vec3u5, Cube<Option<Material>>>,
}