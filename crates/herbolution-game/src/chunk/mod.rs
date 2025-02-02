use serde::Deserialize;
use crate::message::GameMessage;

pub mod generator;
pub mod material;
pub mod section;

#[derive(Debug)]
pub struct LoadedChunk {
    pub(crate) chunk: Chunk,
    cube_updates: Vec<CubeUpdate>,
}

impl LoadedChunk {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            cube_updates: vec![],
        }
    }

    pub fn tick(&mut self) -> Vec<GameMessage> {
        self.cube_updates.drain(..).map(CubeUpdate::into).collect()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chunk {
    pub position: vec2i,
    pub sections: Vec<ChunkSection>,
    state: ChunkState,
}

impl Chunk {
}

impl Chunk {
    pub fn new(position: impl Into<IVec2>) -> Self {
        Self {
            position: position.into(),
            sections: vec![],
            state: ChunkState::Empty,
        }
    }

    pub fn set_column(&mut self, x: usize, z: usize, y1: usize, y2: usize, material: Material) {
        while self.sections.len() * CHUNK_SIZE <= y2 {
            let y_section_index = self.sections.len();
            self.sections.push(ChunkSection::new(IVec3::new(
                self.position.x,
                (y_section_index * CHUNK_SIZE) as i32,
                self.position.y,
            )));
        }

        let y1_section_index = y1 / CHUNK_SIZE;
        let y2_section_index = y2 / CHUNK_SIZE;

        // Handle the first section
        let y1_start = y1 % CHUNK_SIZE;
        let y1_end = std::cmp::min(CHUNK_SIZE, y1 + CHUNK_SIZE - (y1 % CHUNK_SIZE)); // Either 16 or how much space is left until y2
        self.sections[y1_section_index].set_column(x, z, y1_start..y1_end, material);

        // Handle the middle sections (if any)
        for i in (y1_section_index + 1)..y2_section_index {
            self.sections[i].set_column(x, z, 0..CHUNK_SIZE, material);
        }

        // Handle the last section
        let y2_end = y2 % CHUNK_SIZE;
        if y2_section_index < self.sections.len() {
            // Protect from out of bounds access
            self.sections[y2_section_index].set_column(x, z, 0..y2_end, material);
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum ChunkState {
    Empty,
    Ready,
}

#[derive(Debug, Copy, Clone)]
pub struct ChunkLocalPosition {
    pub chunk: IVec3,
    pub local: U8Vec3,
}

#[derive(Debug)]
pub enum CubeUpdate {
    Add {
        position: ChunkLocalPosition,
        material: Material,
    },
    Remove {
        position: ChunkLocalPosition,
    },
}

impl Into<GameMessage> for CubeUpdate {
    fn into(self) -> GameMessage {
        match self {
            CubeUpdate::Add { position, material } => GameMessage::CubeAdded { position, material },
            CubeUpdate::Remove { position } => GameMessage::CubeRemoved { position },
        }
    }
}
