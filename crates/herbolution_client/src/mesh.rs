use engine::sculptor;
use gpu::{Mesh, MeshId};
use std::fmt::Debug;
use std::path::Path;

/// A utility structure that holds the mesh handles used for rendering the herbolution_game.
#[derive(Debug)]
pub struct MeshIds {
    pub(crate) solid_quad: MeshId,
    pub(crate) wire_quad: MeshId,
}

impl MeshIds {
    pub fn from_insertion_into(meshes: &mut sculptor::Meshes, asset_path: &Path) -> Self {
        Self {
            solid_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad.toml"))),
            wire_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad_wire.toml"))),
        }
    }
}
