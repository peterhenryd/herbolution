use crossbeam::channel::Receiver;
use engine::gpu::handle::Handle;
use engine::gpu::mem::model::InstanceGroup;
use engine::gpu::mem::payload::ShaderPayload;
use engine::renderer_3d::vertex::Instance;
use engine::renderer_3d::vertex::InstanceShaderPayload;
use game::world::chunk;
use game::world::chunk::channel::ClientChunkChannel;
use game::world::chunk::cube::Cube;
use game::world::chunk::material::Material;
use game::world::chunk::ChunkUpdate;
use math::color::Rgba;
use math::vector::{vec3i, vec3u5};
use std::collections::HashMap;
use std::ops::Mul;

pub struct SessionChunk {
    pos: vec3i,
    mesh: InstanceGroup,
    data: Box<[Cube<Option<Material>>; chunk::SIZE]>,
    receiver: Receiver<ChunkUpdate>
}

impl SessionChunk {
    pub fn create(pos: vec3i, handle: &Handle, receiver: Receiver<ChunkUpdate>) -> Self {
        Self {
            pos,
            mesh: InstanceGroup::create::<InstanceShaderPayload>(handle, &[]),
            data: Box::new([Cube::new(None); chunk::SIZE]),
            receiver,
        }
    }

    pub fn update(&mut self, handle: &Handle) {
        let mut is_dirty = false;

        while let Ok(update) = self.receiver.try_recv() {
            is_dirty = true;
            for (pos, cube) in update.overwrites {
                let index = chunk::linearize(pos);
                self.data[index] = cube;
            }
        }

        if is_dirty {
            let chunk_pos = self.pos.mul(32).cast::<f32>().unwrap();
            let mut instances = vec![];

            for x in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    for z in 0..chunk::LENGTH {
                        let pos = vec3u5::new(x as u8, y as u8, z as u8);
                        let cube = self.data[chunk::linearize(pos)];
                        if let Some(material) = cube.material {
                            for face in cube.faces() {
                                instances.push(Instance {
                                    pos: chunk_pos + pos.cast().unwrap(),
                                    quat: face.variant().into_quat(),
                                    texture_index: material.texture_index(face.variant()),
                                    color: Rgba::TRANSPARENT,
                                    is_lit: true,
                                }.payload());
                            }
                        }
                    }
                }
            }

            self.mesh.write(handle, &instances);
        }
    }
}

pub struct SessionChunkMap {
    map: HashMap<vec3i, SessionChunk>,
    channel: ClientChunkChannel,
}

impl SessionChunkMap {
    pub fn new(channel: ClientChunkChannel) -> Self {
        Self { map: HashMap::new(), channel }
    }

    pub fn insert(&mut self, chunk: SessionChunk) {
        self.map.insert(chunk.pos, chunk);
    }

    pub fn remove(&mut self, pos: vec3i) {
        self.map.remove(&pos);
    }

    pub fn update(&mut self, handle: &Handle) {
        while let Some((pos, receiver)) = self.channel.recv_load() {
            self.map.insert(pos, SessionChunk::create(pos, handle, receiver));
        }

        while let Some(pos) = self.channel.recv_unload() {
            self.map.remove(&pos);
        }

        for chunk in self.map.values_mut() {
            chunk.update(handle);
        }
    }

    pub fn meshes(&self) -> impl Iterator<Item = (vec3i, &InstanceGroup)> {
        self.map.values().map(|x| (x.pos, &x.mesh))
    }
}