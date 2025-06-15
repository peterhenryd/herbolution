use engine::video::gpu::{Handle, Index, Mesh, MeshId, Vertex, mesh};
use engine::video::v3d;
use math::vector::{Vec2, Vec3};

/// A utility structure that holds the mesh handles used for rendering the game.
#[derive(Debug)]
pub struct MeshIds {
    pub(crate) solid_quad: MeshId,
    pub(crate) wireframe_quad: MeshId,
}

impl MeshIds {
    pub fn from_insertion_into(meshes: &mut v3d::Meshes) -> Self {
        Self {
            solid_quad: meshes.create_and_insert_from(mesh::c_quad),
            wireframe_quad: meshes.create_and_insert_from(wireframe_quad_fn(0.0025)),
        }
    }
}

fn wireframe_quad_fn<V: Vertex, I: Index>(wire_width: f32) -> impl FnOnce(&Handle) -> Mesh<V, I> {
    move |handle| {
        Mesh::create(
            handle,
            &[
                V::new_3d(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
                V::new_3d(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
                V::new_3d(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
                V::new_3d(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
                V::new_3d(
                    Vec3::new(-0.5 - wire_width, -0.5 - wire_width, 0.5),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec2::new(0.0, 0.0),
                ),
                V::new_3d(
                    Vec3::new(-0.5 - wire_width, 0.5 + wire_width, 0.5),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec2::new(1.0, 0.0),
                ),
                V::new_3d(
                    Vec3::new(0.5 + wire_width, -0.5 - wire_width, 0.5),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec2::new(0.0, 1.0),
                ),
                V::new_3d(
                    Vec3::new(0.5 + wire_width, 0.5 + wire_width, 0.5),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec2::new(1.0, 1.0),
                ),
            ],
            &[0, 1, 5, 5, 4, 0, 1, 3, 7, 7, 5, 1, 3, 2, 6, 6, 7, 3, 2, 0, 4, 4, 6, 2].map(I::new_u16),
        )
    }
}
