use crate::camera::Camera;
use crate::gpu::handle::Handle;
use crate::gpu::mem::bind_group::{BindEntry, BindGroup, BindGroupSet};
use crate::gpu::mem::model::{InstanceGroup, Mesh};
use crate::gpu::TextureFormat;
use crate::renderer_3d::lighting::Lighting;
use crate::renderer_3d::vertex::Vertex3D;
use image::DynamicImage;
use image_atlas::{AtlasDescriptor, AtlasEntry, Texcoord};
use math::projection::perspective::Perspective;
use math::vector::{vec2f, Vec2, Vec3};
use wgpu::{include_wgsl, AddressMode, BindGroupEntry, BindGroupLayoutEntry, BindingResource, BindingType, FilterMode, RenderPass, RenderPipeline, SamplerBindingType, SamplerDescriptor, ShaderStages};
use crate::gpu::mem::buffer::UnaryBuffer;

pub struct Pipeline3D {
    render_pipeline: RenderPipeline,
    bind_group_set: BindGroupSet,
    meshes: Meshes,
}

impl Pipeline3D {
    pub fn create(handle: &Handle, camera: &UnaryBuffer<Camera<Perspective>>, lighting: &Lighting, format: TextureFormat) -> Self {
        let mut bind_group_set = BindGroupSet::build(handle)
            .build_group(|builder| builder.with_entries(camera))
            .build_group(|builder| builder
                .with_entries(&lighting.ambient_light_set)
                .with_entries(&lighting.directional_light_set)
                .with_entries(&lighting.point_light_set)
            )
            .finish();
        bind_group_set.push(build_textures(handle));
        let render_pipeline = handle.create_render_pipeline(
            &bind_group_set,
            include_wgsl!("shader.wgsl"),
            &super::vertex::buffer_layouts(),
            format
        );
        let meshes = Meshes::create(handle);

        Self {
            render_pipeline,
            bind_group_set,
            meshes,
        }
    }

    pub fn render(
        &self,
        render_pass: &mut RenderPass,
        tile_instance_groups: impl Iterator<Item = &InstanceGroup>,
        tile_highlight_instance_groups: impl Iterator<Item = &InstanceGroup>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
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
            tile_highlight: create_tile_highlight_mesh(handle, 0.01),
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
        /*
        0, 2, 1, 3, 1, 2,
        4, 5, 6, 7, 6, 5,

         */
    ];

    Mesh::create(handle, vertices, INDICES)
}

fn build_textures(handle: &Handle) -> BindGroup {
    // TODO: fix
    let entries = ["stone", "dirt", "grass"].into_iter()
        .map(|name| image::open(format!("assets/texture/{name}.png")).unwrap())
        .map(|image| AtlasEntry { texture: image, mip: Default::default() })
        .collect::<Vec<_>>();

    let diffuse_atlas = image_atlas::create_atlas(&AtlasDescriptor {
        max_page_count: 1,
        size: 256,
        mip: Default::default(),
        entries: &entries,
    })
        .unwrap();
    let texture = diffuse_atlas.textures.into_iter().next().unwrap();
    let image = texture.mip_maps.into_iter().next().unwrap();

    let atlas_texture = handle.create_texture_from_image(DynamicImage::ImageRgba8(image));
    let mut positions: Vec<vec2f> = vec![];
    for Texcoord { min_x, min_y, max_x, max_y, size, .. } in diffuse_atlas.texcoords {
        let min = Vec2::new(min_x as f32, min_y as f32);
        let max = Vec2::new(max_x as f32, max_y as f32);
        let size = Vec2::splat(size as f32);

        positions.push(Vec2::new(min.x, min.y) / size);
        positions.push(Vec2::new(max.x, min.y) / size);
        positions.push(Vec2::new(min.x, max.y) / size);
        positions.push(Vec2::new(max.x, max.y) / size);
    }
    let positions_uniform = handle.create_array_buffer(positions, ShaderStages::VERTEX);
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

    let builder = BindGroup::build()
        .with_entries(&positions_uniform)
        .with_entries(&atlas_texture);
    let binding = builder.len() as u32;

    builder.with_entry(BindEntry {
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
    }).finish(&handle)
}