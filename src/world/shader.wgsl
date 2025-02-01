struct Vertex {
    @location(0) pos: vec3f,
    @location(1) tex_pos: vec2f,
    @location(2) tex_index: u32,
}

@vertex
fn vs(in: Vertex) -> Fragment {
    var out: Fragment;
    out.clip_pos = vec4f(in.pos, 1.0);
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