use prims::size::size2u;
use wgpu::CreateSurfaceError;

use crate::{Gpu, Surface, SurfaceTarget, Texture};

#[derive(Debug, PartialEq)]
pub struct Video {
    surface: Surface,
    depth_texture: Texture,
}

impl Video {
    pub fn new(gpu: &Gpu, target: impl Into<SurfaceTarget>, size: size2u) -> Result<Self, CreateSurfaceError> {
        let surface = Surface::new(&gpu, target, size)?;
        Ok(Self::from_surface(&gpu, surface, size))
    }

    pub fn from_surface(gpu: &Gpu, surface: Surface, size: size2u) -> Self {
        Self {
            surface,
            depth_texture: gpu.build_texture(size).depth32().create(),
        }
    }
}

impl Video {
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn size(&self) -> size2u {
        self.surface.size()
    }

    pub fn set_size(&mut self, gpu: &Gpu, size: size2u) {
        self.surface.set_size(&gpu, size);
        self.depth_texture = gpu.build_texture(size).depth32().create();
    }
}
