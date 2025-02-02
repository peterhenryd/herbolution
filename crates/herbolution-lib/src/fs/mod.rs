pub mod save;

use crate::fs::save::SaveFs;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Fs {
    path_buf: PathBuf,
    pub saves: SaveFs,
}

impl Fs {
    pub fn initialize(&self) -> io::Result<()> {
        if !self.path_buf.exists() {
            fs::create_dir_all(&self.path_buf)?;
        }

        self.saves.initialize()?;

        Ok(())
    }
}

impl AsRef<Path> for Fs {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}

impl Default for Fs {
    fn default() -> Self {
        let path_buf = homedir::my_home()
            .ok()
            .flatten()
            .unwrap_or(PathBuf::from("."))
            .join(".herbolution");
        Self {
            saves: SaveFs::new(path_buf.join("saves")),
            path_buf,
        }
    }
}
