use crate::gpu;
use crate::gpu::binding::DepthTexture;
use crate::gpu::geometry::InstanceBuffer;
use crate::gpu::{Gpu, GpuError, GpuFrame, GpuOptions, Surface};
use crate::input::{Input, InputFrame};
use crate::state2d::State2d;
use crate::state3d::State3d;
use game::chunk;
use math::size::Size2;
use math::vector::vec3i;
use std::sync::Arc;
use wgpu::{MemoryHints, PowerPreference, PresentMode, RenderPass};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Engine {
    pub(crate) window: Arc<Window>,
    pub(crate) gpu: Gpu,
    pub(crate) surface: Surface<'static>,
    pub(crate) input: Input,
    pub(crate) state2d: State2d,
    pub(crate) state3d: State3d,
    depth_texture: DepthTexture,
}

impl Engine {
    pub fn create(window: Arc<Window>) -> Result<Self, GpuError> {
        let PhysicalSize { width, height } = window.inner_size();
        let resolution = Size2::new(width, height);

        let (gpu, surface) = gpu::create(window.clone(), GpuOptions {
            resolution,
            power_preference: PowerPreference::HighPerformance,
            memory_hints: MemoryHints::MemoryUsage,
            present_mode: PresentMode::Immediate,
        })?;
        let input = Input::default();
        let state2d = State2d::create(&gpu, resolution, surface.format());
        let state3d = State3d::create(&gpu, resolution);
        let depth_texture = DepthTexture::create(&gpu, resolution);

        Ok(Self {
            window,
            gpu,
            surface,
            input,
            state2d,
            state3d,
            depth_texture,
        })
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.surface.set_size(&self.gpu, size);
        self.state2d.set_size(&self.gpu, size);
        self.state3d.set_size(size);
        self.depth_texture.set_size(&self.gpu, size);
    }

    pub fn next_frame(&mut self) -> Result<EngineFrame, GpuError> {
        Ok(EngineFrame {
            gpu: GpuFrame::create(&self.gpu, &self.surface, &self.depth_texture)?,
            input: self.input.take_frame(),
        })
    }

    pub fn update(&mut self) {
        self.state2d.update(&self.gpu);
        self.state3d.update(&self.gpu);
    }

    pub fn render<'a>(&self, render_pass: &mut RenderPass, facial_meshes: impl Iterator<Item = (vec3i, &'a InstanceBuffer)>) {
        let culled_meshes = facial_meshes
            .filter(|(pos, _)| self.state3d.frustum.contains_cube(pos.cast().unwrap(), chunk::LENGTH as f32))
            .map(|(_, mesh)| mesh)
            .collect::<Vec<_>>();

        self.state3d.render(render_pass, &culled_meshes);
    }
}

pub struct EngineFrame {
    pub gpu: GpuFrame,
    pub input: InputFrame,
}