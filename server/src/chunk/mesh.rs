use std::iter::zip;
use std::ops::{Not, Range};

use lib::point::ChunkPt;
use lib::spatial::{CubeFace, CubeFaces};
use lib::vector::{vec3u5, Vec3};
use lib::world::{CHUNK_LENGTH, CHUNK_VOLUME};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::chunk::cube::Cube;
use crate::chunk::material::{Palette, PaletteCube, PaletteMaterialId, PaletteMaterialOptionExt};

#[derive(Debug, Clone)]
pub struct CubeMesh {
    pub position: ChunkPt,
    pub(crate) data: Box<[PaletteCube; CHUNK_VOLUME]>,
    pub(crate) updated_positions: Vec<vec3u5>,
    pub(crate) exposed_faces: CubeFaces,
    pub(crate) palette: Palette,
}

impl CubeMesh {
    pub fn new(position: ChunkPt) -> Self {
        Self {
            position,
            data: Box::new([Cube::new(None); CHUNK_VOLUME]),
            updated_positions: vec![],
            exposed_faces: CubeFaces::all(),
            palette: Palette::new(),
        }
    }

    pub fn get(&self, position: vec3u5) -> Option<PaletteMaterialId> {
        self.data[position.linearize()].material
    }

    pub fn cull_shared_face(&mut self, other: &CubeMesh) {
        let Some(face) = CubeFace::from_normal(other.position.0 - self.position.0) else {
            return;
        };
        let inverse_face = face.inverse();

        fn sized_boundary_slice(face: CubeFace) -> Vec3<Range<u8>> {
            let l = CHUNK_LENGTH as u8;
            match face {
                CubeFace::East => Vec3::new(l - 1..l, 0..l, 0..l),
                CubeFace::West => Vec3::new(0..1, 0..l, 0..l),
                CubeFace::Up => Vec3::new(0..l, l - 1..l, 0..l),
                CubeFace::Down => Vec3::new(0..l, 0..1, 0..l),
                CubeFace::North => Vec3::new(0..l, 0..l, l - 1..l),
                CubeFace::South => Vec3::new(0..l, 0..l, 0..1),
            }
        }

        let boundary = sized_boundary_slice(face);
        let adj_boundary = sized_boundary_slice(inverse_face);
        for (x, xa) in zip(boundary.x, adj_boundary.x) {
            for (y, ya) in zip(boundary.y.clone(), adj_boundary.y.clone()) {
                for (z, za) in zip(boundary.z.clone(), adj_boundary.z.clone()) {
                    let position = vec3u5::new(x, y, z);
                    let position_adj = vec3u5::new(xa, ya, za);

                    let cube = &mut self.data[position.linearize()];
                    let adj_cube = &other.data[position_adj.linearize()];

                    let cullable_faces = cube.material.cullable_faces(&self.palette);
                    let adj_cullable_faces = adj_cube.material.cullable_faces(&other.palette);
                    if cullable_faces.contains(face) && adj_cullable_faces.contains(inverse_face) {
                        cube.flags.remove_faces(face);
                        self.updated_positions.push(position);
                    }
                }
            }
        }
    }

    pub fn cull_shared_faces(&mut self, other: &mut CubeMesh) {
        let Some(shared_face) = CubeFace::from_normal(other.position.0 - self.position.0) else {
            return;
        };
        let other_shared_face = shared_face.inverse();

        let matric = boundary(shared_face);
        let other_matric = boundary(other_shared_face);

        let mut is_exposed = false;
        for (x1, x2) in matric.x.zip(other_matric.x) {
            for (y1, y2) in matric.y.clone().zip(other_matric.y.clone()) {
                for (z1, z2) in matric.z.clone().zip(other_matric.z.clone()) {
                    let position = vec3u5::new(x1, y1, z1);
                    let other_position = vec3u5::new(x2, y2, z2);
                    let cube = &mut self.data[position.linearize()];
                    let other_cube = &mut other.data[other_position.linearize()];

                    if !cube
                        .material
                        .cullable_faces(&self.palette)
                        .is_empty()
                        && !other_cube
                            .material
                            .cullable_faces(&other.palette)
                            .is_empty()
                    {
                        cube.flags
                            .remove_faces(CubeFaces::from(shared_face));
                        other_cube
                            .flags
                            .remove_faces(CubeFaces::from(other_shared_face));

                        self.updated_positions.push(position);
                        other.updated_positions.push(other_position);
                    } else {
                        is_exposed = true;
                    }
                }
            }
        }

        self.exposed_faces
            .set(shared_face.into(), is_exposed);
        other
            .exposed_faces
            .set(other_shared_face.into(), is_exposed);
    }

