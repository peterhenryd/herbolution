use std::collections::HashMap;
use std::hash::Hash;

use crate::handle::Handle;
use crate::pipeline::PipelineOptions;
use crate::texture::SampleCount;
use crate::BindGroup;
use wgpu::{RenderPass, RenderPipeline};

#[derive(Debug)]
pub struct PipelineMap<K> {
    map: HashMap<K, RenderPipeline>,
    bind_groups: Vec<BindGroup>,
    sample_count: SampleCount,
}

impl<R: PipelineType> PipelineMap<R> {
    pub fn create<'a>(gpu: &Handle, state: &R::Options<'_>, sample_count: SampleCount) -> Self {
        let bind_groups = R::create_bind_groups(gpu, state);

        let mut map = HashMap::with_capacity(R::ENTRIES.len());
        for key in R::ENTRIES {
            let bind_group_layouts = bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| key.is_bind_group_enabled(*i))
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = key.pipeline_options(gpu, state);
            let render_pipeline = gpu.create_pipeline(&bind_group_layouts, sample_count, options);

            map.insert(*key, render_pipeline);
        }

        Self {
            map,
            bind_groups,
            sample_count,
        }
    }

    pub fn set_sample_count(&mut self, gpu: &Handle, sample_count: SampleCount, state: &R::Options<'_>) {
        if self.sample_count == sample_count {
            return;
        }

        for (key, render_pipeline) in self.map.iter_mut() {
            let bind_group_layouts = self
                .bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| key.is_bind_group_enabled(*i))
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = key.pipeline_options(gpu, state);
            *render_pipeline = gpu.create_pipeline(&bind_group_layouts, sample_count, options);
        }

        self.sample_count = sample_count;
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

pub trait PipelineType: Copy + Eq + Hash + 'static {
    type Options<'a>;

    const ENTRIES: &'static [Self];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup>;

    fn pipeline_options<'a>(&self, gpu: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a>;

    fn is_bind_group_enabled(&self, _index: usize) -> bool {
        true
    }
}
