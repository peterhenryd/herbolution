use std::fs::{create_dir, read_dir, read_to_string, write, ReadDir};
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Save {
    pub path: PathBuf,
    pub descriptor: SaveDescriptor,
}

impl Save {
    pub fn open(path: PathBuf) -> Result<Self, SaveError> {
        let descriptor = toml::from_str(&read_to_string(path.join("Save.toml"))?)?;

        Ok(Self { path, descriptor })
    }

    pub fn worlds(&self) -> io::Result<Worlds> {
        Worlds::new(self.path.join("worlds"))
    }

    pub fn default_world(&self) -> Result<SaveWorld, SaveError> {
        let world_path = self
            .path
            .join("worlds")
            .join(&self.descriptor.default_world);

        SaveWorld::open(world_path, self.descriptor.default_world.clone())
    }

    pub fn create(path: &Path, attributes: SaveAttributes) -> Result<Self, SaveError> {
        if path.exists() {
            return Err(SaveError::AlreadyExists);
        }

        create_dir(path)?;

        let worlds_path = path.join("worlds");
        if !worlds_path.exists() {
            create_dir(&worlds_path)?;
        }

        let world_path = worlds_path.join(&attributes.default_world.name);
        if !world_path.exists() {
            create_dir(&world_path)?;

            let world_descriptor_path = world_path.join("World.toml");
            let world_descriptor = toml::to_string(&attributes.default_world.descriptor)?;
            write(&world_descriptor_path, world_descriptor)?;
        }

        let descriptor = attributes.into();
        let descriptor_path = path.join("Save.toml");
        write(&descriptor_path, toml::to_string(&descriptor)?)?;

        Ok(Self {
            path: path.to_path_buf(),
            descriptor,
        })
    }

    pub fn create_or_open(path: PathBuf, attributes: SaveAttributes) -> Result<Self, SaveError> {
        match Save::create(&path, attributes) {
            Ok(x) => Ok(x),
            Err(SaveError::AlreadyExists) => Save::open(path),
            Err(e) => Err(e),
        }
    }
}

pub struct SaveAttributes {
    pub title: String,
    pub default_world: WorldAttributes,
}

impl Into<SaveDescriptor> for SaveAttributes {
    fn into(self) -> SaveDescriptor {
        SaveDescriptor {
            title: self.title,
            default_world: self.default_world.name,
        }
    }
}

pub struct WorldAttributes {
    pub name: String,
    pub descriptor: WorldDescriptor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldDescriptor {
    pub title: String,
    pub seed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDescriptor {
    pub title: String,
    pub default_world: String,
}

#[derive(Debug)]
pub struct Saves {
    read_dir: ReadDir,
}

impl Saves {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Saves> {
        Ok(Self { read_dir: read_dir(path)? })
    }
}

impl Iterator for Saves {
    type Item = Result<Save, SaveError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = self.read_dir.next()?.ok()?;

        while !entry.file_type().ok()?.is_dir() {
            entry = self.read_dir.next()?.ok()?;
        }

        Some(Save::open(entry.path()))
    }
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Failed to read save directory: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to deserialize save descriptor: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Failed to serialize save descriptor: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Save already exists")]
    AlreadyExists,
}

pub struct Worlds {
    read_dir: ReadDir,
}

impl Worlds {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self { read_dir: read_dir(path)? })
    }
}

impl Iterator for Worlds {
    type Item = Result<SaveWorld, SaveError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = self.read_dir.next()?.ok()?;

        while !entry.file_type().ok()?.is_dir() {
            entry = self.read_dir.next()?.ok()?;
        }

        Some(SaveWorld::open(entry.path(), entry.file_name().to_string_lossy().into_owned()))
    }
}

pub struct SaveWorld {
    pub path: PathBuf,
    pub name: String,
    pub descriptor: WorldDescriptor,
}

impl SaveWorld {
    pub fn open(path: PathBuf, name: String) -> Result<Self, SaveError> {
        let descriptor = toml::from_str(&read_to_string(path.join("World.toml"))?)?;
        Ok(Self { path, name, descriptor })
    }
}
