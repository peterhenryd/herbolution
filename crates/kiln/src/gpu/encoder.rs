use smallvec::SmallVec;

use crate::{Gpu, RenderPass, Texture};

pub struct Encoder<'gpu> {
    gpu: &'gpu Gpu,
    inner: wgpu::CommandEncoder,
}

impl<'gpu> Encoder<'gpu> {
    pub fn build_render_pass<'encoder>(&'encoder mut self) -> RenderPassBuilder<'gpu, 'encoder, '_> {
        RenderPassBuilder {
            gpu: self.gpu,
            command_encoder: &mut self.inner,
            label: None,
            color_attachments: SmallVec::new(),
            depth_stencil_attachment: None,
        }
    }
}

impl Gpu {
    pub fn create_encoder(&self) -> Encoder {
        Encoder {
            gpu: self,
            inner: self
                .device
                .create_command_encoder(&Default::default()),
        }
    }

    pub fn create_labeled_encoder(&self, label: &str) -> Encoder {
        Encoder {
            gpu: self,
            inner: self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) }),
        }
    }
}

pub struct RenderPassBuilder<'gpu, 'encoder, 'a> {
    gpu: &'gpu Gpu,
    command_encoder: &'encoder mut wgpu::CommandEncoder,
    label: Option<&'a str>,
    color_attachments: SmallVec<Option<wgpu::RenderPassColorAttachment<'a>>, 4>,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
}

impl<'a> RenderPassBuilder<'_, '_, 'a> {
    pub fn add_color_attachment(mut self, output_texture: &'a Texture) -> Self {
        self.color_attachments
            .push(Some(wgpu::RenderPassColorAttachment {
                view: &output_texture.view,
                depth_slice: None,
                resolve_target: None,
                ops: Default::default(),
            }));

        self
    }

    pub fn depth_stencil(mut self, texture: &'a Texture, use_default_depth_ops: bool) -> Self {
        if self.depth_stencil_attachment.is_some() {
            return self;
        }

        self.depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: &texture.view,
            depth_ops: use_default_depth_ops.then_some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });

        self
    }

    pub fn clear_depth(mut self, value: f32) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.depth_ops {
            ops.load = wgpu::LoadOp::Clear(value);
        } else {
            attachment.depth_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(value),
                store: wgpu::StoreOp::Store,
            });
        }

        self
    }

    pub fn load_depth(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.depth_ops {
            ops.load = wgpu::LoadOp::Load;
        } else {
            attachment.depth_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::default(),
            });
        }

        self
    }

    pub fn store_depth(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.depth_ops {
            ops.store = wgpu::StoreOp::Store;
        } else {
            attachment.depth_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::default(),
                store: wgpu::StoreOp::Store,
            });
        }

        self
    }

    pub fn discard_depth(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.depth_ops {
            ops.store = wgpu::StoreOp::Discard;
        } else {
            attachment.depth_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::default(),
                store: wgpu::StoreOp::Discard,
            });
        }

        self
    }

    pub fn clear_stencil(mut self, value: u32) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.stencil_ops {
            ops.load = wgpu::LoadOp::Clear(value);
        } else {
            attachment.stencil_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(value),
                store: wgpu::StoreOp::Store,
            });
        }

        self
    }

    pub fn load_stencil(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.stencil_ops {
            ops.load = wgpu::LoadOp::Load;
        } else {
            attachment.stencil_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::default(),
            });
        }

        self
    }

    pub fn store_stencil(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.stencil_ops {
            ops.store = wgpu::StoreOp::Store;
        } else {
            attachment.stencil_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::default(),
                store: wgpu::StoreOp::Store,
            });
        }

        self
    }

    pub fn discard_stencil(mut self) -> Self {
        let Some(attachment) = &mut self.depth_stencil_attachment else {
            return self;
        };

        if let Some(ops) = &mut attachment.stencil_ops {
            ops.store = wgpu::StoreOp::Discard;
        } else {
            attachment.stencil_ops = Some(wgpu::Operations {
                load: wgpu::LoadOp::default(),
                store: wgpu::StoreOp::Discard,
            });
        }

        self
    }
}

impl<'gpu, 'encoder> RenderPassBuilder<'gpu, 'encoder, '_> {
    pub fn begin(self) -> RenderPass<'gpu, 'encoder> {
        RenderPass {
            gpu: self.gpu,
            inner: self
                .command_encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &self.color_attachments,
                    depth_stencil_attachment: self.depth_stencil_attachment,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }),
        }
    }
}
