use std::fmt::Debug;

use engine::sculptor;
use gpu::{Mesh, MeshId};

/// A utility structure that holds the mesh handles used for rendering the herbolution_game.
#[derive(Debug)]
pub struct MeshIds {
    pub(crate) solid_quad: MeshId,
    pub(crate) wire_quad: MeshId,
}

impl MeshIds {
    pub fn from_insertion_into(meshes: &mut sculptor::Meshes) -> Self {
        Self {
            solid_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, "assets/mesh/quad.toml")),
            wire_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, "assets/mesh/quad_wire.toml")),
        }
    }
}
