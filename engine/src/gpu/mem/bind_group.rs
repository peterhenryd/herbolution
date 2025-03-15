use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, RenderPass};
use crate::gpu::handle::Handle;

pub struct BindGroupSet(Vec<BindGroup>);

impl BindGroupSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn build(handle: &Handle) -> BindGroupSetBuilder {
        BindGroupSetBuilder {
            handle,
            groups: Vec::new(),
        }
    }

    pub fn push(&mut self, bind_group: BindGroup) {
        self.0.push(bind_group);
    }

    pub fn with(mut self, bind_group: BindGroup) -> Self {
        self.push(bind_group);
        self
    }

    pub fn layouts(&self) -> impl Iterator<Item = &BindGroupLayout> {
        self.0.iter().map(|x| &x.layout)
    }

    pub fn bind_consecutive(&self, render_pass: &mut RenderPass, offset: u32) {
        for (i, bind_group) in self.0.iter().enumerate() {
            render_pass.set_bind_group(i as u32 + offset, &bind_group.inner, &[]);
        }
    }
}

pub struct BindGroup {
    inner: wgpu::BindGroup,
    layout: BindGroupLayout,
}

impl BindGroup {
    pub fn build<'a>() -> BindGroupBuilder<'a> {
        BindGroupBuilder { entries: Vec::new() }
    }
}

pub struct BindEntry<'a> {
    pub layout_entry: BindGroupLayoutEntry,
    pub group_entry: BindGroupEntry<'a>,
}

pub trait AddBindEntries {
    fn add_entries<'a>(&'a self, entries: &mut Vec<BindEntry<'a>>);
}

pub struct BindGroupSetBuilder<'h, 'a> {
    handle: &'h Handle,
    groups: Vec<BindGroupBuilder<'a>>,
}

impl<'h, 'a> BindGroupSetBuilder<'h, 'a> {
    pub fn with_group(mut self, builder: BindGroupBuilder<'a>) -> Self {
        self.groups.push(builder);
        self
    }

    pub fn build_group(self, f: impl FnOnce(BindGroupBuilder<'a>) -> BindGroupBuilder<'a>) -> Self {
        self.with_group(f(BindGroupBuilder { entries: Vec::new() }))
    }

    pub fn finish(self) -> BindGroupSet {
        BindGroupSet(self.groups
            .into_iter()
            .map(|builder| builder.finish(self.handle))
            .collect())
    }
}

pub struct BindGroupBuilder<'a> {
    entries: Vec<BindEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn add_entry(&mut self, entry: BindEntry<'a>) {
        self.entries.push(entry);
    }

    pub fn with_entry(mut self, entry: BindEntry<'a>) -> BindGroupBuilder<'a> {
        self.entries.push(entry);
        self
    }

    pub fn with_entries(mut self, provider: &'a impl AddBindEntries) -> BindGroupBuilder<'a> {
        provider.add_entries(&mut self.entries);
        self
    }

    pub fn finish(self, handle: &Handle) -> BindGroup {
        let mut layout_entries = Vec::with_capacity(self.entries.len());
        let mut group_entries = Vec::with_capacity(self.entries.len());
        for entry in self.entries {
            layout_entries.push(entry.layout_entry);
            group_entries.push(entry.group_entry);
        }

        let layout = handle.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });
        let group = handle.device
            .create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &group_entries,
            });
        BindGroup { inner: group, layout }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}