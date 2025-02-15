use crate::engine::geometry::cuboid::{Face, Faces};
use crate::engine::geometry::instance::{ArrInstance, Instance};
use crate::engine::gpu::Gpu;
use crate::world::chunk::cube::Cube;
use crate::world::chunk::material::Material;
use crate::world::chunk::mesh::{ChunkMesh, InstanceMesh};
use math::to_no_uninit::ToNoUninit;
use math::vector::{vec3, vec3i, vec3u8};
use std::iter::StepBy;
use std::ops::{Not, Range};
use std::path::Path;
use wgpu::RenderPass;

pub mod cube;
pub mod material;
pub mod map;
pub mod generator;
pub mod mesh;

pub const LENGTH: usize = 32;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk<M> {
    pub position: vec3i,
    data: Box<[Cube; SIZE]>,
    mesh: M,
}

impl Chunk<()> {
    pub fn new(position: vec3i) -> Self {
        Self {
            position,
            data: Box::new([Cube::new(); SIZE]),
            mesh: (),
        }
    }

    pub fn into_meshed(self, gpu: &Gpu) -> Chunk<InstanceMesh> {
        Chunk {
            mesh: InstanceMesh::new(gpu, &self.get_quad_instances()),
            position: self.position,
            data: self.data,
        }
    }
}

impl<M: ChunkMesh> Chunk<M> {
    pub fn get(&self, p: vec3u8) -> Option<Material> {
        if p.x >= LENGTH as u8 || p.y >= LENGTH as u8 || p.z >= LENGTH as u8 {
            return None;
        }

        let i = self.get_index(p);
        if self.data[i].material == Material::Air {
            None
        } else {
            Some(self.data[i].material)
        }
    }

    // The basic idea here is to find the 32x32x1 slice of cubes on which faces are shared between two
    // chunks, and then to iterate over each cube and its corresponding cube, and cull their
    // shared faces if possible.
    pub fn cull_shared_faces<A: ChunkMesh>(&mut self, other: &mut Chunk<A>) {
        // The difference in position between the two chunks, this is a unit component vector.
        let dp = self.position - other.position;
        // Get the face that is shared between the two chunks.
        let Some(shared_face) = Face::from_vec3i(dp) else { return };

        // Get the slices of cubes that are shared between the two chunks.
        let this_matrix = self.get_facial_boundary_indices(shared_face.inverse());
        let that_matrix = other.get_facial_boundary_indices(shared_face);

        for (x1, x2) in this_matrix.x.zip(that_matrix.x) {
            for (y1, y2) in this_matrix.y.clone().zip(that_matrix.y.clone()) {
                for (z1, z2) in this_matrix.z.clone().zip(that_matrix.z.clone()) {
                    let this = &mut self.data[x1 + y1 + z1];
                    let that = &mut other.data[x2 + y2 + z2];

                    if this.material.is_face_culled() && that.material.is_face_culled() {
                        this.faces.remove(Faces::from(shared_face.inverse()));
                        that.faces.remove(Faces::from(shared_face));
                    }
                }
            }
        }
        self.mesh.schedule_update();
    }

    pub fn get_facial_boundary_indices(&self, face: Face) -> vec3<StepBy<Range<usize>>> {
        let full_x = (0..LENGTH).step_by(1);
        let full_y = (0..LENGTH.pow(2)).step_by(LENGTH);
        let full_z = (0..LENGTH.pow(3)).step_by(LENGTH.pow(2));

        match face {
            Face::Top => vec3::new(
                full_x.clone(),
                ((LENGTH - 1) * LENGTH..LENGTH * LENGTH).step_by(LENGTH),
                full_z.clone(),
            ),
            Face::Bottom => vec3::new(
                full_x.clone(),
                (0..LENGTH).step_by(LENGTH),
                full_z.clone(),
            ),
            Face::Left => vec3::new(
                (0..1).step_by(1),
                full_y.clone(),
                full_z.clone(),
            ),
            Face::Right => vec3::new(
                (LENGTH - 1..LENGTH).step_by(1),
                full_y.clone(),
                full_z.clone(),
            ),
            Face::Front => vec3::new(
                full_x.clone(),
                full_y.clone(),
                ((LENGTH - 1) * LENGTH.pow(2)..LENGTH.pow(3)).step_by(LENGTH.pow(2)),
            ),
            Face::Back => vec3::new(
                full_x.clone(),
                full_y.clone(),
                (0..LENGTH.pow(2)).step_by(LENGTH.pow(2)),
            )
        }
    }

