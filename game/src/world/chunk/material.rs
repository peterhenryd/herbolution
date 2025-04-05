use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bimap::BiHashMap;
use hashbrown::{Equivalent, HashMap};
use pulz_arena::{Arena, Index};
use tokio::spawn;
use tokio::sync::RwLock;
use lib::geometry::cuboid::face::{Face, PerFace};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Material {
    pub id: String,
    pub is_face_culled: bool,
    pub has_collider: bool,
    render: Render,
}

impl Material {
    pub fn texture_index(&self, face: Face) -> TextureIndex {
        match &self.render {
            Render::Uniform(index) => *index,
            Render::Facial(indices) => indices[face],
        }
    }
}

#[derive(Debug)]
pub struct Palette {
    map: BiHashMap<u16, Arc<Material>>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            map: BiHashMap::new(),
        }
    }

    pub fn get_by_index(&self, id: u16) -> Option<&Arc<Material>> {
        self.map.get_by_left(&id)
    }

    pub fn get_or_insert(&mut self, material: Option<&Arc<Material>>) -> u16 {
        let Some(material) = material else { return 0 };

        if let Some(&id) = self.map.get_by_right(material) {
            id
        } else {
            self.insert(material.clone())
        }
    }

    pub fn insert(&mut self, material: Arc<Material>) -> u16 {
        let id = self.map.len() as u16;
        self.map.insert(id, material);
        id
    }
}

#[derive(Clone)]
pub struct Registry {
    arena: Arc<RwLock<Arena<Arc<Material>>>>,
    indices: Arc<RwLock<HashMap<String, Index>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            arena: Arc::new(RwLock::new(Arena::new())),
            indices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add(&self, material: Arc<Material>) {
        let arena = self.arena.clone();
        spawn(async move {
            arena.write().await.insert(material);
        });
    }

    pub async fn get(&self, id: &str) -> Option<Arc<Material>> {
        let index = *self.indices.read().await.get(id)?;
        self.arena.read().await.get(index).cloned()
    }
}

impl Debug for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Render {
    Uniform(TextureIndex),
    Facial(PerFace<TextureIndex>),
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureIndex(u32);

/*
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Material {
    Stone,
    Dirt,
    Grass,
}

impl Material {
    pub fn is_face_culled(self) -> bool {
        true
    }

    pub fn texture_index(self, face: Face) -> u32 {
        match self {
            Self::Stone => 0,
            Self::Dirt => 1,
            Self::Grass => match face {
                Face::Top => 2,
                Face::Bottom => 1,
                _ => 3,
            },
        }
    }

    pub fn can_collide(self) -> bool {
        true
    }

    pub fn entries() -> IntoIter<Material, 3> {
        [Self::Stone, Self::Dirt, Self::Grass].into_iter()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Stone => "stone",
            Self::Dirt => "dirt",
            Self::Grass => "grass",
        }
    }
}

 */

pub trait MaterialProvider {
    fn provide(self, textures: &TextureRegistry) -> Material;
}

pub enum StandardMaterial {
    Stone,
    Dirt,
    Grass,
}

impl MaterialProvider for StandardMaterial {
    fn provide(self, textures: &TextureRegistry) -> Material {
        let [stone_tex, dirt_tex, grass_tex, grass_side_tex] = textures.query(
            ["stone", "dirt", "grass", "grass_side"]
        );
        match self {
            StandardMaterial::Stone => Material {
                id: "stone".to_owned(),
                is_face_culled: true,
                has_collider: true,
                render: Render::Uniform(stone_tex),
            },
            StandardMaterial::Dirt => Material {
                id: "dirt".to_owned(),
                is_face_culled: true,
                has_collider: true,
                render: Render::Uniform(dirt_tex),
            },
            StandardMaterial::Grass => Material {
                id: "grass".to_owned(),
                is_face_culled: true,
                has_collider: true,
                render: Render::Facial(PerFace::splat(grass_side_tex)
                    .top(grass_tex)
                    .bottom(dirt_tex)
                ),
            },
        }
    }
}

pub struct TextureRegistry {
    map: HashMap<String, TextureIndex>,
}

impl TextureRegistry {
    pub fn query<const N: usize>(
        &self,
        names: [&str; N],
    ) -> [TextureIndex; N] {
        names.map(|x| *self.map.get(x).unwrap())
    }
}

pub trait OptionMaterialExt {
    fn is_face_culled(&self) -> bool;
}

impl OptionMaterialExt for Option<Arc<Material>> {
    fn is_face_culled(&self) -> bool {
        self.as_ref().map_or(false, |material| material.is_face_culled)
    }
}

impl<'a> OptionMaterialExt for Option<&'a Arc<Material>> {
    fn is_face_culled(&self) -> bool {
        self.as_ref().map_or(false, |material| material.is_face_culled)
    }
}