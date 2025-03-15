use std::collections::HashMap;
use std::ops::Mul;
use tokio::sync::mpsc::Receiver;
use engine::gpu::handle::Handle;
use engine::gpu::mem::model::InstanceGroup;
use engine::gpu::mem::payload::ShaderPayload;
use engine::renderer_3d::vertex::Instance;
use game::world::chunk::ChunkUpdate;
use game::world::chunk::cube::Cube;
use game::world::chunk::material::Material;
use lib::geometry::cuboid::face::Face;
use lib::geometry::InstanceShaderPayload;
use math::color::Rgba;
use math::vector::{vec3i, vec3u5};

pub struct SessionChunk {
    position: vec3i,
    mesh: InstanceGroup,
    cubes: HashMap<vec3u5, Cube<Option<Material>>>,
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

        while let Ok(update) = self.receiver.try_recv() {
            if !update.cubes.is_empty() {
                is_dirty = true;
            } else {
                continue;
            }

            self.cubes.extend(update.cubes);
        }

        if is_dirty {
            let chunk_position = self.position.mul(32).cast::<f32>().unwrap();
            let mut instances = vec![];
            for (&pos, cube) in &self.cubes {
                let Some(material) = cube.material else { continue };

                for rotation in cube.faces().map(Face::into_quat) {
                    instances.push(Instance {
                        position: chunk_position + pos.cast().unwrap(),
                        rotation,
                        texture_index: material.texture_index(),
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