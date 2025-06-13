use crossbeam::channel::Receiver;
use engine::video;
use engine::video::mem::texture::AtlasTextureCoord;
use engine::video::r3d::{GrowBuffer3d, Instance3d, Instance3dPayload};
use game::chunk;
use game::chunk::channel::ChunkUpdate;
use game::chunk::cube::Cube;
use game::chunk::material::Material;
use math::color::Rgba;
use math::vector::{vec3f, vec3i, vec3u4, Vec3};
use std::ops::Mul;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use wgpu::BufferUsages;
use lib::geo::plane::Plane;
use math::matrix::{mat4f, Mat4};
use crate::player;

pub struct Chunk {
    position: vec3i,
    data: Box<[Cube<Option<Material>>; chunk::SIZE]>,
    receiver: Receiver<ChunkUpdate>,
    mesh: GrowBuffer3d,
    render_flag: Arc<AtomicBool>,
    instances: Vec<Instance3dPayload>,
}

impl Chunk {
    pub fn create(handle: &video::Handle, position: vec3i, receiver: Receiver<ChunkUpdate>, render_flag: Arc<AtomicBool>) -> Self {
        Self {
            position,
            data: Box::new([Cube::new(None); chunk::SIZE]),
            receiver,
            mesh: GrowBuffer3d::empty(handle, BufferUsages::VERTEX | BufferUsages::COPY_DST),
            render_flag,
            instances: vec![],
        }
    }
    
    pub fn render(&self, camera: &player::Camera, frame: &mut video::Frame3d) {
        let chunk = self.position - camera.chunk_position;
        if !camera.frustum.contains_cube(chunk.cast().unwrap(), chunk::LENGTH as f32) {
            return;
        }
        if !self.render_flag.load(Ordering::Relaxed) {
            return;
        }
        
        frame.draw(&self.mesh);
    }

    pub fn update(&mut self, handle: &video::Handle, textures: &[AtlasTextureCoord]) {
        let mut is_dirty = false;

        while let Ok(update) = self.receiver.try_recv() {
            is_dirty = true;
            for (pos, cube) in update.overwrites {
                self.data[pos.linearize()] = cube;
            }
        }

        if !is_dirty {
            return;
        }

        let chunk_position = self.position.mul(chunk::LENGTH as i32).cast::<f64>().unwrap();

        self.instances.clear();
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    let position = vec3u4::new(x as u8, y as u8, z as u8);
                    let cube = self.data[position.linearize()];
                    if let Some(material) = cube.material {
                        for face in cube.faces().variant_iter() {
                            let texture_coord = textures[material.texture_index(face)];
                            
                            self.instances.push(Instance3d {
                                position: chunk_position + position.cast().unwrap(),
                                rotation: face.to_rotation(),
                                texture_coord,
                                color: Rgba::TRANSPARENT,
                            }.payload());
                        }
                    }
                }
            }
        }
        self.mesh.write(handle, &self.instances);
    }

    fn is_rendered(&self) -> bool { 
        let game_flag = self.render_flag.load(Ordering::Relaxed);
        
        // Client-side culling...
        
        game_flag
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frustum([Plane<f32>; 6]);

impl Frustum {
    pub fn new(view_proj: mat4f) -> Self {
        let Mat4 { x, y, z, w } = view_proj;

        let left = Plane::new(x.w + x.x, y.w + y.x, z.w + z.x, w.w + w.x);
        let right = Plane::new(x.w - x.x, y.w - y.x, z.w - z.x, w.w - w.x);
        let top = Plane::new(x.w - x.y, y.w - y.y, z.w - z.y, w.w - w.y);
        let bottom = Plane::new(x.w + x.y, y.w + y.y, z.w + z.y, w.w + w.y);
        let near = Plane::new(x.w + x.z, y.w + y.z, z.w + z.z, w.w + w.z);
        let far = Plane::new(x.w - x.z, y.w - y.z, z.w - z.z, w.w - w.z);

        Self([near, far, left, right, top, bottom].map(Plane::normalize))
    }

    pub fn contains_cube(&self, origin: vec3f, size: f32) -> bool {
        let origin = origin * size;
        let center = origin + Vec3::splat(size / 2.0);
        let radius = (size * size.sqrt()) / 2.0;

        for plane in self.0 {
            let dist = plane.a * center.x
                + plane.b * center.y
                + plane.c * center.z
                + plane.d;
            if dist < -radius {
                return false;
            }
        }

        true
    }
}
