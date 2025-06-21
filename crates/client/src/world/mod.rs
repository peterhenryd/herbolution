use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chunk::Chunk;
use game::chunk::channel::ClientChunkChannel;
use lib::TrackMut;
use math::vec::vec3i;
use rayon::{ThreadPool, ThreadPoolBuilder};
use engine::video::{sculptor, Video};
use engine::video::sculptor::Chisel;
use crate::mesh::MeshIds;

pub mod chunk;
mod frustum;
pub mod player;

/// The render-side representation of a world within the game.
#[derive(Debug)]
pub struct World {
    /// A map of loaded chunks, keyed by their position in chunk space.
    chunk_map: HashMap<vec3i, Chunk>,
    /// The channel used for communicating with the logic-side world.
    channel: ClientChunkChannel,
    /// The settings used by the fragment shader to render the world.
    pub(crate) render_settings: TrackMut<sculptor::World>,
    mesh_thread_pool: Rc<ThreadPool>,
}

impl World {
    /// Creates a new instance with the provided channel and render settings.
    pub fn new(channel: ClientChunkChannel, render_settings: sculptor::World) -> Self {
        Self {
            chunk_map: HashMap::new(),
            channel,
            render_settings: render_settings.into(),
            mesh_thread_pool: Rc::new(
                ThreadPoolBuilder::new()
                    .num_threads(num_cpus::get())
                    .build()
                    .unwrap(),
            ),
        }
    }

    /// Renders the loaded chunks in the world.
    pub fn render(&mut self, camera: &player::Camera, mesh_ids: &MeshIds, chisel: &mut Chisel) {
        chisel.load_mesh(mesh_ids.solid_quad);

        for chunk in self.chunk_map.values() {
            chunk.render(camera, chisel);
        }
    }

    /// Synchronizes with the logic-side world, and updates the video state accordingly.
    pub fn update(&mut self, _: Duration, video: &mut Video) {
        // If the render settings have been modified since the previous update, submit the new settings to the renderer.
        if TrackMut::check(&mut self.render_settings) {
            video
                .sculptor
                .update_world(&self.render_settings);
        }

        // Load the chunks as requested by the logic-side world.
        while let Some((position, receiver, render_flag)) = self.channel.recv_load() {
            let chunk = Chunk::create(&video.handle, position, receiver, render_flag, self.mesh_thread_pool.clone());
            self.chunk_map.insert(position, chunk);
        }

        // Unload the chunks as requested by the logic-side world.
        while let Some(pos) = self.channel.recv_unload() {
            self.chunk_map.remove(&pos);
        }

        // Update each of chunks that are currently loaded.
        for chunk in self.chunk_map.values_mut() {
            chunk.update(&video.handle, video.sculptor.texture_coords());
        }
    }
}
