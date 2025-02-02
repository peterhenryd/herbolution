use crate::game::chunk::Chunk;
use crate::game::server::player::Player;
use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};
use time::OffsetDateTime;

pub struct SaveFs {
    path_buf: PathBuf,
}

impl SaveFs {
    pub fn new(path_buf: PathBuf) -> Self {
        Self { path_buf }
    }

    pub fn initialize(&self) -> io::Result<()> {
        if !self.path_buf.exists() {
            fs::create_dir_all(&self.path_buf)?;
        }

        Ok(())
    }

    pub fn create(&self, options: SaveOptions) -> LocalGameAddress {
        let path_buf = self.path_buf.join(&options.name);

        if has_valid_save_tree(&path_buf) {
            let config_contents = fs::read_to_string(&path_buf.join("config.toml")).unwrap();
            return LocalGameAddress(SaveMetadata {
                config: toml::from_str(&config_contents).unwrap(),
                path_buf,
            });
        }

        fs::create_dir_all(&path_buf).unwrap();

        fs::create_dir(path_buf.join("chunks")).unwrap();

        let time = OffsetDateTime::now_utc();
        let config = SaveConfig {
            name: options.name,
            created_time: time,
            last_played_time: time,
            seed: options.seed,
        };

        fs::write(
            path_buf.join("config.toml"),
            toml::to_string(&config).unwrap().as_bytes(),
        )
            .unwrap();
        fs::write(
            path_buf.join("player.toml"),
            toml::to_string(&Player::new(Vec3::new(0.0, 256.0, 0.0)))
                .unwrap()
                .as_bytes(),
        )
            .unwrap();

        LocalGameAddress(SaveMetadata { path_buf, config })
    }

    pub fn iter(&self) -> anyhow::Result<impl IntoIterator<Item=SaveMetadata>> {
        let mut vec = vec![];

        for entry in fs::read_dir(&self.path_buf)? {
            let entry = entry?;
            let path_buf = entry.path();

            if !has_valid_save_tree(&path_buf) {
                continue;
            }

            let config_contents = fs::read_to_string(&path_buf.join("config.toml"))?;
            let config = toml::from_str(&config_contents)?;

            vec.push(SaveMetadata { path_buf, config });
        }

        Ok(vec)
    }
}

pub struct SaveMetadata {
    pub path_buf: PathBuf,
    pub config: SaveConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaveConfig {
    pub name: String,
    pub created_time: OffsetDateTime,
    pub last_played_time: OffsetDateTime,
    pub seed: i32,
}

pub struct SaveOptions {
    pub name: String,
    pub seed: i32,
}

pub struct LocalGameAddress(SaveMetadata);

impl LocalGameAddress {
    pub fn get_save_path(&self) -> SavePath {
        SavePath {
            base: self.0.path_buf.clone(),
            chunks: self.0.path_buf.join("chunks"),
            player: self.0.path_buf.join("player.toml"),
        }
    }

    pub fn get_seed(&self) -> i32 {
        self.0.config.seed
    }
}

fn has_valid_save_tree(path: &Path) -> bool {
    path.exists() && path.is_dir() && path.join("config.toml").exists()
}

pub struct SavePath {
    base: PathBuf,
    chunks: PathBuf,
    player: PathBuf,
}

impl SavePath {
    pub fn chunks(&self) -> ChunksPath {
        ChunksPath(&self.chunks)
    }

    pub fn player(&self) -> PlayerPath {
        PlayerPath(&self.player)
    }
}

pub struct ChunksPath<'a>(&'a PathBuf);

impl ChunksPath<'_> {
    pub fn load(&self, x: i32, y: i32) -> Chunk {
        let path = self.0.join(format!("{x}.{y}.chunk"));

        if !path.exists() {
            let chunk = Chunk::new((x, y));

            let contents = toml::to_string(&chunk).unwrap();
            tokio::spawn(async move {
                fs::write(&path, contents).unwrap();
            });

            return chunk;
        }

        toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap()
    }
}

pub struct PlayerPath<'a>(&'a PathBuf);

impl PlayerPath<'_> {
    pub fn load(&self) -> Player {
        toml::from_str(&fs::read_to_string(self.0).unwrap()).unwrap()
    }
}
