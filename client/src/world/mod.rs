use std::collections::HashMap;

use chunk::Chunk;
use lib::color::{Color, Rgb};
use lib::point::ChunkPt;
use lib::ptr::DetectMut;
use lib::vector::Vec3;
use server::chunk::handle::ChunkLoad;
use server::handle::GameHandle;

use crate::app::Update;
use crate::session::MeshIds;
use crate::video::world::chisel::Chisel;
use crate::video::{Video, world};
use crate::world::particle::Particles;
use crate::world::player::Player;

pub mod chunk;
pub mod frustum;
pub mod particle;
pub mod player;
pub mod sky;

/// The video-side representation of a world within the server.
#[derive(Debug)]
pub struct World {
    /// A map of loaded chunks, keyed by their position in chunk space.
    chunk_map: HashMap<ChunkPt, Chunk>,
    /// The channel used for communicating with the behavior-side world.
    /// The settings used by the fragment shader to video the world.
    pub(crate) render_settings: DetectMut<world::World>,
    pub(crate) player: Player,
    particles: Particles,
}

impl World {
    /// Creates a new instance with the provided channel and video settings.
    pub fn new(video: &mut Video) -> Self {
        let render_settings = world::World {
            ambient_light: Vec3::splat(0.5),
            light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
            fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
            fog_distance: 400.0,
            fog_density: 20.0,
        };

        Self {
            chunk_map: HashMap::new(),
            render_settings: DetectMut::new(render_settings),
            player: Player::create(render_settings.fog_color.to_rgba(), video),
            particles: Particles::create(&video.handle),
        }
    }

    /// Renders the loaded chunks in the world.
    pub fn render(&mut self, mesh_ids: &MeshIds, chisel: &mut Chisel) {
        chisel.load_mesh(mesh_ids.solid_quad);

        for chunk in self.chunk_map.values() {
            chunk.render(&self.player.camera, chisel);
        }

        self.particles.render(chisel);
    }

    /// Synchronizes with the behavior-side world, and updates the video state accordingly.
    pub fn update(&mut self, is_focused: bool, handle: &GameHandle, ctx: &mut Update) {
        if let Some(handle) = handle.next_player_handle() {
            self.player.handle = Some(handle);
        }

        self.player.update(ctx);

        if is_focused {
            self.player.update_input(ctx);
        }

        // If the video settings have been modified since the previous update, submit the new settings to the renderer.
        if DetectMut::check(&mut self.render_settings) {
            ctx.video
                .sculptor
                .update_world(&self.render_settings);
        }

        // Load the chunks as requested by the behavior-side world.
        while let Some(ChunkLoad { position, handle }) = handle.chunks.next_load() {
            let chunk = Chunk::create(&ctx.video.handle, position, handle);
            self.chunk_map.insert(position, chunk);
        }

        // Unload the chunks as requested by the behavior-side world.
        while let Some(position) = handle.chunks.next_unload() {
            self.chunk_map.remove(&position);
        }

        // Update each of chunks that are currently loaded.
        for chunk in self.chunk_map.values_mut() {
            chunk.update(&ctx.video.handle);
        }

        self.particles
            .update(handle, ctx, self.player.camera.position);
    }
}
