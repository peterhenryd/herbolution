use std::iter::Chain;
use std::path::PathBuf;
use std::sync::Arc;

use crate::chunk::codec::CubeGrid;
use crate::chunk::mesh::CubeMesh;
use crate::generator::{ChunkGenerator, GenerationParams};
use crossbeam::channel::{unbounded, Receiver, Sender, TryIter};
use lib::point::ChunkPt;
use lib::util::display;
use tracing::error;

#[derive(Debug)]
pub struct ChunkProvider {
    pub(crate) dir_path: PathBuf,
    pub(crate) generator: ChunkGenerator,
    pub(crate) reader: ChunkReader,
}

#[derive(Debug)]
pub struct ChunkReader {
    tx: Sender<CubeMesh>,
    rx: Receiver<CubeMesh>,
}

impl ChunkProvider {
    pub fn new(dir_path: PathBuf, seed: i64) -> Self {
        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path).unwrap();
        }

        Self {
            dir_path,
            generator: ChunkGenerator::new(Arc::new(GenerationParams::new(seed))),
            reader: ChunkReader::new(),
        }
    }

    pub fn request(&self, position: ChunkPt) {
        let path = self
            .dir_path
            .join(display::Join::new(&position.0, ".").to_string());

        if path.is_file() {
            self.reader.request(path, position);
        } else {
            self.generator.request(position);
        }
    }

    pub fn dequeue(&self) -> Chain<TryIter<'_, CubeMesh>, TryIter<'_, CubeMesh>> {
        self.generator
            .dequeue()
            .chain(self.reader.rx.try_iter())
    }
}

impl ChunkReader {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self { tx, rx }
    }

    pub fn request(&self, path: PathBuf, position: ChunkPt) {
        let tx = self.tx.clone();

        rayon::spawn(move || {
            let bytes;
            match std::fs::read(path) {
                Ok(x) => bytes = x,
                Err(e) => return error!("Failed to read chunk file: {}", e),
            }
            let material_mesh = CubeGrid::decode(&bytes);

            tx.send(material_mesh.to_mesh(position)).unwrap();
        });
    }
}
