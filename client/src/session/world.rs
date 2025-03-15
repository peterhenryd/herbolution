use crate::session::chunk::SessionChunkMap;

pub struct SessionWorld {
    //sky: Sky,
    pub(crate) chunk_map: SessionChunkMap,
}
/*
pub struct Sky {
    color: Rgba<f64>,
}


 */
impl SessionWorld {
    pub fn new() -> Self {
        Self {
            /*
            sky: Sky {
                color: Rgba::from_rgb(117, 255, 250).into(),
            },

             */
            chunk_map: SessionChunkMap::new(),
        }
    }
}