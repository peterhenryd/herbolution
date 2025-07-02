use std::collections::HashMap;

use chunk::Chunk;
use engine::sculptor::Chisel;
use engine::{sculptor, Engine};
use game::chunk::handle::ChunkLoad;
use game::handle::GameHandle;
use lib::point::ChunkPt;
use lib::ptr::DetectMut;
use math::color::{Color, Rgb};
use math::vector::Vec3;

use crate::app::Update;
use crate::mesh::MeshIds;
use crate::player::Player;
use crate::world::particle::Particles;

pub mod chunk;
pub mod frustum;
pub mod particle;
pub mod sky;

/// The render-side representation of a world within the herbolution_game.
#[derive(Debug)]
pub struct World {
    /// A map of loaded chunks, keyed by their position in chunk space.
    chunk_map: HashMap<ChunkPt, Chunk>,
    /// The channel used for communicating with the behavior-side world.
    /// The settings used by the fragment shader to render the world.
    pub(crate) render_settings: DetectMut<sculptor::World>,
    pub(crate) player: Player,
    particles: Particles,
}

impl World {
    /// Creates a new instance with the provided channel and render settings.
    pub fn new(engine: &mut Engine) -> Self {
        let render_settings = sculptor::World {
            ambient_light: Vec3::splat(0.5),
            light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
            fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
            fog_distance: 200.0,
            fog_density: 20.0,
        };

        Self {
            chunk_map: HashMap::new(),
            render_settings: DetectMut::new(render_settings),
            player: Player::create(render_settings.fog_color.to_rgba(), engine),
            particles: Particles::create(&engine.video.handle),
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

        // If the render settings have been modified since the previous update, submit the new settings to the renderer.
        if DetectMut::check(&mut self.render_settings) {
            ctx.engine
                .video
                .sculptor
                .update_world(&self.render_settings);
        }

        // Load the chunks as requested by the behavior-side world.
        while let Some(ChunkLoad { position, handle }) = handle.chunks.next_load() {
            let chunk = Chunk::create(&ctx.engine.video.handle, position, handle);
            self.chunk_map.insert(position, chunk);
        }

        // Unload the chunks as requested by the behavior-side world.
        while let Some(position) = handle.chunks.next_unload() {
            self.chunk_map.remove(&position);
        }

        // Update each of chunks that are currently loaded.
        for chunk in self.chunk_map.values_mut() {
            chunk.update(&ctx.engine.video.handle);
        }

        self.particles
            .update(handle, ctx, self.player.position());
    }
}
