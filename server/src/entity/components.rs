use std::any::Any;
use std::collections::HashSet;

use lib::chunk;
use lib::point::ChunkPt;
use lib::vector::vec3i;

use crate::entity::behavior::{EntityBehavior, EntityBehaviorType, EntityContext};

#[derive(Debug)]
pub struct ChunkLoader {
    pub(crate) prev_chunk_position: ChunkPt,
    radial_chunk_positions: HashSet<ChunkPt>,
}

impl ChunkLoader {
    pub fn new() -> Self {
        Self {
            prev_chunk_position: ChunkPt::ZERO,
            radial_chunk_positions: HashSet::new(),
        }
    }
}

impl EntityBehavior for ChunkLoader {
    fn update(&mut self, ctx: &mut EntityContext<'_>) {
        let chunk_position = ChunkPt(ctx.entity.body.position().cast() / chunk::LENGTH as i32);
        if chunk_position == self.prev_chunk_position {
            return;
        }

        self.prev_chunk_position = chunk_position;

        let new_positions = fill_rhombus(chunk_position, 16);

        self.radial_chunk_positions
            .iter()
            .filter(|&x| !new_positions.contains(x))
            .for_each(|&x| ctx.chunk_map.queue_unload(x));

        self.radial_chunk_positions = new_positions;

        for &chunk_position in &self.radial_chunk_positions {
            ctx.chunk_map.queue_load(chunk_position);
        }
    }

    fn select_from(behavior: &mut EntityBehaviorType) -> Option<&mut Self>
    where
        Self: Sized,
    {
        match behavior {
            EntityBehaviorType::Player(_) => None,
            EntityBehaviorType::ChunkLoader(x) => Some(x),
            EntityBehaviorType::Dyn(x) => (x.as_mut() as &mut dyn Any).downcast_mut(),
            _ => None,
        }
    }
}

fn fill_rhombus(center: ChunkPt, radius: i32) -> HashSet<ChunkPt> {
    let mut set = HashSet::new();
    for x in -radius..=radius {
        for y in -radius / 3..=radius / 3 {
            for z in -radius..=radius {
                if x.abs() + y.abs() + z.abs() <= radius {
                    set.insert(ChunkPt(center.0 + vec3i::new(x, y, z)));
                }
            }
        }
    }
    set
}