    #[inline]
    fn get_index(&self, p: vec3u8) -> usize {
        p.x as usize + p.y as usize * LENGTH + p.z as usize * LENGTH * LENGTH
    }

    pub fn set(&mut self, p: vec3u8, new_material: Material) {
        if p.x >= LENGTH as u8 || p.x >= LENGTH as u8 || p.x >= LENGTH as u8 {
            return;
        }

        let i = self.get_index(p);
        let old_material = self.data[i].material;

        self.data[i].material = new_material;

        if !old_material.is_face_culled() && new_material.is_face_culled() {
            let missing = self.data[i].faces.not();
            self.data[i].faces = Faces::all();
            self.remove_neighboring_faces(missing, p);
        }

        if old_material.is_face_culled() && !new_material.is_face_culled() {
            let present = self.data[i].faces;
            self.data[i].faces = Faces::empty();
            self.add_neighboring_faces(present, p);
        }

        self.mesh.schedule_update();
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, p: vec3u8) {
        let i = self.get_index(p);
        faces.map(|f| (f, f.into_vec3i()))
            .into_iter()
            .map(|(f, v)| (f, p.cast::<i32>() + v))
            .filter(|(_, x)| in_bounds(*x))
            .map(|(f, v)| (f, v.cast::<u8>()))
            .for_each(|(f, v)| {
                let index = self.get_index(v);

                if self.data[index].material.is_face_culled() {
                    self.data[i].faces.remove(Faces::from(f));
                }

                self.data[self.get_index(v)].faces.remove(Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, p: vec3u8) {
        faces
            // Get the faces that are not present on the cube.
            .not()
            // Get the corresponding cube offset vectors for each face
            .map(|f| (f, f.into_vec3i()))
            .into_iter()
            // Offset vectors by cube local position
            .map(|(f, v)| (f, p.cast::<i32>() + v))
            // Filter positions that exist outside the current chunk
            // TODO: these should be returned to the caller so that thy can be used to cull faces on
            // neighboring chunks.
            .filter(|(_, x)| in_bounds(*x))
            .map(|(f, v)| (f, v.cast::<u8>()))
            // Add the inverse of the removed face to the neighboring cube.
            .for_each(|(f, v)| {
                self.data[self.get_index(v)].faces.insert(Faces::from(f.inverse()));
            });
    }

    pub(crate) fn get_quad_instances(&self) -> Vec<ArrInstance> {
        let mut vec = vec![];

        for x in 0..LENGTH {
            for y in 0..LENGTH {
                for z in 0..LENGTH {
                    self.add_quads(&mut vec, vec3::new(x, y, z).cast());
                }
            }
        }

        vec
    }

    fn add_quads(&self, quads: &mut Vec<ArrInstance>, p: vec3u8) {
        let i = self.get_index(p);
        let cube = &self.data[i];

        if cube.material == Material::Air {
            return;
        }

        let chunk_position = self.position.cast::<f32>() * LENGTH as f32;
        let position = p.cast();
        for (i, rotation) in cube.faces.map(|f| f.into_quat()).into_iter().enumerate() {
            quads.push(Instance {
                position: chunk_position + position,
                rotation,
                texture_index: cube.material.get_texture_index(),
                light_level_alpha: cube.light_levels[i].as_alpha() as f32 / 255.0,
            }.to_no_uninit());
        }
    }

    pub async fn load(&mut self, _: impl AsRef<Path>) -> std::io::Result<()> {
        todo!()
    }
}

impl Chunk<InstanceMesh> {
    pub fn update(&mut self, gpu: &Gpu) {
        if self.mesh.has_changed {
            self.mesh.update(gpu, &self.get_quad_instances());
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_vertex_buffer(1, self.mesh.instance_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..self.mesh.instance_count);
    }
}

fn in_bounds(vec3 { x, y, z }: vec3i) -> bool {
    x >= 0 && y >= 0 && z >= 0 && x < LENGTH as i32 && y < LENGTH as i32 && z < LENGTH as i32
}