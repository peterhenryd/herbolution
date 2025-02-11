use crate::engine::binding::{Binding, BindingBuilder};
use crate::engine::geometry;
use crate::engine::geometry::quad::Quad;
use crate::engine::gpu::Gpu;
use crate::engine::mesh::Mesh;
use crate::engine::pipeline::Pipeline;
use crate::engine::storage::Storage;
use crate::engine::surface::Surface;
use crate::engine::texture::depth::DepthTexture;
use crate::engine::texture::Texture;
use crate::engine::uniform::{AsByteStructUniformExt, Uniform};
use crate::listener::{InputEvent, Listener};
use crate::world::camera::proj::perspective::Perspective;
use crate::world::camera::Camera;
use crate::world::chunk::material::Material;
use crate::world::lighting::Lighting;
use crate::world::transform::{Rotation, Transform};
use image_atlas::{AtlasDescriptor, AtlasEntry, AtlasEntryMipOption};
use math::vector::{vec2, vec2f, vec3};
use wgpu::{include_wgsl, AddressMode, CompareFunction, DepthBiasState, DepthStencilState, FilterMode, SamplerBindingType, SamplerDescriptor, ShaderStages, StencilState, TextureFormat};
use winit::dpi::PhysicalSize;

pub struct Renderer {
    pub(crate) gpu: Gpu,
    pub pipeline: Pipeline,
    pub camera: Uniform<Camera<Perspective>>,
    pub lighting: Lighting,
    pub(crate) quad_mesh: Mesh,
    pub(crate) depth_texture: DepthTexture,
}

impl Renderer {
    pub fn create(gpu: Gpu, surface: &Surface) -> Self {
        let size = surface.get_size();
        let proj = Perspective::from(size);
        let camera = Camera::new(Transform::new(vec3::zero(), Rotation::default()), proj)
            .into_uniform(&gpu, "camera");
        let lighting = Lighting::create(&gpu);
        let pipeline = gpu
            .build_pipeline("world", surface.get_format())
            .with_shader(include_wgsl!("shader.wgsl"))
            .build_binding("world_camera", |b| b
                .with_uniform(ShaderStages::VERTEX_FRAGMENT, &camera)
                .finish(),
            )
            .build_binding("world_texture", |b| build_textures(&gpu, b))
            .with_binding(lighting.binding.clone())
            .with_buffers(geometry::get_vertex_instance_buffer_layouts())
            .with_depth_stencil(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            })
            .finish();
        let quad_mesh = Quad::create_mesh(&gpu);
        let depth_texture = DepthTexture::create(&gpu, size);

        Self { gpu, pipeline, camera, lighting, quad_mesh, depth_texture }
    }

    pub fn camera(&self) -> &Transform {
        &self.camera.transform
    }
}

impl Listener for Renderer {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.depth_texture.resize(size);
        self.camera.edit(|c| c.proj.resize(size));
    }

    fn on_input(&mut self, _: &InputEvent) {}
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

    let diffuse_atlas_texture = Texture::from_bytes(
        gpu,
        "texture_atlas",
        image.width(),
        image.height(),
        image.as_ref(),
    );
    let mut diffuse_atlas_positions: Vec<vec2f> = vec![];
    for tex_coord in diffuse_atlas.texcoords {
        let size = vec2::new(tex_coord.size, tex_coord.size).cast::<f32>();
        diffuse_atlas_positions.push(vec2::new(tex_coord.min_x, tex_coord.min_y).cast::<f32>() / size);
        diffuse_atlas_positions.push(vec2::new(tex_coord.max_x, tex_coord.min_y).cast::<f32>() / size);
        diffuse_atlas_positions.push(vec2::new(tex_coord.min_x, tex_coord.max_y).cast::<f32>() / size);
        diffuse_atlas_positions.push(vec2::new(tex_coord.max_x, tex_coord.max_y).cast::<f32>() / size);
    }
    let diffuse_atlas_positions_uniform =
        Storage::create(gpu, "diffuse_atlas_positions", diffuse_atlas_positions);

    builder
        .with_texture(&diffuse_atlas_texture.create_view())
        .with_sampler(
            SamplerBindingType::Filtering,
            &gpu.device.create_sampler(&SamplerDescriptor {
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