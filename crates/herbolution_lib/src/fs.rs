use crate::save::{Save, SaveAttributes, SaveError, Saves};
use include_dir::{include_dir, Dir};
use std::env::home_dir;
use std::fs::{create_dir, write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Fs {
    saves: PathBuf,
    root: PathBuf,
}

impl Fs {
    pub fn new(root: Option<PathBuf>) -> Self {
        let root = root.unwrap_or_else(root_dir);

        Self {
            saves: root.join("saves"),
            root,
        }
    }

    pub fn init(&self) -> std::io::Result<()> {
        if !self.root.is_dir() {
            create_dir(&self.root)?;
        }

        if !self.saves.exists() {
            create_dir(&self.saves)?;
        }

        copy_assets(&self.root)?;

        Ok(())
    }

    pub fn saves(&self) -> std::io::Result<Saves> {
        Saves::new(&self.saves)
    }

    pub fn open_save(&self, name: impl AsRef<str>) -> Result<Save, SaveError> {
        Save::open(self.saves.join(name.as_ref()))
    }

    pub fn create_or_open_save(&self, name: impl AsRef<str>, attributes: SaveAttributes) -> Result<Save, SaveError> {
        Save::create_or_open(self.saves.join(name.as_ref()), attributes)
    }

    pub fn path(&self) -> &Path {
        &self.root
    }
}

fn root_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("../../../../../../.."))
        .join(".herbolution")
}

fn copy_assets(base_path: &Path) -> std::io::Result<()> {
    const DIR: Dir<'_> = include_dir!("assets");

    let mut entries = DIR.entries().to_vec();
    while let Some(entry) = entries.pop() {
        let path = base_path.join(entry.path());

        if let Some(file) = entry.as_file() {
            if !path.exists() {
                write(&path, file.contents())?;
            }
        } else if let Some(dir) = entry.as_dir() {
            if !path.exists() {
                create_dir(&path)?;
            }

            entries.extend_from_slice(dir.entries());
        }
    }

    write(
        base_path.join("README"),
        "Please do not edit the contents of this directory manually; it is overwritten frequently without warning.",
    )?;

    Ok(())
}
