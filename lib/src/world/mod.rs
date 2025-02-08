pub mod camera;
pub mod chunk;
pub mod entity;
pub mod player;
pub mod position;
mod renderer;
pub mod physics;
pub mod transform;

use crate::engine::binding::{Binding, BindingBuilder};
use crate::engine::gpu::Gpu;
use crate::engine::storage::Storage;
use crate::engine::texture::Texture;
use crate::engine::Engine;
use crate::listener::{InputEvent, Listener};
use crate::world::chunk::map::ChunkMap;
use crate::world::chunk::material::Material;
use crate::world::entity::set::EntitySet;
use crate::world::physics::Physics;
use crate::world::player::Player;
use crate::world::position::{ChunkLocalPosition, CubePosition};
use crate::world::renderer::Renderer;
use bytemuck::Zeroable;
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption};
use math::vector::{vec2, vec3, vec3d, vec3f, vec3i, ArrVec2F32};
use std::time::Duration;
use wgpu::{AddressMode, FilterMode, RenderPass, SamplerBindingType, SamplerDescriptor, ShaderStages};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

pub struct World {
    pub(crate) renderer: Renderer,
    chunk_map: ChunkMap,
    entity_set: EntitySet,
    player: Player,
    physics: Physics,
}

impl Listener for World {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.renderer.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        match event {
            InputEvent::Key { code, state } => {
                let f = if state.is_pressed() { 1.0 } else { 0.0 };

                match code {
                    KeyCode::KeyW => self.player.motion.forward = f,
                    KeyCode::KeyS => self.player.motion.backward = f,
                    KeyCode::KeyA => self.player.motion.leftward = f,
                    KeyCode::KeyD => self.player.motion.rightward = f,
                    KeyCode::Space => self.player.motion.upward = f,
                    KeyCode::ShiftLeft => self.player.motion.downward = f,
                    _ => {}
                }
            }
            &InputEvent::MouseMoving { dx, dy } => {
                if self.player.motion.rotation.is_none() {
                    return self.player.motion.rotation = Some((dx, dy));
                }

                let (x, y) = self.player.motion.rotation.as_mut().unwrap();
                *x += dx;
                *y += dy;
            }
            InputEvent::MouseClick { button: MouseButton::Left, state: ElementState::Pressed } => self.player.break_cube += 1,
            _ => {}
        }
    }
}

impl World {
    pub fn create(engine: &Engine) -> Self {
        let renderer = Renderer::create(engine.gpu.clone(), &engine.surface);
        let mut chunk_map = ChunkMap::new(engine.gpu.clone(), 48323);


        for x in -2..2 {
            for y in 0..5 {
                for z in -2..2 {
                    chunk_map.load_chunk(vec3::new(x, y, z));
                }
            }
        }

        let player = Player::new(vec3::new(0., 128., 0.));
        let entity_set = EntitySet::new();
        let physics = Physics::new();

        Self {
            chunk_map,
            renderer,
            player,
            entity_set,
            physics,
        }
    }

