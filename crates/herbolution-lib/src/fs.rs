use std::fs::{create_dir, create_dir_all, write};
use std::path::{Path, PathBuf};

use include_dir::{Dir, include_dir};

use crate::save::{Save, SaveAttributes, SaveError, Saves};

#[derive(Debug)]
pub struct Fs {
    root_path: PathBuf,
}

pub fn init(data_dir: &Path) -> std::io::Result<()> {
    create_dir_all(data_dir)?;
    create_dir_all(data_dir.join("saves"))?;
    copy_assets_from_binary(&data_dir.join("assets"))?;

    Ok(())
}

impl Fs {
    pub fn saves(&self) -> std::io::Result<Saves> {
        Saves::new(self.root_path.join("saves"))
    }

    pub fn open_save(&self, name: impl AsRef<str>) -> Result<Save, SaveError> {
        Save::open(self.root_path.join("saves").join(name.as_ref()))
    }

    pub fn create_or_open_save(&self, name: impl AsRef<str>, attributes: SaveAttributes) -> Result<Save, SaveError> {
        Save::create_or_open(self.root_path.join("saves").join(name.as_ref()), attributes)
    }

    pub fn path(&self) -> &Path {
        &self.root_path
    }
}

pub fn copy_assets_from_binary(base_path: &Path) -> std::io::Result<()> {
    const ASSET_DIR: Dir<'_> = include_dir!("assets");

    if !base_path.exists() {
        create_dir(base_path)?;
    }

    let mut entries = ASSET_DIR.entries().to_vec();
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
