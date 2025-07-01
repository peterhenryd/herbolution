use crate::Handle;
use std::collections::{HashMap, HashSet};
use thiserror::Error;
pub use wgpu::ShaderModule as Module;
pub use wgpu::ShaderStages as Stage;

#[derive(Debug, Default)]
pub struct ShaderSources {
    vec: Vec<String>,
    indices: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct CompiledShaders {
    modules: HashMap<String, Module>,
}

#[derive(Debug, Error)]
pub enum CompileShaderError<'a> {
    #[error("Shader import '{name}' could not be found.")]
    UnknownImport { shader: &'a str, name: &'a str },
    #[error("Failed to compile shader '{shader}' because of a cyclical import: {cycle:?}")]
    CyclicImport { shader: &'a str, cycle: Vec<&'a str> },
}

impl ShaderSources {
    pub fn insert(&mut self, name: impl AsRef<str>, source: impl AsRef<str>) {
        let (name, source) = (name.as_ref(), source.as_ref());

        if !self.indices.contains_key(name) {
            let index = self.vec.len();
            self.vec.push(source.to_string());
            self.indices.insert(name.to_string(), index);
        }
    }

    pub fn with(mut self, name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        self.insert(name, source);
        self
    }

    pub fn compile(&self, gpu: &Handle) -> Result<CompiledShaders, CompileShaderError<'_>> {
        let mut parsed_sources = Vec::with_capacity(self.vec.len());

        for (name, i) in &self.indices {
            let mut imports = Vec::new();
            let mut wgsl = String::new();

            for line in self.vec[*i].lines() {
                if let Some(import) = line.strip_prefix("@import ") {
                    imports.push(import);
                } else {
                    wgsl.push_str(line);
                    wgsl.push('\n');
                }
            }

            parsed_sources.push(ParsedSource { name, imports, wgsl });
        }

        if let Err((shader, cycle)) = find_cyclical_imports(&self.indices, &parsed_sources) {
            return Err(CompileShaderError::CyclicImport { shader, cycle });
        }

        let mut full_sources = HashMap::<_, String>::new();
        let mut sources_to_process = parsed_sources.into_iter().peekable();

        loop {
            let Some(source) = sources_to_process.peek() else {
                break;
            };

            let mut import_sources = vec![];
            for &import in &source.imports {
                if let Some(full_source) = full_sources.get(import) {
                    import_sources.push(full_source);
                } else {
                    continue;
                }
            }

            let source = sources_to_process.next().unwrap();
            let mut full_source = source.wgsl;

            for import in source.imports {
                let import_source = &full_sources[import];

                full_source.push('\n');
                full_source.push_str(import_source);
            }

            full_sources.insert(source.name, full_source);
        }

        let modules = full_sources
            .into_iter()
            .map(|(name, source)| {
                (
                    name.to_owned(),
                    gpu.device()
                        .create_shader_module(wgpu::ShaderModuleDescriptor {
                            label: Some(name),
                            source: wgpu::ShaderSource::Wgsl(source.into()),
                        }),
                )
            })
            .collect();

        Ok(CompiledShaders { modules })
    }
}

struct ParsedSource<'a> {
    name: &'a str,
    imports: Vec<&'a str>,
    wgsl: String,
}

impl CompiledShaders {
    pub fn get_module(&self, name: impl AsRef<str>) -> Option<&Module> {
        self.modules.get(name.as_ref())
    }
}

fn find_cyclical_imports<'a>(indices: &'a HashMap<String, usize>, parsed_sources: &[ParsedSource<'a>]) -> Result<(), (&'a str, Vec<&'a str>)> {
    let mut fully_explored_nodes = HashSet::new();

    for (name, index) in indices {
        if !fully_explored_nodes.contains(index) {
            let mut path = Vec::new();

            if let Err(cycle) = visit(name, *index, indices, parsed_sources, &mut path, &mut fully_explored_nodes) {
                return Err((name, cycle));
            }
        }
    }

    Ok(())
}

fn visit<'a>(
    name: &'a str,
    index: usize,
    indices: &'a HashMap<String, usize>,
    parsed_sources: &[ParsedSource<'a>],
    path: &mut Vec<&'a str>,
    fully_explored_nodes: &mut HashSet<usize>,
) -> Result<(), Vec<&'a str>> {
    path.push(name);

    if let Some(ParsedSource { imports, .. }) = parsed_sources.get(index) {
        for &dependency in imports {
            if let Some(start_index) = path.iter().position(|&p| p == dependency) {
                let mut cycle_report: Vec<_> = path[start_index..].to_vec();

                cycle_report.push(dependency);
                return Err(cycle_report);
            }

            let index = indices.get(dependency).unwrap();
            if !fully_explored_nodes.contains(index) {
                if let Err(cycle) = visit(dependency, *index, indices, parsed_sources, path, fully_explored_nodes) {
                    return Err(cycle);
                }
            }
        }
    }

    path.pop();
    fully_explored_nodes.insert(index);

    Ok(())
}
