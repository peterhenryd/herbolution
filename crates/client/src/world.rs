use crate::chunk::Chunk;
use crate::player;
use engine::video::mem::mesh::MeshId;
use engine::video::r3d::{Meshes3d, Vertex3d};
use engine::video::{r3d, Video};
use engine::{video, Engine};
use game::chunk::channel::ClientChunkChannel;
use lib::TrackMut;
use math::color::{Color, Rgb};
use math::vector::{vec3i, Vec2, Vec3};
use std::collections::HashMap;
use std::time::Duration;

pub struct World {
    chunk_map: HashMap<vec3i, Chunk>,
    channel: ClientChunkChannel,
    pub(crate) render_settings: TrackMut<r3d::World>,
    pub(crate) mesh_ids: MeshIds,
}

impl World {
    pub fn new(channel: ClientChunkChannel, engine: &mut Engine) -> Self {
        Self {
            chunk_map: HashMap::new(),
            channel,
            render_settings: r3d::World {
                ambient_light: Vec3::splat(0.5),
                light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
                fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
                fog_distance: 150.0,
            }.into(),
            mesh_ids: MeshIds::from_insertion_into(engine.video.r3d.meshes()),
        }
    }
    
    pub fn render(&mut self, camera: &player::Camera, frame: &mut video::Frame3d) {
        frame.load_mesh(self.mesh_ids.solid_quad);
        
        for chunk in self.chunk_map.values() {
            chunk.render(camera, frame);
        }
    }

    pub fn update(&mut self, _: Duration, video: &mut Video) {
        if let Some(settings) = self.render_settings.take_modified() {
            video.r3d.update_world(settings);
        }
        
        while let Some((position, receiver, render_flag)) = self.channel.recv_load() {
            let chunk = Chunk::create(&video.handle, position, receiver, render_flag);
            self.chunk_map.insert(position, chunk);
        }

        while let Some(pos) = self.channel.recv_unload() {
            self.chunk_map.remove(&pos);
        }

        for chunk in self.chunk_map.values_mut() {
            chunk.update(&video.handle, video.r3d.texture_coords());
        }
    }
}

pub struct MeshIds {
    pub(crate) solid_quad: MeshId,
    pub(crate) wireframe_quad: MeshId,
}

impl MeshIds {
    fn from_insertion_into(meshes: &mut Meshes3d) -> Self {
        const WIRE_WIDTH: f32 = 0.0025;
        
        Self {
            solid_quad: meshes.create_and_insert(
                &[
                    Vertex3d::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
                    Vertex3d::new(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
                    Vertex3d::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
                    Vertex3d::new(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
                ],
                &[0, 2, 1, 3, 1, 2]
            ),
            wireframe_quad: meshes.create_and_insert(
                &[
                    Vertex3d::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
                    Vertex3d::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
                    Vertex3d::new(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
                    Vertex3d::new(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
                    Vertex3d::new(Vec3::new(-0.5 - WIRE_WIDTH, -0.5 - WIRE_WIDTH, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
                    Vertex3d::new(Vec3::new(-0.5 - WIRE_WIDTH, 0.5 + WIRE_WIDTH, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
                    Vertex3d::new(Vec3::new(0.5 + WIRE_WIDTH, -0.5 - WIRE_WIDTH, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
                    Vertex3d::new(Vec3::new(0.5 + WIRE_WIDTH, 0.5 + WIRE_WIDTH, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
                ],
                &[
                    0, 1, 5, 5, 4, 0,
                    1, 3, 7, 7, 5, 1,
                    3, 2, 6, 6, 7, 3,
                    2, 0, 4, 4, 6, 2,
                ]
            ),
        }
    }
}