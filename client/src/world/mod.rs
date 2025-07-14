use chunk::Chunk;
use lib::color::{Color, Rgb};
use lib::ptr::DetectMut;
use lib::vector::Vec3;
use server::chunk::handle::ChunkLoad;
use server::handle::GameHandle;

use crate::app::Update;
use crate::session::MeshIds;
use crate::video::world::chisel::Chisel;
use crate::video::{world, Video};
use crate::world::chunk::ChunkMap;
use crate::world::particle::Particles;
use crate::world::player::Player;

pub mod chunk;
pub mod frustum;
pub mod particle;
pub mod player;
pub mod sky;

#[derive(Debug)]
pub struct World {
    chunk_map: ChunkMap,

    pub(crate) render_settings: DetectMut<world::World>,
    pub(crate) player: Player,
    particles: Particles,
}

impl World {
    pub fn new(video: &mut Video) -> Self {
        let render_settings = world::World {
            ambient_light: Vec3::splat(0.5),
            light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
            fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
            fog_distance: 300.0,
            fog_density: 20.0,
        };

        Self {
            chunk_map: ChunkMap::new(),
            render_settings: DetectMut::new(render_settings),
            player: Player::create(render_settings.fog_color.to_rgba(), video),
            particles: Particles::create(&video.handle),
        }
    }

    pub fn render(&mut self, mesh_ids: &MeshIds, chisel: &mut Chisel) {
        chisel.load_mesh(mesh_ids.solid_quad);

        self.chunk_map
            .render(&self.player.frustum, chisel);
        self.particles.render(chisel);
    }

    pub fn update(&mut self, is_focused: bool, handle: &GameHandle, ctx: &mut Update) {
        if let Some(handle) = handle.next_player_handle() {
            self.player.handle = Some(handle);
        }

        self.player.update(ctx);

        if is_focused {
            self.player.update_input(ctx);
        }

        if DetectMut::check(&mut self.render_settings) {
            ctx.video
                .sculptor
                .update_world(&self.render_settings);
        }

        while let Some(ChunkLoad { position, handle }) = handle.chunks.next_load() {
            let chunk = Chunk::create(&ctx.video.handle, position, handle);
            self.chunk_map.map.insert(position, chunk);
        }

        while let Some(position) = handle.chunks.next_unload() {
            self.chunk_map.map.remove(&position);
        }

        self.chunk_map.update(&ctx.video.handle);

        self.particles
            .update(handle, ctx, self.player.state.position);
    }
}
