use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Mul;

use crate::player::PlayerCamera;
use crossbeam::channel::{bounded, Receiver, Sender};
use engine::sculptor::chisel::Chisel;
use engine::sculptor::{GrowBuffer3d, Instance3d};
use fastrand::Rng;
use game::chunk::cube::Cube;
use game::chunk::handle::{ChunkCube, GameChunkHandle};
use game::chunk::material::{Palette, PaletteCube};
use herbolution_math::spatial::face::PerFace;
use lib::chunk;
use lib::point::ChunkPt;
use math::vector::{vec3u5, Vec3};
use wgpu::BufferUsages;

/// The render-side representation of a chunk within the world.
#[derive(Debug)]
pub struct Chunk {
    /// The position of this chunk in chunk space.
    position: ChunkPt,
    /// An array of cubes that make up this chunk, linearized in YXZ-order in ascending power.
    /// The channel used to receive updates of this chunk from its corresponding behavior-side chunk.
    handle: GameChunkHandle,
    /// The GPU buffer that holds the mesh data for this chunk.
    /// A flag that indicates whether this chunk should be rendered according to the behavior-side herbolution_game.
    /// An instance cache used during remeshing to avoid excessive allocations.
    cqi_tx: Sender<MovableContext>,
    cqi_rx: Receiver<MovableContext>,
    context: Option<MovableContext>,
    mesh: GrowBuffer3d,
}

// TODO: this may not be the best way to multithread the chunk remeshing process
// instead of moving the entire context, we could just move the cached quad instances and the updated values in the mesh function, make it so the vector is
// linearized instead of sequential, and only remesh the chunk data that has been changed. this will remove the potential for stale mesh data to be rendered, as
// the current cloning of GrowBuffer3d is hacky due to the instance count not necessarily matching the instances in the buffer.
#[derive(Debug)]
struct MovableContext {
    position: ChunkPt,
    cached_quad_instances: Vec<Instance3d>,
    data: Box<[PaletteCube; chunk::VOLUME]>,
    mesh: GrowBuffer3d,
    palette: Palette,
}

impl Chunk {
    /// Creates and allocates a full chunk at the given position filled with air.
    pub fn create(gpu: &gpu::Handle, position: ChunkPt, handle: GameChunkHandle) -> Self {
        let (cqi_tx, cqi_rx) = bounded(1);

        let mesh = GrowBuffer3d::empty(gpu, BufferUsages::VERTEX | BufferUsages::COPY_DST);

        Self {
            position,
            handle,
            cqi_tx,
            cqi_rx,
            mesh: mesh.clone(),
            context: Some(MovableContext {
                position,
                cached_quad_instances: vec![],
                data: Box::new([Cube::new(None); chunk::VOLUME]),
                mesh,
                palette: Palette::new(),
            }),
        }
    }

    /// Renders this chunk using the provided camera and drawing context.
    ///
    /// # Panics
    ///
    /// This function assumes that there is a unit-size quad mesh (e.g., see [gpu::mesh::c_quad]) already loaded in the drawing context. If no mesh is
    /// loaded, the function will result in a panic.
    pub fn render(&self, camera: &PlayerCamera, chisel: &mut Chisel) {
        // Offset the chunk position by the camera's chunk position because of camera-relative rendering.
        let chunk = self.position.0 - camera.chunk_position;

        // If the chunk is not within the camera's frustum, don't render it.
        if !camera
            .frustum
            .contains_cube(chunk.cast(), chunk::LENGTH as f32)
        {
            return;
        }

        // If the chunk is culled by the behavior-side herbolution_game, don't render it.
        if !self.handle.is_rendered() {
            return;
        }

        chisel.render_each(&self.mesh);
    }

    /// Regenerates the mesh for this chunk if the behavior-side chunk has sent updates.
    pub fn update(&mut self, handle: &gpu::Handle) {
        if let Ok(context) = self.cqi_rx.try_recv() {
            self.mesh = context.mesh.clone();
            self.context = Some(context);
        }

        let not_updated = self.handle.cube_update.is_empty();
        if let Some(context) = self.context.as_mut() {
            while let Some(update) = self.handle.next_cube_update() {
                for ChunkCube { position, cube } in update.overwrites {
                    context.data[position.linearize()] = cube;
                }
            }

            while let Some(update) = self.handle.next_palette_update() {
                context.palette.insert(update.material);
            }
        };

        if not_updated {
            return;
        }

        let Some(mut context) = self.context.take() else {
            return;
        };

        let tx = self.cqi_tx.clone();
        let handle = handle.clone();

        let mut hasher = DefaultHasher::new();
        self.position.hash(&mut hasher);
        let mut rng = Rng::with_seed(hasher.finish());

        rayon::spawn(move || {
            // Converts the chunk position from chunk space to world space.
            let chunk_position = context
                .position
                .0
                .mul(chunk::LENGTH as i32)
                .cast::<f64>();

            // Clear the instance cache and remesh the chunk.
            context.cached_quad_instances.clear();

            let (mut cached_material, mut prev_material_id) = (None, None);
            for x in 0..chunk::LENGTH {
                for z in 0..chunk::LENGTH {
                    for y in 0..chunk::LENGTH {
                        let position = vec3u5::new(x as u8, y as u8, z as u8);
                        let cube = context.data[position.linearize()];

                        let perms = PerFace::mapped(|_| rng.f32());
                        if let Some(material) = cube.material {
                            if Some(material) != prev_material_id {
                                prev_material_id = Some(material);
                                cached_material = context.palette.get(material);
                            }

                            let Some(material) = &cached_material else {
                                return;
                            };

                            for face in cube.mesh.faces() {
                                let color = material.get_color(perms[face]);

                                context
                                    .cached_quad_instances
                                    .push(Instance3d::new(
                                        chunk_position + position.try_cast().unwrap(),
                                        face.to_rotation(),
                                        Vec3::ONE,
                                        color,
                                        0,
                                    ));
                            }
                        }
                    }
                }
            }

            // Write the quad instances to the GPU buffer.
            context
                .mesh
                .write(&handle, &context.cached_quad_instances);

            let _ = tx.send(context);
        });
    }
}