    pub fn is_colliding(&self, cuboid: Cuboid) -> bool {
        let min = vec3::new(cuboid.min.x.floor(), cuboid.min.y.floor(), cuboid.min.z.floor());
        let max = vec3::new(cuboid.max.x.ceil(), cuboid.max.y.ceil(), cuboid.max.z.ceil());

        for x in min.x as i32..max.x as i32 {
            for y in min.y as i32..max.y as i32 {
                for z in min.z as i32..max.z as i32 {
                    if let Some(material) = self.get_material(CubePosition(vec3::new(x, y, z))) {
                        if material.can_collide() {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn ray_cast(&self, origin: vec3f, direction: vec3d) -> Option<vec3i> {
        let mut position = origin;
        let step = direction.normalize().cast() * 0.1;
        let mut distance = 0.0;

        while distance < 10.0 {
            let x = position.x.floor() as i32;
            let y = position.y.floor() as i32;
            let z = position.z.floor() as i32;

            if let Some(_) = self.get_material(CubePosition(vec3::new(x, y, z))) {
                return Some(vec3::new(x, y, z).cast());
            }

            position += step;
            distance += step.length();
        }

        None
    }

    pub fn get_material(&self, position: CubePosition) -> Option<Material> {
        let p: ChunkLocalPosition = position.into();
        self.chunk_map.get_chunk(p.chunk)?.get(p.local)
    }

    pub fn set_material(&mut self, position: CubePosition, material: Material) {
        let p: ChunkLocalPosition = position.into();
        self.chunk_map.chunk(p.chunk).set(p.local, material);
    }

    pub fn update(&mut self, dt: Duration) {
        self.physics.update();

        while self.player.break_cube > 0 {
            self.player.break_cube -= 1;

            let Some(pos) = self.ray_cast(self.player.position, self.player.rotation.into_center()) else { continue };

            self.set_material(CubePosition(pos), Material::Air);
        }

        self.player.update(dt);
        if self.renderer.camera().position != self.player.position || self.renderer.camera().rotation != self.player.rotation {
            self.renderer.camera.edit(|c| {
                c.transform.position = self.player.position;
                c.transform.rotation = self.player.rotation;
            });
        }

        /*
        if !self.is_colliding(self.player.get_collision_box()) {
            self.player.position.y -= self.player.motion.speed * dt.as_secs_f32();
            self.player.motion.speed += self.player.motion.acceleration * dt.as_secs_f32();
            self.player.motion.speed = self.player.motion.speed.max(self.player.motion.max_speed);
        } else {
            self.player.motion.speed -= 0.1 * dt.as_secs_f32();
            self.player.motion.speed = self.player.motion.speed.max(0.0);
        }

         */

        for chunk in self.chunk_map.iter_mut() {
            chunk.update(&self.renderer.gpu);
        }

        self.entity_set.tick_all();
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.renderer.pipeline.enable(render_pass);
        self.renderer.quad_mesh.load(render_pass);

        for chunk in self.chunk_map.iter() {
            chunk.render(render_pass);
        }
    }
}

fn build_textures(gpu: &Gpu, builder: BindingBuilder) -> Binding {
    let entries = Material::entries()
        .map(Material::id)
        .map(|id| AtlasEntry {
            texture: image::open(format!("assets/texture/{}.png", id)).unwrap(),
            mip: AtlasEntryMipOption::Repeat,
        })
        .collect::<Vec<_>>();
    let diffuse_atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size: 256,
        mip: Default::default(),
        entries: &entries,
    }).unwrap();
    let texture = diffuse_atlas.textures.into_iter().next().unwrap();
    let image = texture.mip_maps.into_iter().next().unwrap();

    let diffuse_atlas_texture = Texture::from_bytes(gpu, "texture_atlas", image.width(), image.height(), image.as_ref());
    let mut diffuse_atlas_positions = [ArrVec2F32::zeroed(); 128];
    for (i, tex_coord) in diffuse_atlas.texcoords.iter().enumerate() {
        let size = vec2::new(tex_coord.size, tex_coord.size).cast::<f32>();
        diffuse_atlas_positions[i * 4] = (vec2::new(tex_coord.min_x, tex_coord.min_y).cast::<f32>() / size).into();
        diffuse_atlas_positions[i * 4 + 1] = (vec2::new(tex_coord.max_x, tex_coord.min_y).cast::<f32>() / size).into();
        diffuse_atlas_positions[i * 4 + 2] = (vec2::new(tex_coord.min_x, tex_coord.max_y).cast::<f32>() / size).into();
        diffuse_atlas_positions[i * 4 + 3] = (vec2::new(tex_coord.max_x, tex_coord.max_y).cast::<f32>() / size).into();
    }
    let diffuse_atlas_positions_uniform = Storage::create(gpu, "diffuse_atlas_positions", diffuse_atlas_positions);

    builder
        .with_texture(&diffuse_atlas_texture.create_view())
        .with_sampler(SamplerBindingType::Filtering, &gpu.device
            .create_sampler(&SamplerDescriptor {
                label: Some("herbolution_world_texture_sampler"),
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Nearest,
                ..Default::default()
            }),
        )
        .with_storage(ShaderStages::VERTEX, &diffuse_atlas_positions_uniform)
        .finish()
}

pub struct Cuboid {
    pub min: vec3f,
    pub max: vec3f,
}

impl Cuboid {
    pub const fn new(min: vec3f, max: vec3f) -> Self {
        Self { min, max }
    }
}