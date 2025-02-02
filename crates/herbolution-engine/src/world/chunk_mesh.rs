use crate::engine::gpu::Gpu;
use crate::game::chunk::material::Material;
use crate::game::chunk::section::{CHUNK_SIZE, CHUNK_TOTAL};
use crate::math::vector::Vector3;
use crate::world::geometry::quad::Quad;
use bitflags::bitflags;
use glam::{EulerRot, IVec3, Quat, U8Vec3};
use std::sync::Arc;
use wgpu::{Buffer, BufferAddress, BufferDescriptor, BufferUsages, Queue, RenderPass};

pub struct ChunkMesh {
    queue: Arc<Queue>,
    pub position: IVec3,
    instance_buffer: Buffer,
    instance_count: u32,
    cube_face_array: [CubeFaces; CHUNK_TOTAL],
}

impl ChunkMesh {
    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..self.instance_count);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CubeFaces {
    material: Material,
    faces: Faces,
}

impl CubeFaces {
    pub const fn empty() -> Self {
        Self {
            material: Material::Air,
            faces: Faces::empty(),
        }
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    struct Faces: u8 {
        const FRONT =  0b00000001;
        const BACK =   0b00000010;
        const LEFT =   0b00000100;
        const RIGHT =  0b00001000;
        const TOP =    0b00010000;
        const BOTTOM = 0b00100000;
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Face {
    Front = 0,
    Back = 1,
    Left = 2,
    Right = 3,
    Top = 4,
    Bottom = 5,
}

impl Into<Faces> for Face {
    fn into(self) -> Faces {
        match self {
            Face::Front => Faces::FRONT,
            Face::Back => Faces::BACK,
            Face::Left => Faces::LEFT,
            Face::Right => Faces::RIGHT,
            Face::Top => Faces::TOP,
            Face::Bottom => Faces::BOTTOM,
        }
    }
}

impl Face {
    pub fn inverse(self) -> Face {
        match self {
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Left => Face::Right,
            Face::Right => Face::Left,
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
        }
    }

    pub fn into_quat(self) -> Quat {
        match self {
            Face::Front => Quat::from_euler(EulerRot::YXZ, -90f32.to_radians(), 0., 0.),
            Face::Back => Quat::from_euler(EulerRot::YXZ, 90f32.to_radians(), 0., 0.),
            Face::Left => Quat::from_euler(EulerRot::YXZ, 0., -90f32.to_radians(), 0.),
            Face::Right => Quat::from_euler(EulerRot::YXZ, 0., 90f32.to_radians(), 0.),
            Face::Top => Quat::from_euler(EulerRot::YXZ, 0., 0., 0.),
            Face::Bottom => Quat::from_euler(EulerRot::YXZ, 180f32.to_radians(), 0., 0.),
        }
    }
}

impl Faces {
    pub fn map<T>(self, f: impl Fn(Face) -> T) -> Vec<T> {
        let mut vec = vec![];

        if self.contains(Faces::FRONT) {
            vec.push(f(Face::Front));
        }
        if self.contains(Faces::BACK) {
            vec.push(f(Face::Back));
        }
        if self.contains(Faces::LEFT) {
            vec.push(f(Face::Left));
        }
        if self.contains(Faces::RIGHT) {
            vec.push(f(Face::Right));
        }
        if self.contains(Faces::TOP) {
            vec.push(f(Face::Top));
        }
        if self.contains(Faces::BOTTOM) {
            vec.push(f(Face::Bottom));
        }

        vec
    }
}

impl ChunkMesh {
    pub fn new(gpu: &Gpu, position: IVec3) -> Self {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: None,
            size: 1671168 as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            position,
            queue: gpu.queue.clone(),
            instance_buffer,
            instance_count: 0,
            cube_face_array: [CubeFaces::empty(); CHUNK_TOTAL],
        }
    }

    pub fn update(&mut self) {
        let mut instances = vec![];

        linearized_3d_loop(|position, i, adjacent,| {
            let &CubeFaces { material, faces } = &self.cube_face_array[i];

            for quat in faces.map(Face::into_quat) {
                let quad = Quad::new(position.to_f32(), quat, material.into_texture_index());
                instances.push(quad);
            }
        });

        self.queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
        self.instance_count = instances.len() as u32;
    }

    pub fn set_linear_cube(&mut self, i: usize, material: Material) {
        match material {
            Material::Air => self.cube_face_array[i].faces = Faces::empty(),
            _ => self.cube_face_array[i].faces = Faces::all(),
        }
        self.cube_face_array[i].material = material;
    }

    pub fn set_cube(&mut self, position: U8Vec3, material: Material) {
        let i = position.x as usize
            + position.y as usize * CHUNK_SIZE
            + position.z as usize * CHUNK_SIZE * CHUNK_SIZE;
        self.set_linear_cube(i, material);
    }
}

#[inline(always)]
fn linearized_3d_loop(mut f: impl FnMut(Vector3<usize>, usize, [Option<usize>; 6]) -> ()) {
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let vector = Vector3::new(x, y, z);
                let i = vector.linearize(CHUNK_SIZE);
                let adjacent = [
                    if z != 15 { Some(Vector3::new(x, y, z + 1).linearize(CHUNK_SIZE)) } else { None },
                    if z != 0 { Some(Vector3::new(x, y, z - 1).linearize(CHUNK_SIZE)) } else { None },
                    if y != 15 { Some(Vector3::new(x, y + 1, z).linearize(CHUNK_SIZE)) } else { None },
                    if y != 0 { Some(Vector3::new(x, y - 1, z).linearize(CHUNK_SIZE)) } else { None },
                    if x != 15 { Some(Vector3::new(x + 1, y, z).linearize(CHUNK_SIZE)) } else { None },
                    if x != 0 { Some(Vector3::new(x - 1, y, z).linearize(CHUNK_SIZE)) } else { None },
                ];
                f(vector, i, adjacent);
            }
        }
    }
}
