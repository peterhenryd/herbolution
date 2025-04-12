use std::iter::once;
use crate::camera::Camera;
use crate::gpu::handle::Handle;
use crate::gpu::mem::bind_group::{BindEntry, BindGroup, BindGroupSet};
use crate::gpu::mem::buffer::UnaryBuffer;
use crate::gpu::mem::model::{InstanceGroup, Mesh};
use crate::gpu::TextureFormat;
use crate::renderer_3d::vertex::Vertex3D;
use image::DynamicImage;
use image_atlas::AtlasEntry;
use image_atlas::AtlasDescriptor;
use math::vector::{vec2f, Vec2, Vec3};
use wgpu::{include_wgsl, AddressMode, BindGroupEntry, BindGroupLayoutEntry, BindingResource, BindingType, Face, FilterMode, RenderPass, RenderPipeline, SamplerBindingType, SamplerDescriptor, ShaderStages};
use math::proj::Perspective;

pub struct Pipeline3D {
    // Front-face culling with depth writing
    front_pipeline: RenderPipeline,
    // Back-face culling without depth writing
    back_pipeline: RenderPipeline,
    bind_group_set: BindGroupSet,
    meshes: Meshes,
    pub texture_positions: Vec<(vec2f, f32)>,
}

impl Pipeline3D {
    pub fn create(handle: &Handle, camera: &UnaryBuffer<Camera<Perspective>>, format: TextureFormat) -> Self {
        let mut bind_group_set = BindGroupSet::build(handle)
            .build_group(|builder| builder.with_entries(camera))
            .finish();
        let (tex_bg, texture_positions) = build_textures(handle);
        bind_group_set.push(tex_bg);
        let front_pipeline = handle.create_render_pipeline(
            Face::Front,
            &bind_group_set,
            include_wgsl!("quad.wgsl"),
            &super::vertex::buffer_layouts(),
            format,
            true,
        );
        let back_pipeline = handle.create_render_pipeline(
            Face::Back,
            &bind_group_set,
            include_wgsl!("quad.wgsl"),
            &super::vertex::buffer_layouts(),
            format,
            false
        );
        let meshes = Meshes::create(handle);

        Self {
            front_pipeline,
            back_pipeline,
            bind_group_set,
            meshes,
            texture_positions
        }
    }

    pub fn render(
        &self,
        render_pass: &mut RenderPass,
        skybox: &InstanceGroup,
        tile_instance_groups: impl Iterator<Item = &InstanceGroup>,
        tile_highlight_instance_groups: impl Iterator<Item = &InstanceGroup>,
    ) {
        render_pass.set_pipeline(&self.back_pipeline);
        self.bind_group_set.bind_consecutive(render_pass, 0);
        self.meshes.tile_quad.render(render_pass, once(skybox));

        render_pass.set_pipeline(&self.front_pipeline);
        self.bind_group_set.bind_consecutive(render_pass, 0);
        self.meshes.tile_quad.render(render_pass, tile_instance_groups);
        self.meshes.tile_highlight.render(render_pass, tile_highlight_instance_groups);
    }
}

struct Meshes {
    tile_quad: Mesh,
    tile_highlight: Mesh,
}

impl Meshes {
    fn create(handle: &Handle) -> Self {
        Self {
            tile_quad: create_tile_quad_mesh(handle),
            tile_highlight: create_tile_highlight_mesh(handle, 0.0025),
        }
    }
}

fn create_tile_quad_mesh(handle: &Handle) -> Mesh {
    const VERTICES: &[Vertex3D; 4] = &[
        Vertex3D::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
    ];
    const INDICES: &[u16; 6] = &[0, 2, 1, 3, 1, 2];

    Mesh::create(handle, VERTICES, INDICES)
}

fn create_tile_highlight_mesh(handle: &Handle, width: f32) -> Mesh {
    let vertices = &[
        Vertex3D::new(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(-0.5 - width, -0.5 - width, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(-0.5 - width, 0.5 + width, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5 + width, -0.5 - width, 0.5), Vec3::new(0.0, 0.0, 1.0)),
        Vertex3D::new(Vec3::new(0.5 + width, 0.5 + width, 0.5), Vec3::new(0.0, 0.0, 1.0)),
    ];
    const INDICES: &[u16; 24] = &[
        0, 1, 5, 5, 4, 0,
        1, 3, 7, 7, 5, 1,
        3, 2, 6, 6, 7, 3,
        2, 0, 4, 4, 6, 2,
    ];

    Mesh::create(handle, vertices, INDICES)
}

fn build_textures(handle: &Handle) -> (BindGroup, Vec<(vec2f, f32)>) {
    // TODO: fix
    let entries = ["stone", "dirt", "grass", "grass_side", "px", "nx", "py", "ny", "pz", "nz"].into_iter()
        .map(|name| {
            let image = image::open(format!("assets/texture/{name}.png")).unwrap();
            if name == "py" {
                image.rotate90()
            } else if name == "ny" {
                image.rotate270()
            } else {
                image
            }
        })
        .map(|image| AtlasEntry { texture: image, mip: Default::default() })
        .collect::<Vec<_>>();

    let diffuse_atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size: 8192,
        mip: Default::default(),
        entries: &entries,
    }).unwrap();
    let texture = diffuse_atlas.textures.into_iter().next().unwrap();
    let image = texture.mip_maps.into_iter().next().unwrap();

    let atlas_texture = handle.create_texture_from_image(DynamicImage::ImageRgba8(image));
    let tex_coords = diffuse_atlas.texcoords.into_iter()
        .map(|x| (Vec2::new(x.min_x as f32 / x.size as f32, x.min_y as f32 / x.size as f32), (x.max_x - x.min_x) as f32 / x.size as f32))
        .collect();

    let sampler = handle.device
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

    let builder = BindGroup::build().with_entries(&atlas_texture);
    let binding = builder.len() as u32;

    (builder.with_entry(BindEntry {
        layout_entry: BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
        },
        group_entry: BindGroupEntry {
            binding,
            resource: BindingResource::Sampler(&sampler),
        },
    }).finish(&handle), tex_coords)
}