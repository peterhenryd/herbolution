use crate::gpu::binding::{AutoPayload, BindGroupItem, Payload, Texture, UniformBuffer, UniqueBindGroup};
use crate::gpu::ext::{BindGroupExt, RenderPipelineExt, RenderPipelineOptions};
use crate::gpu::geometry::{InstanceBuffer, Mesh, Primitive};
use crate::gpu::renderer::{InstancedMesh, RenderGroup, RenderType, Renderer};
use crate::gpu::Gpu;
use crate::uniform::{Camera, Frustum, World};
use bytemuck::{Pod, Zeroable};
use image::DynamicImage;
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasMipOption};
use lib::geo::face::Face;
use lib::Modify;
use math::color::{Color, ColorConsts, Rgba};
use math::proj::Perspective;
use math::rotation::Quat;
use math::size::Size2;
use math::vector::{vec2f, vec3d, vec3f, vec3i, vec4f, Vec2, Vec3};
use wgpu::{include_wgsl, vertex_attr_array, AddressMode, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, BufferAddress, FilterMode, RenderPass, RenderPipeline, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub struct State3d {
    renderer: Renderer<RenderType3d>,
    pub(crate) camera: Modify<Camera<Perspective>>,
    pub(crate) frustum: Frustum,
    world: Modify<World>,
    uniforms: Uniforms,
    meshes: Meshes,
    highlighted_tile: InstanceBuffer,
    sky_box: InstanceBuffer,
    pub(crate) texture_positions: Vec<(vec2f, f32)>,
}

impl State3d {
    pub fn create(gpu: &Gpu, size: Size2<u32>) -> Self {
        let CreatedTextures {
            bind_group: texture_bind_group,
            positions: texture_positions,
        } = create_textures(gpu);
        let camera: Modify<_> = Camera::new(Vec3::ZERO, Perspective::from(size)).into();
        let frustum = Frustum::new(camera.view_proj_matrix());
        let world: Modify<_> = World::new().into();
        let uniforms = Uniforms::create(gpu, &*camera, &*world);
        let renderer = Renderer::create(
            gpu,
            vec![
                BindGroup::build().append(&uniforms.camera, ShaderStages::VERTEX_FRAGMENT).finish(gpu),
                BindGroup::build().append(&uniforms.world, ShaderStages::FRAGMENT).finish(gpu),
                texture_bind_group,
            ],
            &[RenderType3d::WORLD, RenderType3d::SKY_BOX],
        );
        let meshes = Meshes::create(gpu);

        let highlighted_tile = InstanceBuffer::create::<Instance3dPayload>(gpu, &[]);
        let mut sky_box = InstanceBuffer::create(gpu, &[Instance3dPayload::zeroed(); 6]);
        update_skybox(&mut sky_box, &texture_positions, gpu, camera.position);

        Self { renderer, camera, frustum, world, uniforms, meshes, highlighted_tile, sky_box, texture_positions }
    }

    pub fn update(&mut self, gpu: &Gpu) {
        if let Some(value) = self.camera.take_modified() {
            self.uniforms.camera.write(gpu, value);
            self.frustum = Frustum::new(value.view_proj_matrix());
            update_skybox(&mut self.sky_box, &self.texture_positions, gpu, value.position);
        }

        if let Some(value) = self.world.take_modified() {
            self.uniforms.world.write(gpu, value);
        }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.camera.proj.set_size(size);
    }

    pub fn render(
        &self,
        render_pass: &mut RenderPass,
        back_tiles: &[&InstanceBuffer],
    ) {
        self.renderer.render_group(render_pass, RenderGroup {
            render_type: RenderType3d::SKY_BOX,
            instanced_meshes: &[
                InstancedMesh {
                    mesh: &self.meshes.tile_quad,
                    instance_buffers: &[&self.sky_box],
                },
            ],
        });
        self.renderer.render_group(render_pass, RenderGroup {
            render_type: RenderType3d::WORLD,
            instanced_meshes: &[
                InstancedMesh {
                    mesh: &self.meshes.tile_quad,
                    instance_buffers: back_tiles,
                },
                InstancedMesh {
                    mesh: &self.meshes.tile_highlight,
                    instance_buffers: &[&self.highlighted_tile],
                },
            ],
        });
    }

    pub fn set_highlighted_tile(&mut self, gpu: &Gpu, position: Option<vec3i>) {
        let Some(position) = position else {
            self.highlighted_tile.write::<Instance3dPayload>(gpu, &[]);
            return;
        };

        let position = position.cast().unwrap();
        let instances = Face::entries()
            .map(|x| x.into_quat())
            .map(|rotation| {
                Instance3d {
                    position,
                    rotation,
                    color: Rgba::BLACK,
                    ..Default::default()
                }.payload()
            })
            .collect::<Vec<_>>();
        self.highlighted_tile.write(gpu, &instances);
    }
}

fn update_skybox(instances: &mut InstanceBuffer, _: &[(vec2f, f32)], gpu: &Gpu, position: vec3d) {
    //let (tp1, ts1) = texture_positions[4];
    //let (tp2, ts2) = texture_positions[5];
    //let (tp3, ts3) = texture_positions[6];
    //let (tp4, ts4) = texture_positions[7];
    //let (tp5, ts5) = texture_positions[8];
    //let (tp6, ts6) = texture_positions[9];

    let color = Rgba::<u8>::from_rgb(177, 242, 255).into();
    instances.write(gpu, &[
        Instance3d {
            position,
            rotation: Face::North.into_quat(),
            color,
            // tex_pos: tp1,
            // tex_size: ts1,
            ..Default::default()
        }.payload(),
        Instance3d {
            position,
            rotation: Face::South.into_quat(),
            color,
            // tex_pos: tp2,
            // tex_size: ts2,
            ..Default::default()
        }.payload(),
        Instance3d {
            position,
            rotation: Face::Up.into_quat(),
            color,
            // tex_pos: tp3,
            // tex_size: ts3,
            ..Default::default()
        }.payload(),
        Instance3d {
            position,
            rotation: Face::Down.into_quat(),
            color,
            // tex_pos: tp4,
            // tex_size: ts4,
            ..Default::default()
        }.payload(),
        Instance3d {
            position,
            rotation: Face::West.into_quat(),
            color,
            // tex_pos: tp5,
            // tex_size: ts5,
            ..Default::default()
        }.payload(),
        Instance3d {
            position,
            rotation: Face::East.into_quat(),
            color,
            // tex_pos: tp6,
            // tex_size: ts6,
            ..Default::default()
        }.payload(),
    ]);
}

pub struct Uniforms {
    pub camera: UniformBuffer<Camera<Perspective>>,
    pub world: UniformBuffer<World>,
}

impl Uniforms {
    pub fn create(gpu: &Gpu, camera: &Camera<Perspective>, world: &World) -> Self {
        Self {
            camera: UniformBuffer::create(gpu, camera),
            world: UniformBuffer::create(gpu, world),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Vertex3d {
    pub position: vec3f,
    pub normal: vec3f,
    pub texture_offset: vec2f,
}

impl Primitive for Vertex3d {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2,
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex3d>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };
}

impl Vertex3d {
    pub const fn new(position: vec3f, texture_offset: vec2f, normal: vec3f) -> Self {
        Self { position, texture_offset, normal }
    }
}

impl AutoPayload for Vertex3d {}

pub struct Instance3d {
    pub position: vec3d,
    pub rotation: Quat,
    pub texture_position: vec2f,
    pub texture_size: f32,
    pub color: Rgba<f32>,
}

impl Primitive for Instance3d {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Sint32x3,
        7 => Float32x3,
        8 => Float32x4,
        9 => Float32x3,
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Instance3dPayload>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };
}

impl Payload for Instance3d {
    type Output = Instance3dPayload;

    fn payload(&self) -> Self::Output {
        let rotation_matrix = self.rotation.to_matrix();
        let world_position_int = self.position.cast::<i32>().unwrap();
        let world_position_frac = self.position.fract().cast().unwrap();
        
        Instance3dPayload {
            model_0: rotation_matrix.x,
            model_1: rotation_matrix.y,
            model_2: rotation_matrix.z,
            world_position_int,
            world_position_frac,
            color: self.color,
            texture: self.texture_position.extend(self.texture_size),
        }
    }
}

impl Default for Instance3d {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            texture_position: Vec2::ZERO,
            texture_size: 1.0,
            color: Rgba::TRANSPARENT,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance3dPayload {
    pub model_0: vec4f,
    pub model_1: vec4f,
    pub model_2: vec4f,
    pub world_position_int: vec3i,
    pub world_position_frac: vec3f,
    pub color: Rgba<f32>,
    pub texture: vec3f,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderType3d {
    pub cull_mode: wgpu::Face,
    pub depth_write_enabled: bool,
}

impl RenderType3d {
    pub const WORLD: Self = Self {
        cull_mode: wgpu::Face::Front,
        depth_write_enabled: true,
    };
    pub const SKY_BOX: Self = Self {
        cull_mode: wgpu::Face::Back,
        depth_write_enabled: false,
    };
}

impl RenderType for RenderType3d {
    type Vertex = Vertex3d;
    type Instance = Instance3d;

    fn create_render_pipeline(&self, gpu: &Gpu, bind_group_layouts: &[&BindGroupLayout]) -> RenderPipeline {
        let shader_module = gpu.device.create_shader_module(include_wgsl!("shader.wgsl"));
        RenderPipeline::create(gpu, RenderPipelineOptions {
            bind_group_layouts,
            shader_module: &shader_module,
            input: &[
                Vertex3d::LAYOUT,
                Instance3d::LAYOUT,
            ],
            cull_mode: self.cull_mode,
            depth_write_enabled: self.depth_write_enabled,
            texture_format: TextureFormat::Bgra8UnormSrgb,
        })
    }

    fn set_bind_groups(&self, render_pass: &mut RenderPass, bind_groups: &[BindGroup]) {
        render_pass.set_bind_group(0, &bind_groups[0], &[]);
        render_pass.set_bind_group(1, &bind_groups[1], &[]);
        render_pass.set_bind_group(2, &bind_groups[2], &[]);
    }
}

struct Meshes {
    tile_quad: Mesh,
    tile_highlight: Mesh,
}

impl Meshes {
    fn create(gpu: &Gpu) -> Self {
        Self {
            tile_quad: create_tile_quad_mesh(gpu),
            tile_highlight: create_tile_highlight_mesh(gpu, 0.0025),
        }
    }
}

fn create_tile_quad_mesh(gpu: &Gpu) -> Mesh {
    const VERTICES: [Vertex3d; 4] = [
        Vertex3d::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
    ];
    const INDICES: [u16; 6] = [0, 2, 1, 3, 1, 2];

    Mesh::create(gpu, &VERTICES, &INDICES)
}

fn create_tile_highlight_mesh(gpu: &Gpu, width: f32) -> Mesh {
    let vertices = [
        Vertex3d::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(-0.5 - width, -0.5 - width, 0.5), Vec2::new(0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(-0.5 - width, 0.5 + width, 0.5), Vec2::new(1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5 + width, -0.5 - width, 0.5), Vec2::new(0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3d::new(Vec3::new(0.5 + width, 0.5 + width, 0.5), Vec2::new(1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
    ];
    const INDICES: [u16; 24] = [
        0, 1, 5, 5, 4, 0,
        1, 3, 7, 7, 5, 1,
        3, 2, 6, 6, 7, 3,
        2, 0, 4, 4, 6, 2,
    ];

    Mesh::create(gpu, &vertices, &INDICES)
}

struct CreatedTextures {
    bind_group: UniqueBindGroup,
    positions: Vec<(vec2f, f32)>,
}

fn create_textures(gpu: &Gpu) -> CreatedTextures {
    // TODO: fix
    let entries = ["stone", "dirt", "grass", "grass_side"].into_iter()
        .map(|name| image::open(format!("assets/texture/{name}.png")).unwrap())
        .map(|image| AtlasEntry { texture: image, mip: Default::default() })
        .collect::<Vec<_>>();

    let diffuse_atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size: 8192,
        mip: AtlasMipOption::NoMipWithPadding(8),
        entries: &entries,
    }).unwrap();
    let texture = diffuse_atlas.textures.into_iter().next().unwrap();
    let image = texture.mip_maps.into_iter().next().unwrap();

    let atlas_texture = Texture::from_image(gpu, DynamicImage::ImageRgba8(image));
    let positions = diffuse_atlas.texcoords.into_iter()
        .map(|x| (
            Vec2::new(x.min_x as f32 / x.size as f32, x.min_y as f32 / x.size as f32),
            (x.max_x - x.min_x) as f32 / x.size as f32
        ))
        .collect();

    let sampler = gpu.device
        .create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

    let bind_group = BindGroup::build()
        .with_item(BindGroupItem {
            layout_entry: BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            group_entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::Sampler(&sampler),
            },
        })
        .append(&atlas_texture, ShaderStages::FRAGMENT)
        .append(&atlas_texture, ShaderStages::FRAGMENT)
        .append(&atlas_texture, ShaderStages::FRAGMENT)
        .finish(&gpu);

    CreatedTextures {
        bind_group,
        positions,
    }
}