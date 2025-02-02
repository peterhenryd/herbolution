use crate::engine::input::Input;
use crate::fs::save::{LocalGameAddress, SavePath};
use crate::game::chunk::generator::ChunkGenerator;
use crate::game::chunk::{Chunk, LoadedChunk};
use crate::game::message::{GameMessage, InputMessage, KeyedAction};
use crate::game::server::player::Player;
use crate::util::time::TickTime;
use glam::IVec2;
use std::collections::HashMap;
use winit::event::ElementState;

pub mod player;

pub struct GameServer {
    save_path: SavePath,
    time: TickTime,
    loaded_chunks: HashMap<IVec2, LoadedChunk>,
    added_chunks: Vec<IVec2>,
    chunk_generator: ChunkGenerator,
    player: Player,
    input: Input,
}

impl GameServer {
    pub fn new(address: LocalGameAddress) -> Self {
        let save_path = address.get_save_path();
        let player = save_path.player().load();

        Self {
            save_path,
            time: TickTime::default(),
            loaded_chunks: HashMap::new(),
            added_chunks: vec![],
            chunk_generator: ChunkGenerator::new(address.get_seed()),
            player,
            input: Input::default(),
        }
    }

    pub fn update(&mut self) -> Vec<GameMessage> {
        self.time.update_clock();

        let mut messages = vec![];

        while self.time.should_tick() {
            self.time.tick();
            messages.extend(self.tick());
        }

        messages
    }

    fn tick(&mut self) -> Vec<GameMessage> {
        let mut messages = vec![];

        for added_chunk in self.added_chunks.drain(..) {
            let loaded_chunk = self.loaded_chunks.get_mut(&added_chunk).unwrap();
            messages.push(GameMessage::ChunkLoaded {
                chunk: Box::new(loaded_chunk.chunk.clone()),
            });
        }

        for loaded_chunk in self.loaded_chunks.values_mut() {
            messages.extend(loaded_chunk.tick());
        }

        messages.extend(self.player.update());

        messages
    }

    pub(crate) fn load_chunk(&mut self, position: IVec2) -> &LoadedChunk {
        if !self.loaded_chunks.contains_key(&position) {
            let mut chunk = Chunk::new(position);
            self.chunk_generator.generate(&mut chunk);
            self.loaded_chunks.insert(position, LoadedChunk::new(chunk));
            self.added_chunks.push(position);
        }

        self.loaded_chunks.get_mut(&position).unwrap()
    }

    pub fn process_input(&mut self, input: InputMessage) {
        match input {
            InputMessage::MouseClicked { .. } => {}
            InputMessage::MouseMoved { dx, dy } => {
                self.player.motion.rotation = Some((dx, dy));
            }
            InputMessage::Keyed { action, state } => {
                let controller = match action {
                    KeyedAction::MoveForward => &mut self.player.motion.forward,
                    KeyedAction::MoveBackward => &mut self.player.motion.backward,
                    KeyedAction::MoveLeft => &mut self.player.motion.leftward,
                    KeyedAction::MoveRight => &mut self.player.motion.rightward,
                    KeyedAction::MoveUp => &mut self.player.motion.upward,
                    KeyedAction::MoveDown => &mut self.player.motion.downward,
                };
                *controller = if state == ElementState::Pressed {
                    1.0
                } else {
                    0.0
                };
            }
        }
    }
}
