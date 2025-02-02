use crate::game::chunk::material::Material;
use glam::IVec3;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::mem::transmute;
use std::ops::Range;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_TOTAL: usize = CHUNK_SIZE.pow(3);

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub position: IVec3,
    pub(crate) data: [Material; CHUNK_TOTAL],
}

impl Serialize for ChunkSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = Vec::with_capacity(size_of::<i32>() * 3 + size_of::<u16>() * CHUNK_TOTAL);
        bytes.extend(self.position.x.to_le_bytes());
        bytes.extend(self.position.y.to_le_bytes());
        bytes.extend(self.position.z.to_le_bytes());
        bytes.extend_from_slice(unsafe { transmute::<&[Material], &[u8]>(&self.data) });

        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for ChunkSection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ChunkSectionVisitor)
    }
}

struct ChunkSectionVisitor;

impl<'de> Visitor<'de> for ChunkSectionVisitor {
    type Value = ChunkSection;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a chunk section")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let pos_data = v
            .iter()
            .take(size_of::<i32>() * 3)
            .copied()
            .array_chunks::<{ size_of::<i32>() }>()
            .map(|arr| i32::from_le_bytes(arr.try_into().unwrap()))
            .array_chunks::<3>()
            .next()
            .unwrap();

        let position = IVec3::from(pos_data);
        let data = v
            .into_iter()
            .skip(12)
            .array_chunks::<2>()
            .map(|[&a, &b]| u16::from_le_bytes([a, b]))
            .filter_map(|n| Material::try_from(n).ok())
            .array_chunks::<CHUNK_TOTAL>()
            .next()
            .unwrap();

        Ok(ChunkSection { position, data })
    }
}

impl ChunkSection {
    pub fn new(position: IVec3) -> Self {
        Self {
            position,
            data: [Material::Air; CHUNK_TOTAL],
        }
    }

    pub fn set_column(&mut self, x: usize, z: usize, y_range: Range<usize>, material: Material) {
        let m = x * CHUNK_SIZE * CHUNK_SIZE + z;
        for y in y_range {
            self.data[m + CHUNK_SIZE * y] = material;
        }
    }
}
