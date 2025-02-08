@group(0) @binding(0) var<uniform> view_proj: mat4x4<f32>;
@group(1) @binding(0) var diffuse_atlas: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var<storage> diffuse_texture_positions: array<vec2f, 128>;

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3f,
}

struct Instance {
    @location(2) model_0: vec4f,
    @location(3) model_1: vec4f,
    @location(4) model_2: vec4f,
    @location(5) model_3: vec4f,
    @location(6) texture_index: u32,
}

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let model = mat4x4<f32>(
        inst.model_0,
        inst.model_1,
        inst.model_2,
        inst.model_3,
    );

    var frag: Fragment;

    let world_position = vec4f(vert.position, 1.0);
    frag.clip_position = view_proj * model * world_position;
    frag.texture_position = diffuse_texture_positions[inst.texture_index * 4 + vert.index];

    return frag;
}

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) texture_position: vec2f,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    return textureSample(diffuse_atlas, texture_sampler, frag.texture_position);
}