use chunk::Chunk;
use server::chunk::handle::ChunkLoad;
use server::handle::GameHandle;

use crate::app::Update;
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

    pub(crate) player: Player,
    particles: Particles,
}

impl World {
    pub fn new(aspect: f32) -> Self {
        Self {
            chunk_map: ChunkMap::new(),
            player: Player::new(aspect),
            particles: Particles::create(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn render(&mut self, frame: &mut kiln::Draw3d<'_, '_>) {
        self.chunk_map.render(&self.player.frustum, frame);
        self.particles.render(frame);
    }

    pub fn update(&mut self, is_focused: bool, handle: &GameHandle, ctx: &mut Update) {
        if let Some(handle) = handle.next_player_handle() {
            self.player.handle = Some(handle);
        }

        self.player.update(ctx);

        if is_focused {
            self.player.update_input(ctx);
        }

        while let Some(ChunkLoad { position, handle }) = handle.chunks.next_load() {
            let chunk = Chunk::create(position, handle);
            self.chunk_map.map.insert(position, chunk);
        }

        while let Some(position) = handle.chunks.next_unload() {
            self.chunk_map.map.remove(&position);
        }

        self.chunk_map.update(ctx.app.gpu, ctx.app.core3d.texture_coords());

        self.particles
            .update(handle, ctx, self.player.state.pos);
    }
}
