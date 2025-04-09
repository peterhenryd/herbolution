@group(0) @binding(0) var<uniform> camera: Camera;
struct Camera {
    view_projection_matrix: mat4x4<f32>,
    world_pos: vec3f,
}

struct Vertex {
    @location(0) pos: vec2f,
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
}

@vertex
fn vs(vert: Vertex) -> Fragment {
    var frag: Fragment;
    frag.clip_pos = camera.view_projection_matrix * vec4f(vert.pos, 0.0, 1.0);
    return frag;
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0);
}