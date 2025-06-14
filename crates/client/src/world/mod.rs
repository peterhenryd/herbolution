use std::collections::HashMap;
use std::time::Duration;

use chunk::Chunk;
use engine::video::{v3d, Video};
use game::chunk::channel::ClientChunkChannel;
use lib::TrackMut;
use math::vector::vec3i;

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
    pub(crate) render_settings: TrackMut<v3d::World>,
}

impl World {
    /// Creates a new instance with the provided channel and render settings.
    pub fn new(channel: ClientChunkChannel, render_settings: v3d::World) -> Self {
        Self {
            chunk_map: HashMap::new(),
            channel,
            render_settings: render_settings.into(),
        }
    }

    /// Renders the loaded chunks in the world.
    pub fn render(&mut self, camera: &player::Camera, mesh_ids: &MeshIds, drawing: &mut v3d::Drawing) {
        drawing.load_mesh(mesh_ids.solid_quad);

        for chunk in self.chunk_map.values() {
            chunk.render(camera, drawing);
        }
    }

    /// Synchronizes with the logic-side world, and updates the video state accordingly.
    pub fn update(&mut self, _: Duration, video: &mut Video) {
        // If the render settings have been modified since the previous update, submit the new settings to the renderer.
        if TrackMut::check(&mut self.render_settings) {
            video.r3d.update_world(&self.render_settings);
        }

        // Load the chunks as requested by the logic-side world.
        while let Some((position, receiver, render_flag)) = self.channel.recv_load() {
            let chunk = Chunk::create(&video.handle, position, receiver, render_flag);
            self.chunk_map.insert(position, chunk);
        }

        // Unload the chunks as requested by the logic-side world.
        while let Some(pos) = self.channel.recv_unload() {
            self.chunk_map.remove(&pos);
        }

        // Update each of chunks that are currently loaded.
        for chunk in self.chunk_map.values_mut() {
            chunk.update(&video.handle, video.r3d.texture_coords());
        }
    }
}
