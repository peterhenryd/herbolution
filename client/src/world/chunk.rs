use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::take;
use std::ops::Mul;
use std::sync::Arc;

use fastrand::Rng;
use lib::aabb::Aabb3;
use lib::collections::Mailbox;
use lib::point::ChunkPt;
use lib::spatial::{CubeFace, PerFace};
use lib::vector::{vec3i, vec3u5, Vec3, Vec4};
use lib::world::{CHUNK_LENGTH, CHUNK_VOLUME};
use parking_lot::{RwLock, RwLockReadGuard};
use server::chunk::cube::Cube;
use server::chunk::handle::{ChunkCube, GameChunkHandle};
use server::chunk::material::{Palette, PaletteCube};
use wgpu::BufferUsages;

use crate::video::gpu;
use crate::video::resource::GrowBuffer;
use crate::video::world::chisel::Chisel;
use crate::video::world::Instance3d;
use crate::world::player::PlayerCamera;

type ChunkShell = [Option<Arc<RwLock<ChunkData>>>; 27];

type ChunkShellGuard<'a> = [Option<RwLockReadGuard<'a, ChunkData>>; 27];

#[derive(Debug)]
pub struct ChunkMap {
    pub(crate) map: HashMap<ChunkPt, Chunk>,
    remesh_queue: Vec<(ChunkPt, Vec<Instance3d>)>,
    mesh_return: Mailbox<(ChunkPt, Vec<Instance3d>)>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            remesh_queue: vec![],
            mesh_return: Mailbox::default(),
        }
    }

    pub fn update(&mut self, handle: &gpu::Handle) {
        for (position, instances) in &self.mesh_return {
            if let Some(chunk) = self.map.get_mut(&position) {
                chunk.submit_mesh(handle, instances);
            }
        }

        self.remesh_queue.clear();
        for (position, chunk) in &mut self.map {
            let updated = chunk.apply_updates_from_server();
            if !updated {
                continue;
            }

            chunk.cached_quad_instances.clear();
            let instances = take(&mut chunk.cached_quad_instances);
            self.remesh_queue.push((*position, instances));
        }

        for (chunk_position, mut instances) in self.remesh_queue.drain(..) {
            let return_tx = self.mesh_return.sender();
            let chunk_shell = create_chunk_shell(&self.map, chunk_position);

            rayon::spawn(move || {
                generate_mesh(&chunk_shell, &mut instances);

                let _ = return_tx.send((chunk_position, instances));
            })
        }
    }

    pub fn render(&self, camera: &PlayerCamera, chisel: &mut Chisel) {
        for chunk in self.map.values() {
            chunk.render(camera, chisel);
        }
    }
}

// TODO: cache neighboring cube solidity for ao calculations instead of querying 26 neighbors every time

#[derive(Debug)]
pub struct Chunk {
    position: ChunkPt,
    handle: GameChunkHandle,
    cached_quad_instances: Vec<Instance3d>,
    mesh: GrowBuffer<Instance3d>,
    data: Arc<RwLock<ChunkData>>,
}

#[derive(Debug)]
struct ChunkData {
    position: ChunkPt,
    cubes: Box<[PaletteCube; CHUNK_VOLUME]>,
    palette: Palette,
}

impl Chunk {
    pub fn create(gpu: &gpu::Handle, position: ChunkPt, handle: GameChunkHandle) -> Self {
        let mesh = GrowBuffer::empty(gpu, BufferUsages::VERTEX | BufferUsages::COPY_DST);

        Self {
            position,
            handle,
            cached_quad_instances: vec![],
            data: Arc::new(RwLock::new(ChunkData {
                position,
                cubes: Box::new([Cube::new(None); CHUNK_VOLUME]),
                palette: Palette::new(),
            })),
            mesh,
        }
    }

    pub fn render(&self, camera: &PlayerCamera, chisel: &mut Chisel) {
        if !camera
            .frustum
            .contains_cube(self.position.0.cast(), CHUNK_LENGTH as f32)
        {
            return;
        }

        if !self.handle.is_rendered() {
            return;
        }

        chisel.render_each(&self.mesh);
    }

    fn apply_updates_from_server(&mut self) -> bool {
        if self.handle.cube_update.is_empty() {
            return false;
        }

        let mut data = self.data.write();
        while let Some(update) = self.handle.next_cube_update() {
            for ChunkCube { position, cube } in update.overwrites {
                data.cubes[position.linearize()] = cube;
            }
        }

        while let Some(update) = self.handle.next_palette_update() {
            data.palette.insert(update.material);
        }

        true
    }

