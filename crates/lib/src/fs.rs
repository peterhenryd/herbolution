use serde::{Deserialize, Serialize};
use std::fs::{read_dir, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Clone)]
pub struct Fs {
    data_dir: PathBuf,
}

pub struct Save {
    pub path: PathBuf,
    pub descriptor: SaveDescriptor,
}

#[derive(Serialize, Deserialize)]
pub struct SaveDescriptor {
    title: String,
}

pub struct Saves {
    read_dir: ReadDir,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Failed to read save directory: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to deserialize save descriptor: {0}")]
    Toml(#[from] toml::de::Error),
}

impl Fs {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn saves(&self) -> io::Result<Saves> {
        Ok(Saves {
            read_dir: read_dir(self.data_dir.join("saves"))?
        })
    }
}

impl Save {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SaveError> {
        let path = path.as_ref();
        //let descriptor = toml::from_str(&read_to_string(path.join("Save.toml"))?)?;

        Ok(Self { path: path.to_path_buf(), descriptor: SaveDescriptor { title: String::new() } })
    }
}

impl Iterator for Saves {
    type Item = Result<Save, SaveError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = self.read_dir.next()?.ok()?;
        // Skip non-directory entries
        while !entry.file_type().ok()?.is_dir() {
            entry = self.read_dir.next()?.ok()?;
        }

        Some(Save::open(entry.path()))
    }
}