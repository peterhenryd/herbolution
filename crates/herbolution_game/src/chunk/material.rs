use std::hash::Hash;
use std::num::NonZeroU16;
use std::slice::Iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use hashbrown::{Equivalent, HashMap};
use lib::face::Faces;
use lib::group_key::{group_key, GroupKeyBuf};
use math::color::Rgba;
use serde::{Deserialize, Serialize};

use crate::chunk::cube::Cube;
use crate::chunk::handle::ClientChunkHandle;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Material {
    pub group_key: GroupKeyBuf,
    pub has_collider: bool,
    pub cullable_faces: Faces,
    pub texture: Texture,
}

impl Material {
    pub fn stone() -> Self {
        Self {
            group_key: group_key("herbolution", "stone"),
            has_collider: true,
            cullable_faces: Faces::all(),
            texture: Texture::Colors {
                vec: vec![Rgba::new(0.5, 0.5, 0.5, 1.0), Rgba::new(0.6, 0.6, 0.6, 1.0), Rgba::new(0.7, 0.7, 0.7, 1.0)],
            },
        }
    }

    pub fn dirt() -> Self {
        Self {
            group_key: group_key("herbolution", "dirt"),
            has_collider: true,
            cullable_faces: Faces::all(),
            texture: Texture::Colors {
                vec: vec![Rgba::new(0.4, 0.3, 0.2, 1.0), Rgba::new(0.5, 0.4, 0.3, 1.0), Rgba::new(0.6, 0.5, 0.4, 1.0)],
            },
        }
    }

    pub fn grass() -> Self {
        Self {
            group_key: group_key("herbolution", "grass"),
            has_collider: true,
            cullable_faces: Faces::all(),
            texture: Texture::Colors {
                vec: vec![Rgba::new(0.1, 0.8, 0.1, 1.0), Rgba::new(0.2, 0.9, 0.2, 1.0), Rgba::new(0.3, 1.0, 0.3, 1.0)],
            },
        }
    }

    pub fn values() -> [Self; 3] {
        [Self::stone(), Self::dirt(), Self::grass()]
    }

    pub fn get_color(&self, p: f32) -> Rgba<f32> {
        match &self.texture {
            Texture::Colors { vec } => vec[(vec.len().saturating_sub(1) as f32 * p) as usize],
        }
    }

    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(self.group_key.group.len() as u8);
        buf.extend(self.group_key.group.bytes());
        buf.push(self.group_key.key.len() as u8);
        buf.extend(self.group_key.key.bytes());

        let encoded_0 = self.cullable_faces.bits() << 6 | self.has_collider as u8;
        buf.push(encoded_0);

        match &self.texture {
            Texture::Colors { vec } => {
                buf.push(0);
                buf.extend((vec.len() as u16).to_le_bytes());
                for rgba in vec {
                    buf.extend(rgba.r.to_le_bytes());
                    buf.extend(rgba.g.to_le_bytes());
                    buf.extend(rgba.b.to_le_bytes());
                    buf.extend(rgba.a.to_le_bytes());
                }
            }
        }
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        let mut bytes = buf.iter().copied();

        let group_len = bytes.next()? as usize;
        let group_bytes = bytes.by_ref().take(group_len).collect::<Vec<_>>();
        let group_string = String::from_utf8(group_bytes).ok()?;

        let key_len = bytes.next()? as usize;
        let key_bytes = bytes.by_ref().take(key_len).collect::<Vec<_>>();
        let key_string = String::from_utf8(key_bytes).ok()?;

        let group_key = GroupKeyBuf::new(group_string, key_string);

        let encoded_0 = bytes.next()?;
        let has_collider = (encoded_0 >> 6) != 0;
        let cullable_faces = Faces::from_bits_truncate(encoded_0);

        let texture;
        match bytes.next()? {
            0 => {
                let mut vec = Vec::with_capacity(u16::from_le_bytes(bytes.next_chunk::<2>().unwrap()) as usize);
                vec.fill_with(|| Rgba {
                    r: f32::from_le_bytes(bytes.next_chunk::<4>().unwrap()),
                    g: f32::from_le_bytes(bytes.next_chunk::<4>().unwrap()),
                    b: f32::from_le_bytes(bytes.next_chunk::<4>().unwrap()),
                    a: f32::from_le_bytes(bytes.next_chunk::<4>().unwrap()),
                });

                texture = Texture::Colors { vec };
            }
            _ => return None,
        }

        Some(Self {
            group_key,
            has_collider,
            cullable_faces,
            texture,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Texture {
    Colors { vec: Vec<Rgba<f32>> },
}

pub type PaletteCube = Cube<Option<PaletteMaterialId>>;

#[derive(Debug, Clone)]
pub struct Palette {
    vec: Vec<Arc<Material>>,
    named_indices: HashMap<GroupKeyBuf, PaletteMaterialId>,
    cursor: Arc<AtomicUsize>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            named_indices: HashMap::new(),
            cursor: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn insert(&mut self, material: Arc<Material>) -> PaletteMaterialId {
        let group_key = material.group_key.clone();
        if let Some(id) = self.named_indices.get(&group_key) {
            return *id;
        }

        let id = PaletteMaterialId::new(self.vec.len() as u16 + 1).expect("PaletteMaterialId must be non-zero");
        self.named_indices.insert(group_key, id);
        self.vec.push(material);
        id
    }

    pub fn get(&self, id: PaletteMaterialId) -> Option<&Arc<Material>> {
        self.vec.get(id.to_u16() as usize - 1)
    }

    pub fn get_id_by_key<Q>(&self, key: &Q) -> Option<PaletteMaterialId>
    where
        Q: Equivalent<GroupKeyBuf> + Hash,
    {
        self.named_indices.get(key).cloned()
    }

    pub fn update(&self, handle: &ClientChunkHandle) {
        let mut i = self.cursor.load(Ordering::Relaxed);
        while i < self.vec.len() {
            handle.register_material(self.vec[i].clone());
            i += 1;
        }
        self.cursor.store(i, Ordering::SeqCst);
    }

    pub fn materials(&self) -> Iter<'_, Arc<Material>> {
        self.vec.iter()
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PaletteMaterialId(NonZeroU16);

impl PaletteMaterialId {
    pub(crate) fn new(value: u16) -> Option<Self> {
        NonZeroU16::new(value).map(Self)
    }

    pub fn to_u16(self) -> u16 {
        self.0.get()
    }

    pub fn using<T>(self, palette: &Palette, f: impl FnOnce(&Arc<Material>) -> T) -> Option<T> {
        palette.get(self).map(f)
    }
}

pub trait PaletteMaterialOptionExt: Copy {
    fn using<T>(self, palette: &Palette, f: impl FnOnce(&Arc<Material>) -> T) -> Option<T>;

    fn cullable_faces(self, palette: &Palette) -> Faces {
        self.using(palette, |material| material.cullable_faces)
            .unwrap_or(Faces::empty())
    }
}

impl PaletteMaterialOptionExt for Option<PaletteMaterialId> {
    fn using<T>(self, palette: &Palette, f: impl FnOnce(&Arc<Material>) -> T) -> Option<T> {
        self.and_then(|id| id.using(palette, f))
    }
}