    pub fn set(&mut self, position: vec3u5, new_material: Option<PaletteMaterialId>) {
        let i = position.linearize();
        let old_material = self.data[i].material;

        if new_material == old_material {
            return;
        }

        self.data[i].material = new_material;

        if old_material
            .cullable_faces(&self.palette)
            .is_empty()
            && !new_material
                .cullable_faces(&self.palette)
                .is_empty()
        {
            let missing = self.data[i].flags.faces().not();
            self.data[i].flags.set_opaque(CubeFaces::all());
            self.remove_neighboring_faces(missing, position);
        }

        if !old_material
            .cullable_faces(&self.palette)
            .is_empty()
            && new_material
                .cullable_faces(&self.palette)
                .is_empty()
        {
            let present = self.data[i].flags.faces();
            self.data[i].flags.set_opaque(CubeFaces::none());
            self.add_neighboring_faces(present, position);
        }

        let (x, y, z) = (position.x(), position.y(), position.z());
        if x == 0 || x == 15 && new_material.is_none() {
            self.exposed_faces.set(
                CubeFace::from_normal(Vec3::new(if x == 0 { -1 } else { 1 }, 0, 0))
                    .unwrap()
                    .into(),
                true,
            );
        }
        if y == 0 || y == 15 && new_material.is_none() {
            self.exposed_faces.set(
                CubeFace::from_normal(Vec3::new(0, if y == 0 { -1 } else { 1 }, 0))
                    .unwrap()
                    .into(),
                true,
            );
        }
        if z == 0 || z == 15 && new_material.is_none() {
            self.exposed_faces.set(
                CubeFace::from_normal(Vec3::new(0, 0, if y == 0 { -1 } else { 1 }))
                    .unwrap()
                    .into(),
                true,
            );
        }

        self.updated_positions.push(position);
    }

    pub fn fill(&mut self, material: Option<PaletteMaterialId>) {
        self.data.par_iter_mut().for_each(|cube| {
            *cube = Cube::new(material);
        });

        for face in CubeFace::values() {
            let boundary = boundary(face);
            for x in boundary.x {
                for y in boundary.y.clone() {
                    for z in boundary.z.clone() {
                        let position = vec3u5::new(x, y, z);
                        self.data[position.linearize()]
                            .flags
                            .insert_faces(face);
                        self.updated_positions.push(position);
                    }
                }
            }
        }
    }

    fn remove_neighboring_faces(&mut self, faces: CubeFaces, position: vec3u5) {
        faces
            .iter()
            .map(|f| (f, f.normal()))
            .map(|(f, v)| (f, position.try_cast::<i32>().unwrap() + v))
            .filter_map(|(f, v)| v.try_cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u5::try_from(v).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.linearize();
                self.updated_positions.push(v);

                if !self.data[index]
                    .material
                    .cullable_faces(&self.palette)
                    .is_empty()
                {
                    self.data[position.linearize()]
                        .flags
                        .remove_faces(f);
                }

                self.data[index].flags.remove_faces(f.inverse());
            });
    }

    fn add_neighboring_faces(&mut self, faces: CubeFaces, position: vec3u5) {
        let position = position.try_cast::<i32>().unwrap();

        let in_chunk = faces.not().iter().filter_map(|a| {
            (a.normal() + position)
                .try_cast::<u8>()
                .map(|v| vec3u5::try_from(v))
                .flatten()
                .map(|b| (a, b))
        });

        for (f, position) in in_chunk {
            self.updated_positions.push(position);
            self.data[position.linearize()]
                .flags
                .insert_faces(f.inverse());
        }
    }
}

fn boundary(face: CubeFace) -> Vec3<Range<u8>> {
    let l = CHUNK_LENGTH as u8;
    match face {
        CubeFace::East => Vec3::new(l - 1..l, 0..l, 0..l),
        CubeFace::West => Vec3::new(0..1, 0..l, 0..l),
        CubeFace::Up => Vec3::new(0..l, l - 1..l, 0..l),
        CubeFace::Down => Vec3::new(0..l, 0..1, 0..l),
        CubeFace::North => Vec3::new(0..l, 0..l, l - 1..l),
        CubeFace::South => Vec3::new(0..l, 0..l, 0..1),
    }
}
