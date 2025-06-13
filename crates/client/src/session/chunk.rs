use crate::gpu::binding::Payload;
use crate::gpu::geometry::InstanceBuffer;
use crate::gpu::Gpu;
use crate::state3d::{Instance3d, Instance3dPayload};
use crossbeam::channel::Receiver;
use game::chunk;
use game::chunk::channel::{ChunkUpdate, ClientChunkChannel};
use game::chunk::cube::Cube;
use game::chunk::material::Material;
use math::color::Rgba;
use math::vector::{vec2f, vec3i, vec3u4};
use std::collections::HashMap;
use std::ops::Mul;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SessionChunk {
    position: vec3i,
    data: Box<[Cube<Option<Material>>; chunk::SIZE]>,
    receiver: Receiver<ChunkUpdate>,
    mesh: InstanceBuffer,
    render_flag: Arc<AtomicBool>,
}

impl SessionChunk {
    pub fn create(pos: vec3i, gpu: &Gpu, receiver: Receiver<ChunkUpdate>, render_flag: Arc<AtomicBool>) -> Self {
        Self {
            position: pos,
            data: Box::new([Cube::new(None); chunk::SIZE]),
            receiver,
            mesh: InstanceBuffer::create::<Instance3dPayload>(gpu, &[]),
            render_flag,
        }
    }

    pub fn update(&mut self, gpu: &Gpu, texture_positions: &[(vec2f, f32)]) {
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

        let chunk_pos = self.position.mul(chunk::LENGTH as i32).cast::<f64>().unwrap();

        let mut instances = vec![];
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    let pos = vec3u4::new(x as u8, y as u8, z as u8);
                    let cube = self.data[pos.linearize()];
                    if let Some(material) = cube.material {
                        for face in cube.faces().variant_iter() {
                            let (tex_pos, tex_size) = texture_positions[material.texture_index(face)];

                            instances.push(Instance3d {
                                position: chunk_pos + pos.cast().unwrap(),
                                rotation: face.into_quat(),
                                texture_position: tex_pos,
                                texture_size: tex_size,
                                color: Rgba::TRANSPARENT,
                            }.payload());
                        }
                    }
                }
            }
        }

        self.mesh.write(gpu, &instances);
    }

    fn is_rendered(&self) -> bool { 
        let game_flag = self.render_flag.load(Ordering::Relaxed);
        
        // Client-side culling...
        
        game_flag
    }
}

pub struct SessionChunkMap {
    map: HashMap<vec3i, SessionChunk>,
    channel: ClientChunkChannel,
}

impl SessionChunkMap {
    pub fn new(channel: ClientChunkChannel) -> Self {
        Self {
            map: HashMap::new(),
            channel,
        }
    }

    pub fn insert(&mut self, chunk: SessionChunk) {
        self.map.insert(chunk.position, chunk);
    }

    pub fn remove(&mut self, pos: vec3i) {
        self.map.remove(&pos);
    }

    pub fn update(&mut self, gpu: &Gpu, texture_positions: &[(vec2f, f32)]) {
        while let Some((pos, receiver, render_flag)) = self.channel.recv_load() {
            self.map
                .insert(pos, SessionChunk::create(pos, gpu, receiver, render_flag));
        }

        while let Some(pos) = self.channel.recv_unload() {
            self.map.remove(&pos);
        }

        for chunk in self.map.values_mut() {
            chunk.update(gpu, texture_positions);
        }
    }

    pub fn meshes(&self) -> impl Iterator<Item = (vec3i, &InstanceBuffer)> {
        self.map.values().filter(|x| x.is_rendered()).map(|x| (x.position, &x.mesh))
    }
}
