use std::collections::HashMap;
use std::hash::Hash;

use wgpu::{RenderPass, RenderPipeline};

use crate::bind_group::BindGroup;
use crate::handle::Handle;
use crate::pipeline::PipelineOptions;

#[derive(Debug)]
pub struct PipelineMap<K, const N: usize> {
    map: HashMap<K, RenderPipeline>,
    bind_groups: [BindGroup; N],
}

impl<R: Key<N>, const N: usize> PipelineMap<R, N> {
    pub fn create<'a>(handle: &Handle, state: &R::Options<'_>) -> Self {
        let bind_groups = R::create_bind_groups(handle, state);

        let mut map = HashMap::with_capacity(N);
        for entry in R::ENTRIES {
            let bind_group_included = entry.bind_groups();
            let bind_group_layouts = bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| bind_group_included[*i])
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = entry.pipeline_options(handle, state);
            let render_pipeline = handle.create_pipeline(&bind_group_layouts, options);

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

        let bind_group_enabled = render_type.bind_groups();
        for (i, bind_group) in self.bind_groups.iter().enumerate() {
            if bind_group_enabled[i] {
                render_pass.set_bind_group(i as u32, &bind_group.inner, &[]);
            }
        }
    }
}

pub trait Key<const N: usize>: Copy + Eq + Hash + 'static {
    type Options<'a>;

    const ENTRIES: &'static [Self];

    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; N];

    fn pipeline_options<'a>(&self, handle: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a>;

    fn bind_groups(&self) -> [bool; N] {
        [true; N]
    }
}
