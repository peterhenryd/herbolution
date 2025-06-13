use crate::chunk::generator::{ChunkGenerator, GenerationParams};
use crate::chunk::{CubeGrid, CubeMesh};
use lib::display;
use math::vector::vec3i;
use std::path::PathBuf;
use std::sync::Arc;
use crossbeam::channel::{unbounded, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};
use tracing::error;

#[derive(Debug)]
pub struct ChunkProvider {
    pub(crate) dir_path: PathBuf,
    generator: ChunkGenerator,
    reader: ChunkReader,
}

#[derive(Debug)]
pub struct ChunkReader {
    tx: Sender<CubeMesh>,
    rx: Receiver<CubeMesh>,
    thread_pool: ThreadPool,
}

impl ChunkProvider {
    pub fn new(dir_path: PathBuf, generator: Arc<GenerationParams>) -> Self {
        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path).unwrap();
        }

        Self {
            dir_path,
            generator: ChunkGenerator::new(generator),
            reader: ChunkReader::new(),
        }
    }

    pub fn request(&self, pos: vec3i) {
        let path = self.dir_path.join(display::Join::new(&pos, ".").to_string());

        if path.is_file() {
            self.reader.request(path, pos);
        } else {
            self.generator.request(pos);
        }
    }

    pub fn dequeue(&self) -> Vec<CubeMesh> {
        let mut vec = vec![];

        vec.extend(self.reader.rx.try_iter());
        vec.extend(self.generator.dequeue());

        vec
    }
}

impl ChunkReader {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        
        Self { tx, rx, thread_pool }
    }

    pub fn request(&self, path: PathBuf, pos: vec3i) {
        let tx = self.tx.clone();

        self.thread_pool.spawn(move || {
            let bytes;
            match std::fs::read(path) {
                Ok(x) => bytes = x,
                Err(e) => return error!("Failed to read chunk file: {}", e),
            }
            let material_mesh = CubeGrid::decode(&bytes);

            tx.send(material_mesh.to_mesh(pos)).unwrap();
        });
    }
}