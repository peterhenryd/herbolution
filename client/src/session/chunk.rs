use crossbeam::channel::Receiver;
use engine::gpu::handle::Handle;
use engine::gpu::mem::model::InstanceGroup;
use engine::gpu::mem::payload::ShaderPayload;
use engine::renderer_3d::vertex::Instance;
use engine::renderer_3d::vertex::InstanceShaderPayload;
use game::world::chunk;
use game::world::chunk::cube::Cube;
use game::world::chunk::material::Material;
use game::world::chunk::ChunkUpdate;
use math::color::Rgba;
use math::vector::{vec3i, vec3u5};
use std::collections::HashMap;
use std::ops::Mul;

pub struct SessionChunk {
    position: vec3i,
    mesh: InstanceGroup,
    data: Box<[Cube<Option<Material>>; chunk::SIZE]>,
    receiver: Receiver<ChunkUpdate>
}

impl SessionChunk {
    pub fn create(position: vec3i, handle: &Handle, receiver: Receiver<ChunkUpdate>) -> Self {
        Self {
            position,
            mesh: InstanceGroup::create::<InstanceShaderPayload>(handle, &[]),
            data: Box::new([Cube::new(None); chunk::SIZE]),
            receiver,
        }
    }

    pub fn update(&mut self, handle: &Handle) {
        let mut is_dirty = false;

        while let Ok(update) = self.receiver.try_recv() {
            is_dirty = true;
            for (position, cube) in update.overwrites {
                let index = chunk::linearize(position);
                self.data[index] = cube;
            }
        }

        if is_dirty {
            let chunk_position = self.position.mul(32).cast::<f32>().unwrap();
            let mut instances = vec![];

            for x in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    for z in 0..chunk::LENGTH {
                        let position = vec3u5::new(x as u8, y as u8, z as u8);
                        let cube = self.data[chunk::linearize(position)];
                        if let Some(material) = cube.material {
                            for face in cube.faces() {
                                instances.push(Instance {
                                    position: chunk_position + position.cast().unwrap(),
                                    rotation: face.variant().into_quat(),
                                    texture_index: material.texture_index(face.variant()),
                                    color: Rgba::TRANSPARENT,
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
}

impl SessionChunkMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, chunk: SessionChunk) {
        self.map.insert(chunk.position, chunk);
    }

    pub fn remove(&mut self, position: vec3i) {
        self.map.remove(&position);
    }

    pub fn update(&mut self, handle: &Handle) {
        for chunk in self.map.values_mut() {
            chunk.update(handle);
        }
    }

    pub fn meshes(&self) -> impl Iterator<Item = (vec3i, &InstanceGroup)> {
        self.map.values().map(|x| (x.position, &x.mesh))
    }
}