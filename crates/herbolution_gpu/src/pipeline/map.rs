use std::collections::HashMap;
use std::hash::Hash;

use wgpu::{RenderPass, RenderPipeline};

use crate::bind_group::BindGroup;
use crate::handle::Handle;
use crate::pipeline::PipelineOptions;

#[derive(Debug)]
pub struct PipelineMap<K> {
    map: HashMap<K, RenderPipeline>,
    bind_groups: Vec<BindGroup>,
}

impl<R: Key> PipelineMap<R> {
    pub fn create<'a>(gpu: &Handle, state: &R::Options<'_>) -> Self {
        let bind_groups = R::create_bind_groups(gpu, state);

        let mut map = HashMap::with_capacity(R::ENTRIES.len());
        for entry in R::ENTRIES {
            let bind_group_layouts = bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| entry.is_bind_group_enabled(*i))
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = entry.pipeline_options(gpu, state);
            let render_pipeline = gpu.create_pipeline(&bind_group_layouts, options);

            map.insert(*entry, render_pipeline);
        }

        Self { map, bind_groups }
    }

    pub fn load_by_type(&self, render_type: R, render_pass: &mut RenderPass<'_>) {
        let render_pipeline = self
            .map
            .get(&render_type)
            .expect("Unknown render type");
        render_pass.set_pipeline(render_pipeline);

        for (i, bind_group) in self.bind_groups.iter().enumerate() {
            if render_type.is_bind_group_enabled(i) {
                render_pass.set_bind_group(i as u32, &bind_group.inner, &[]);
            }
        }
    }
}

pub trait Key: Copy + Eq + Hash + 'static {
    type Options<'a>;

    const ENTRIES: &'static [Self];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup>;

    fn pipeline_options<'a>(&self, gpu: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a>;

    fn is_bind_group_enabled(&self, _index: usize) -> bool {
        true
    }
}
