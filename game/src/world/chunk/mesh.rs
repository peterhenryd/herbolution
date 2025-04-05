use std::ops::Not;
use std::sync::Arc;
use hashbrown::HashMap;
use pulz_arena::Index;
use crate::world::chunk::cube::{Cube, DynamicCubeGrid};
use crate::world::chunk::material::{Material, OptionMaterialExt};
use crate::world::chunk::{in_bounds, linearize, material, LENGTH};
use lib::geometry::cuboid::face::{Face, Faces};
use math::vector::{vec3i, vec3u5, Vec3};

#[derive(Debug)]
pub struct Mesh {
    pub(crate) position: vec3i,
    pub(crate) data: DynamicCubeGrid,
    pub(crate) materials: material::Palette,
    pub(crate) updated_pos: Vec<vec3u5>,
}

impl Mesh {
    pub fn new(position: vec3i) -> Self {
        Self {
            position,
            data: DynamicCubeGrid::new(),
            materials: material::Palette::new(),
            updated_pos: vec![],
        }
    }

    pub fn get(&self, position: vec3u5) -> Option<&Arc<Material>> {
        self.data.get_material(linearize(position), &self.materials)
    }

    pub fn cull_shared_faces(&mut self, other: &mut Mesh) {
        let Some(shared_face) = Face::from_vec3(self.position - other.position) else {
            return;
        };

        let this_matrix = shared_face.inverse().sized_boundary_slice(LENGTH as u8);
        let that_matrix = shared_face.sized_boundary_slice(LENGTH as u8);

        for (x1, x2) in this_matrix.x.zip(that_matrix.x) {
            for (y1, y2) in this_matrix.y.clone().zip(that_matrix.y.clone()) {
                for (z1, z2) in this_matrix.z.clone().zip(that_matrix.z.clone()) {
                    let position1 = vec3u5::new(x1, y1, z1);
                    let position2 = vec3u5::new(x2, y2, z2);
                    let this = self.data.get_material(linearize(position1), &self.materials);
                    let that = other.data.get_material(linearize(position2), &self.materials);

                    if this.is_face_culled() && that.is_face_culled() {
                        self.data.remove_faces_at(linearize(position1), Faces::from(shared_face.inverse()));
                        other.data.remove_faces_at(linearize(position2), Faces::from(shared_face));

                        self.updated_pos.push(position1);
                        other.updated_pos.push(position2);
                    }
                }
            }
        }
    }

    pub fn set(&mut self, position: vec3u5, new_material: Option<&Arc<Material>>) {
        let is_new_material_face_culled = new_material.is_face_culled();
        let id = self.materials.get_or_insert(new_material);

        let i = linearize(position);
        let old_material = self.data.get_material(i, &self.materials);

        self.data.set_material_index(i, id);

        let present = self.data.get_faces_at(i);
        if !old_material.is_face_culled() && is_new_material_face_culled {
            let missing = !present;

            self.data.set_faces_at(i, Faces::all());
            self.remove_neighboring_faces(missing, position);
        }

        if old_material.is_face_culled() && !is_new_material_face_culled {
            self.data.set_faces_at(i, Faces::empty());
            self.add_neighboring_faces(present, position);
        }

        self.updated_pos.push(position);
    }

    fn remove_neighboring_faces(&mut self, faces: Faces, position: vec3u5) {
        faces
            .map(|f| (f, f.into_vec3()))
            .map(|(f, v)| (f, position.cast::<i32>().unwrap() + v))
            //.filter(|(_, x)| in_bounds(*x))
            .filter_map(|(f, v)| v.cast::<u8>().map(|x| (f, x)))
            .filter_map(|(f, v)| vec3u5::try_new(v.x, v.y, v.z).map(|x| (f, x)))
            .for_each(|(f, v)| {
                let index = v.cast().unwrap().linearize(LENGTH);
                self.updated_pos.push(v);

                if self.data.get_material(index, &self.materials).is_face_culled() {
                    self.data.remove_faces_at(linearize(position), Faces::from(f));
                }

                self.data.remove_faces_at(index, Faces::from(f.inverse()));
            });
    }

    fn add_neighboring_faces(&mut self, faces: Faces, position: vec3u5) -> Vec<(Face, Vec3<i32>)> {
        let position = position.cast::<i32>().unwrap();

        let (in_chunk, out_chunk) = faces.not()
            .map(|f| (f, f.into_vec3() + position))
            .partition::<Vec<_>, _>(|&(_, position)| in_bounds(position));

        for (f, Vec3 { x, y, z }) in in_chunk {
            let position = vec3u5::new(x as u8, y as u8, z as u8);
            let index = linearize(position);

            self.updated_pos.push(position);
            self.data.insert_faces_at(index, Faces::from(f.inverse()));
        }

        out_chunk
    }

    pub fn overwrites(&mut self, registry: &material::Registry) -> HashMap<vec3u5, Cube<Index>> {
        let mut overwrites = HashMap::new();
        for position in self.updated_pos.drain(..) {
            let index = linearize(position);
            let cube = self.data.get_cube_at(index);
            let Some(material) = self.materials.get_by_index(cube.material) else { continue };
            registry.index(material).unwrap();
            overwrites.insert(position, cube);
        }
        overwrites
    }
}