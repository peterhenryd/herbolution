@group(0) @binding(0) var<uniform> view_projection: mat4x4<f32>;
@group(1) @binding(0) var texture_array: texture_2d_array<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

struct Vertex {
    @location(0) pos: vec3f,
    @location(1) tex_pos: vec2f,
}

struct Quad {
    @location(3) model_0: vec4f,
    @location(4) model_1: vec4f,
    @location(5) model_2: vec4f,
    @location(6) model_3: vec4f,
    @location(7) tex_index: u32,
}

@vertex
fn vs(vertex: Vertex, quad: Quad) -> Fragment {
    var model: mat4x4<f32> = mat4x4<f32>(
        quad.model_0,
        quad.model_1,
        quad.model_2,
        quad.model_3
    );

    var out: Fragment;
    out.clip_pos = view_projection * model * vec4f(vertex.pos, 1.0);
    out.tex_pos = vertex.tex_pos;
    out.tex_index = quad.tex_index;

    return out;
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
    @location(0) tex_pos: vec2f,
    @location(1) tex_index: u32,
}

@fragment
fn fs(in: Fragment) -> @location(0) vec4f {
    return textureSample(texture_array, texture_sampler, in.tex_pos, in.tex_index);
}