use crate::fs::save::{Save, SaveAttributes, SaveError, Saves};
use std::fs::create_dir;
use std::io;
use std::path::{Path, PathBuf};

pub mod save;

#[derive(Debug)]
pub struct Fs {
    path: Paths,
}

impl Fs {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            path: Paths::new(data_dir),
        }
    }
    
    pub fn path(&self) -> &Paths {
        &self.path
    }

    pub fn saves(&self) -> io::Result<Saves> {
        Saves::new(&self.path.saves)
    }
    
    pub fn open_save(&self, name: impl AsRef<str>) -> Result<Save, SaveError> {
        Save::open(self.path.saves.join(name.as_ref()))
    }
    
    pub fn create_or_open_save(&self, name: impl AsRef<str>, attributes: SaveAttributes) -> Result<Save, SaveError> {
        Save::create_or_open(self.path.saves.join(name.as_ref()), attributes)
    }
}

#[derive(Debug)]
pub struct Paths {
    root: PathBuf,
    saves: PathBuf,
}

impl Paths {
    pub fn new(root: PathBuf) -> Self {
        if !root.exists() {
            create_dir(&root).unwrap();
        }
        
        let saves = root.join("saves");
        if !saves.exists() {
            create_dir(&saves).unwrap();
        }
        
        Self {
            root,
            saves
        }
    }
}

impl AsRef<Path> for Paths {
    fn as_ref(&self) -> &Path {
        &self.root
    }
}