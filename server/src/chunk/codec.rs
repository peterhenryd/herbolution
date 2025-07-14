use lib::point::ChunkPt;
use lib::world::CHUNK_VOLUME;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::chunk::material::{Palette, PaletteMaterialId};
use crate::chunk::mesh::CubeMesh;

pub struct CubeGrid {
    data: Box<[Option<PaletteMaterialId>]>,
    palette: Palette,
}

impl CubeGrid {
    pub fn new(palette: Palette) -> Self {
        Self {
            data: Box::new([None; CHUNK_VOLUME]),
            palette,
        }
    }

    pub fn from_mesh(cube_mesh: &CubeMesh) -> Self {
        let mut mesh = Self::new(cube_mesh.palette.clone());

        mesh.data
            .par_iter_mut()
            .zip(cube_mesh.data.par_iter())
            .for_each(|(dest, src)| {
                *dest = src.material;
            });

        mesh
    }

    pub fn to_mesh(&self, position: ChunkPt) -> CubeMesh {
        let mut mesh = CubeMesh::new(position);

        mesh.data
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, dest)| {
                dest.material = self.data[index];
            });

        mesh
    }

    pub fn encode(&self, buf: &mut Vec<u8>) {
        encode_palette(&self.palette, buf);
        encode_cubes(self.data.iter().copied(), buf);
    }

    pub fn decode(bytes: &[u8]) -> Self {
        let mut i = 0;
        let palette = Palette::new();

        let mut mesh = Self::new(palette);

        for &[count, m0, m1] in bytes[i..].array_chunks::<3>() {
            let material = PaletteMaterialId::new(u16::from_le_bytes([m0, m1]));

            for _ in 0..count {
                mesh.data[i] = material;
                i += 1;
            }
        }

        mesh
    }
}

fn encode_palette(palette: &Palette, buf: &mut Vec<u8>) {
    for (i, material) in palette.materials().enumerate() {
        buf.extend((i as u16).to_le_bytes());
        material.encode(buf);
    }
}

fn encode_cubes(materials: impl Iterator<Item = Option<PaletteMaterialId>>, buf: &mut Vec<u8>) {
    let mut count = 0;
    let mut current = None;

    for material in materials {
        if current != material || count == u8::MAX {
            buf.push(count);
            buf.extend(
                current
                    .map(|id| id.to_u16())
                    .unwrap_or(0)
                    .to_le_bytes(),
            );

            current = material;
            count = 1;
        } else {
            count += 1;
        }
    }

    buf.push(count);
    buf.extend(
        current
            .map(|id| id.to_u16())
            .unwrap_or(0)
            .to_le_bytes(),
    );
}
