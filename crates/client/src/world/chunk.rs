use std::ops::Mul;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossbeam::channel::Receiver;
use engine::video::gpu::AtlasTextureCoord;
use engine::video::v3d::GrowBuffer3d;
use engine::video::{gpu, v3d};
use game::chunk;
use game::chunk::channel::ChunkUpdate;
use game::chunk::cube::Cube;
use game::chunk::material::Material;
use math::color::Rgba;
use math::vector::{vec3i, vec3u4};
use wgpu::BufferUsages;

use crate::world::player;

/// The render-side representation of a chunk within the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of this chunk in chunk space.
    position: vec3i,
    /// An array of cubes that make up this chunk, linearized in YXZ-order in ascending power.
    data: Box<[Cube<Option<Material>>; chunk::SIZE]>,
    /// The channel used to receive updates of this chunk from its corresponding logic-side chunk.
    receiver: Receiver<ChunkUpdate>,
    /// The GPU buffer that holds the mesh data for this chunk.
    mesh: GrowBuffer3d,
    /// A flag that indicates whether this chunk should be rendered according to the logic-side game.
    render_flag: Arc<AtomicBool>,
    /// An instance cache used during remeshing to avoid excessive allocations.
    cached_quad_instances: Vec<v3d::InstancePayload>,
}

impl Chunk {
    /// Creates and allocates a full chunk at the given position filled with air.
    pub fn create(handle: &gpu::Handle, position: vec3i, receiver: Receiver<ChunkUpdate>, render_flag: Arc<AtomicBool>) -> Self {
        Self {
            position,
            data: Box::new([Cube::new(None); chunk::SIZE]),
            receiver,
            mesh: GrowBuffer3d::empty(handle, BufferUsages::VERTEX | BufferUsages::COPY_DST),
            render_flag,
            cached_quad_instances: vec![],
        }
    }

    /// Renders this chunk using the provided camera and drawing context.
    ///
    /// # Panics
    ///
    /// This function assumes that there is a unit-size quad mesh (e.g., see [gpu::mesh::quad]) already loaded in the drawing context. If no mesh is loaded,
    /// the function will result in a panic.
    pub fn render(&self, camera: &player::Camera, drawing: &mut v3d::Drawing) {
        // Offset the chunk position by the camera's chunk position because of camera-relative rendering.
        let chunk = self.position - camera.chunk_position;

        // If the chunk is not within the camera's frustum, don't render it.
        if !camera.frustum.contains_cube(chunk.cast().unwrap(), chunk::LENGTH as f32) {
            return;
        }

        // If the chunk is culled by the logic-side game, don't render it.
        if !self.render_flag.load(Ordering::Relaxed) {
            return;
        }

        drawing.draw(&self.mesh);
    }

    /// Regenerates the mesh for this chunk if the logic-side chunk has sent updates.
    pub fn update(&mut self, handle: &gpu::Handle, textures: &[AtlasTextureCoord]) {
        let not_updated = self.receiver.is_empty();
        while let Ok(update) = self.receiver.try_recv() {
            for (pos, cube) in update.overwrites {
                self.data[pos.linearize()] = cube;
            }
        }

        if not_updated {
            return;
        }

        // Converts the chunk position from chunk space to world space.
        let chunk_position = self.position.mul(chunk::LENGTH as i32).cast::<f64>().unwrap();

        // Clear the instance cache and remesh the chunk.
        self.cached_quad_instances.clear();
        for x in 0..chunk::LENGTH {
            for z in 0..chunk::LENGTH {
                for y in 0..chunk::LENGTH {
                    let position = vec3u4::new(x as u8, y as u8, z as u8);
                    let cube = self.data[position.linearize()];

                    if let Some(material) = cube.material {
                        for face in cube.faces().variant_iter() {
                            let texture_coord = textures[material.texture_index(face)];

                            self.cached_quad_instances.push(
                                v3d::Instance {
                                    position: chunk_position + position.cast().unwrap(),
                                    rotation: face.to_rotation(),
                                    texture_coord,
                                    color: Rgba::TRANSPARENT,
                                }
                                .payload(),
                            );
                        }
                    }
                }
            }
        }

        // Write the quad instances to the GPU buffer.
        self.mesh.write(handle, &self.cached_quad_instances);
    }
}
