use crossbeam::channel::{Receiver, TryRecvError};
use engine::gpu::handle::Handle;
use engine::gpu::mem::model::InstanceGroup;
use engine::gpu::mem::payload::ShaderPayload;
use engine::renderer_3d::vertex::Instance;
use engine::renderer_3d::vertex::InstanceShaderPayload;
use game::world::chunk;
use game::world::chunk::{ChunkUpdate, Cube};
use math::color::Rgba;
use math::vector::{vec3i, Vec3};
use std::collections::HashMap;
use std::convert::identity;
use std::ops::Mul;

pub struct SessionChunk {
    position: vec3i,
    mesh: InstanceGroup,
    data: Box<[Cube<>; chunk::SIZE]>,
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

    pub fn update(&mut self, handle: &Handle) -> bool {
        let mut is_dirty = false;

        loop {
            let update;
            match self.receiver.try_recv() {
                Ok(x) => update = x,
                Err(TryRecvError::Disconnected) => return false,
                Err(TryRecvError::Empty) => break,
            }


            is_dirty = true;
            for (position, cube) in update.overwrites {
                let index = chunk::linearize(position);
                self.data[index] = cube;
            }
        }

        if is_dirty {
            let chunk_position = self.position.mul(32);
            let mut instances = vec![];

            for x in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    for z in 0..chunk::LENGTH {
                        let position = Vec3::new(x, y, z);
                        let cube = self.data[position.linearize(chunk::LENGTH)];
                        if let Some(material) = cube.material {
                            for face in cube.faces().map(identity) {
                                instances.push(Instance {
                                    position: (chunk_position + position.cast().unwrap()).cast().unwrap(),
                                    rotation: face.into_quat(),
                                    texture_index: material.texture_index(face),
                                    color: Rgba::TRANSPARENT,
                                }.payload());
                            }
                        }
                    }
                }
            }

            self.mesh.write(&handle, &instances);
        }

        true
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
        let mut to_remove = vec![];

        for (position, chunk) in self.map.iter_mut() {
            if !chunk.update(handle) {
                to_remove.push(*position);
            }
        }

        for position in to_remove {
            self.map.remove(&position);
        }
    }

    pub fn meshes(&self) -> impl Iterator<Item = (vec3i, &InstanceGroup)> {
        self.map.values().map(|x| (x.position, &x.mesh))
    }
}