use engine::gpu::handle::Handle;
use engine::gpu::mem::model::InstanceGroup;
use engine::gpu::mem::payload::ShaderPayload;
use engine::renderer_3d::vertex::Instance;
use game::world::chunk::cube::Cube;
use game::world::chunk::material::Material;
use game::world::chunk::ChunkUpdate;
use engine::renderer_3d::vertex::InstanceShaderPayload;
use math::color::Rgba;
use math::vector::{vec3i, vec3u5};
use std::collections::HashMap;
use std::ops::Mul;
use kanal::Receiver;

pub struct SessionChunk {
    position: vec3i,
    mesh: InstanceGroup,
    cubes: HashMap<vec3u5, Cube<Material>>,
    receiver: Receiver<ChunkUpdate>
}

impl SessionChunk {
    pub fn create(position: vec3i, handle: &Handle, receiver: Receiver<ChunkUpdate>) -> Self {
        Self {
            position,
            mesh: InstanceGroup::create::<InstanceShaderPayload>(handle, &[]),
            cubes: HashMap::new(),
            receiver,
        }
    }

    pub fn update(&mut self, handle: &Handle) {
        let mut is_dirty = false;

        while let Ok(Some(update)) = self.receiver.try_recv() {
            for (position, cube) in update.cubes {
                is_dirty = true;

                let Some(material) = cube.material else {
                    self.cubes.remove(&position);
                    continue;
                };

                self.cubes.insert(position, Cube {
                    material,
                    dependent_data: cube.dependent_data,
                });
            }
        }

        if is_dirty {
            let chunk_position = self.position.mul(32).cast::<f32>().unwrap();
            let mut instances = vec![];
            for (&pos, cube) in &self.cubes {
                for face in cube.faces() {
                    instances.push(Instance {
                        position: chunk_position + pos.cast().unwrap(),
                        rotation: face.variant().into_quat(),
                        texture_index: cube.material.texture_index(),
                        color: Rgba::TRANSPARENT,
                    }.payload());
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

    pub fn update(&mut self, handle: &Handle) {
        for chunk in self.map.values_mut() {
            chunk.update(handle);
        }
    }

    pub fn meshes(&self) -> impl Iterator<Item = (vec3i, &InstanceGroup)> {
        self.map.values().map(|x| (x.position, &x.mesh))
    }
}