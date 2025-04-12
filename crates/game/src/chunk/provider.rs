use crate::chunk::generator::{ChunkGenerator, GenerationParams};
use crate::chunk::{CubeMesh, MaterialMesh};
use lib::display;
use math::vector::vec3i;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[derive(Debug)]
pub struct ChunkProvider {
    pub(crate) dir_path: PathBuf,
    generator: ChunkGenerator,
    reader: ChunkReader,
}

#[derive(Debug)]
pub struct ChunkReader {
    to_be_loaded: Arc<Mutex<Vec<CubeMesh>>>,
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

        if let Ok(mut guard) = self.reader.to_be_loaded.try_lock() {
            vec.extend(guard.drain(..));
        }

        vec.extend(self.generator.dequeue());

        vec
    }
}

impl ChunkReader {
    pub fn new() -> Self {
        Self {
            to_be_loaded: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn request(&self, path: PathBuf, pos: vec3i) {
        let to_be_loaded = self.to_be_loaded.clone();

        task::spawn(async move {
            let bytes = tokio::fs::read(path).await.unwrap();
            let material_mesh = MaterialMesh::decode(&bytes);

            to_be_loaded.lock().await.push(material_mesh.to_cube_mesh(pos));
        });
    }
}