    fn submit_mesh(&mut self, handle: &gpu::Handle, instances: Vec<Instance3d>) {
        self.cached_quad_instances = instances;
        self.mesh
            .write(handle, &self.cached_quad_instances);
    }
}

fn create_chunk_shell(map: &HashMap<ChunkPt, Chunk>, position: ChunkPt) -> ChunkShell {
    let mut shell = [const { None }; 27];

    let mut i = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let pt = position + Vec3::new(x, y, z);

                let Some(chunk) = map.get(&pt) else {
                    i += 1;
                    continue;
                };

                shell[i] = Some(chunk.data.clone());
                i += 1;
            }
        }
    }

    shell
}

fn generate_mesh(shell: &ChunkShell, instances: &mut Vec<Instance3d>) {
    let center_chunk = shell[13].as_ref().unwrap();
    let center_chunk = center_chunk.read();

    let mut hasher = DefaultHasher::new();
    center_chunk.position.hash(&mut hasher);
    let mut rng = Rng::with_seed(hasher.finish());

    let chunk_position = center_chunk
        .position
        .0
        .mul(CHUNK_LENGTH as i32)
        .cast::<f32>();

    let shell_guard = shell
        .each_ref()
        .map(|chunk| chunk.as_ref().map(|x| x.read()));

    let (mut cached_material, mut prev_material_id) = (None, None);
    for x in 0..CHUNK_LENGTH {
        for z in 0..CHUNK_LENGTH {
            for y in 0..CHUNK_LENGTH {
                let position = vec3u5::new(x as u8, y as u8, z as u8);
                let cube = center_chunk.cubes[position.linearize()];

                let perms = PerFace::mapped(|_| rng.f32());
                if let Some(material_id) = cube.material {
                    if Some(material_id) != prev_material_id {
                        prev_material_id = Some(material_id);
                        cached_material = center_chunk.palette.get_by_id(material_id);
                    }

                    let Some(material) = &cached_material else {
                        continue;
                    };

                    for face in cube.flags.faces() {
                        let color = material.get_color(perms[face]);
                        let ao = facial_ao(&shell_guard, face, position.cast());

                        instances.push(Instance3d::new(chunk_position + position.cast(), face.rotation(), Vec3::ONE, color, 0, ao));
                    }
                }
            }
        }
    }
}

fn is_cube_present(shell: &ChunkShellGuard, position: vec3i) -> bool {
    let chunk_offset = position.div_euclid(Vec3::splat(CHUNK_LENGTH as i32));

    if !Aabb3::new(-Vec3::ONE, Vec3::ONE).contains(chunk_offset) {
        return false;
    }

    let index = (chunk_offset + 1).linearize(3) as usize;
    let Some(target_chunk) = &shell[index] else {
        return false;
    };

    let local_position = position.rem_euclid(Vec3::splat(CHUNK_LENGTH as i32));
    let Some(local_position) = local_position
        .try_cast::<u8>()
        .map(vec3u5::try_from)
        .flatten()
    else {
        return false;
    };

    target_chunk.cubes[local_position.linearize()]
        .material
        .is_some()
}

fn vertex_ao(s1: bool, s2: bool, c: bool) -> u8 {
    if s1 && s2 { 3 } else { s1 as u8 + s2 as u8 + c as u8 }
}

fn facial_ao(shell: &ChunkShellGuard, face: CubeFace, position: vec3i) -> Vec4<f32> {
    let (u, v, n) = face.orthonormal_basis();

    let tl = is_cube_present(shell, position + n - v - u);
    let tc = is_cube_present(shell, position + n - v);
    let tr = is_cube_present(shell, position + n - v + u);
    let ml = is_cube_present(shell, position + n - u);
    let mr = is_cube_present(shell, position + n + u);
    let bl = is_cube_present(shell, position + n + v - u);
    let bc = is_cube_present(shell, position + n + v);
    let br = is_cube_present(shell, position + n + v + u);

    let occlusion_bl = vertex_ao(ml, tc, tl);
    let occlusion_tl = vertex_ao(ml, bc, bl);
    let occlusion_br = vertex_ao(mr, tc, tr);
    let occlusion_tr = vertex_ao(mr, bc, br);

    let ao_amount = Vec4::new(occlusion_bl, occlusion_tl, occlusion_br, occlusion_tr).cast() / 3.0;
    let ao_factor = -ao_amount.cast::<f32>() + 1.0;

    Vec4::max(ao_factor, Vec4::splat(0.15))
}
