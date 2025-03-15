@group(0) @binding(0) var<uniform> camera: Camera;
struct Camera {
    view_projection_matrix: mat4x4<f32>,
    world_position: vec3f,
}

struct Vertex {
    @location(0) position: vec2f,
}

struct Fragment {
    @builtin(position) clip_position: vec4f,
}

@vertex
fn vs(vert: Vertex) -> Fragment {
    var frag: Fragment;
    frag.clip_position = camera.view_projection_matrix * vec4f(vert.position, 0.0, 1.0);
    return frag;
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0);
}