use std::fs::create_dir;
use std::io;
use std::path::PathBuf;

use crate::save::{Save, SaveAttributes, SaveError, Saves};

#[derive(Debug)]
pub struct Fs {
    saves: PathBuf,
    root: PathBuf,
}

impl Fs {
    pub fn new(root: PathBuf) -> Self {
        Self {
            saves: root.join("saves"),
            root,
        }
    }

    pub fn init(&self) -> io::Result<()> {
        if !self.root.is_dir() {
            create_dir(&self.root)?;
        }

        if !self.saves.exists() {
            create_dir(&self.saves)?;
        }

        Ok(())
    }

    pub fn saves(&self) -> io::Result<Saves> {
        Saves::new(&self.saves)
    }

    pub fn open_save(&self, name: impl AsRef<str>) -> Result<Save, SaveError> {
        Save::open(self.saves.join(name.as_ref()))
    }

    pub fn create_or_open_save(&self, name: impl AsRef<str>, attributes: SaveAttributes) -> Result<Save, SaveError> {
        Save::create_or_open(self.saves.join(name.as_ref()), attributes)
    }
}
