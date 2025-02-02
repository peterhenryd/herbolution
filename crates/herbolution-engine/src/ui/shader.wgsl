@group(0) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
    @location(0) pos: vec2f,
    @location(1) tex_pos: vec2f,
    @location(2) tex_index: u32,
}

@vertex
fn vs(in: Vertex) -> Fragment {
    var out: Fragment;
    out.clip_pos = view_projection * vec4f(in.pos, 0.0, 1.0);
    out.tex_pos = in.tex_pos;
    out.tex_index = in.tex_index;

    return out;
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
    @location(0) tex_pos: vec2f,
    @location(1) tex_index: u32,
}

@fragment
fn fs(in: Fragment) -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 1.0, 1.0);
}