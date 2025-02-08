use crate::engine::gpu::Gpu;
use crate::world::chunk::material::Material;
use math::vector::{vec3, vec3i, vec3u8};
use std::ops::Not;
use std::path::Path;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, RenderPass};
use crate::engine::as_no_uninit::AsNoUninit;
use crate::engine::geometry::cube::Faces;
use crate::engine::geometry::instance::{ArrInstance, Instance};

pub mod material;
pub mod map;
pub mod generator;

pub const LENGTH: usize = 32;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk {
    position: vec3i,
    instance_buffer: Buffer,
    instance_count: u32,
    data: [CubeData; SIZE],
    has_changed: bool,
}

impl Chunk {
    pub fn new(gpu: &Gpu, position: vec3i) -> Self {
        let instance_buffer = gpu.device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &[0u8; 1024],
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            position,
            instance_buffer,
            instance_count: 0,
            data: [CubeData {
                material: Material::Air,
                faces: Faces::empty(),
            }; SIZE],
            has_changed: false,
        }
    }

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
            dbg!(p);
            dbg!(present);
            self.add_neighboring_faces(present, p);
        }

        self.has_changed = true;
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, p: vec3u8) {
        let i = self.get_index(p);
        faces.map(|f| (f, f.into_vec3i()))
            .into_iter()
            .map(|(f, v)| (f, p.cast::<i32>() + v))
            .filter(|(_, v)| v.x >= 0 && v.y >= 0 && v.z >= 0 && v.x < LENGTH as i32 && v.y < LENGTH as i32 && v.z < LENGTH as i32)
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
        faces.not()
            .map(|f| (f, f.into_vec3i()))
            .into_iter()
            .map(|(f, v)| (f, p.cast::<i32>() + v))
            .filter(|(_, v)| v.x >= 0 && v.y >= 0 && v.z >= 0 && v.x < LENGTH as i32 && v.y < LENGTH as i32 && v.z < LENGTH as i32)
            .map(|(f, v)| (f, v.cast::<u8>()))
            .for_each(|(f, v)| {
                self.data[self.get_index(v)].faces.insert(Faces::from(f.inverse()));
            });
    }

    pub(crate) fn update(&mut self, gpu: &Gpu) {
        if !self.has_changed {
            return;
        }

        self.has_changed = false;

        let mut quads = vec![];
        for x in 0..LENGTH {
            for y in 0..LENGTH {
                for z in 0..LENGTH {
                    self.add_quads(&mut quads, vec3::new(x, y, z).cast());
                }
            }
        }

        self.instance_count = quads.len() as u32;

        if self.instance_buffer.size() < (quads.len() * size_of::<ArrInstance>()) as u64 {
            self.instance_buffer = gpu.device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&quads),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
        } else {
            gpu.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&quads));
        }
    }

    fn add_quads(&self, quads: &mut Vec<ArrInstance>, p: vec3u8) {
        let i = self.get_index(p);
        let cube = &self.data[i];

        if cube.material == Material::Air {
            return;
        }

        let chunk_position = self.position.cast::<f32>() * 32.;
        let position = p.cast();
        for rotation in cube.faces.map(|f| f.into_quat()) {
            quads.push(Instance {
                position: chunk_position + position,
                rotation,
                texture_index: cube.material.get_texture_index(),
            }.as_no_uninit());
        }
    }

    pub async fn load(&mut self, _: impl AsRef<Path>) -> std::io::Result<()> {
        todo!()
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..self.instance_count);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CubeData {
    material: Material,
    faces: Faces,
}
