use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use bytemuck::cast_slice;
use cg_kiln::Vertex3d;
use prims::vector::Vec3;
use serde::{Deserialize, Serialize};
use wgpu::BufferUsages;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug, Default)]
pub struct Resources {
    pub mesh_buffers: HashMap<String, MeshBuffer>,
}

impl Resources {
    pub fn load_standard(gpu: &kiln::Gpu, asset_path: impl AsRef<Path>) -> Self {
        let mut resources = Self::default();
        resources.read_mesh_buffer(&gpu, asset_path.as_ref(), "quad.toml");
        resources.read_mesh_buffer(&gpu, asset_path.as_ref(), "quad_shell.toml");
        resources.read_mesh_buffer(&gpu, asset_path.as_ref(), "quad_wire.toml");
        resources
    }

    pub fn read_mesh_buffer(&mut self, gpu: &kiln::Gpu, path: &Path, name: &str) {
        let file_path = path.join("mesh").join(name);
        if !file_path.is_file() {
            panic!("Mesh must be a file.")
        }

        let contents = read_to_string(&file_path).unwrap();
        let TomlMesh {
            positions,
            mut normals,
            mut uvs,
            mut tangents,
            indices,
        } = toml::from_str::<TomlMeshFile>(&contents)
            .unwrap()
            .mesh;

        fn fill<T: Copy>(vec: &mut Vec<T>, target_len: usize) {
            let start_len = vec.len();
            let mut i = 0;
            while vec.len() < target_len {
                let value = vec[i % start_len];
                vec.push(value);
                i += 1;
            }
        }

        let target_len = positions.len();
        fill(&mut normals, target_len);
        fill(&mut uvs, target_len);
        fill(&mut tangents, target_len);

        let vertices = positions
            .into_iter()
            .zip(normals)
            .zip(uvs)
            .map(|((pos, normal), uv)| Vertex3d {
                pos: pos.into(),
                uv: uv.into(),
                normal: normal.into(),
                tangent: Vec3::ONE,
            })
            .collect::<Vec<_>>();

        let file_stem = file_path
            .file_stem()
            .map(|x| x.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.mesh_buffers.insert(
            file_stem,
            MeshBuffer {
                vertex_buffer: gpu
                    .as_device()
                    .create_buffer_init(&BufferInitDescriptor {
                        label: None,
                        contents: cast_slice(&vertices),
                        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    }),
                index_buffer: gpu
                    .as_device()
                    .create_buffer_init(&BufferInitDescriptor {
                        label: None,
                        contents: cast_slice(&indices),
                        usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                    }),
                index_count: indices.len() as u32,
            },
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TomlMeshFile {
    mesh: TomlMesh,
}

#[derive(Debug, Serialize, Deserialize)]
struct TomlMesh {
    positions: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    tangents: Vec<[f32; 3]>,
    indices: Vec<u16>,
}
