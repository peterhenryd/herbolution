@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<storage> diffuse_texture_poss: array<vec2f>;
@group(1) @binding(1) var diffuse_atlas: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

struct Camera {
    view_proj: mat4x4<f32>,
    pos: vec3f,
}

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) pos: vec3f,
    @location(1) normal: vec3f,
}

struct Instance {
    @location(2) model_0: vec4f,
    @location(3) model_1: vec4f,
    @location(4) model_2: vec4f,
    @location(5) model_3: vec4f,
    @location(6) texture_index: u32,
    @location(7) color: vec4f,
    @location(8) is_lit: u32,
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

    let world_pos = model * vec4(vert.pos, 1.0);
    frag.clip_pos = camera.view_proj * world_pos;
    frag.texture_pos = diffuse_texture_poss[inst.texture_index * 4 + vert.index];
    frag.color = inst.color;
    frag.world_normal = (model * vec4(vert.normal, 0.0)).xyz;
    frag.world_pos = world_pos.xyz;
    frag.is_lit = inst.is_lit;

    return frag;
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
    @location(0) texture_pos: vec2f,
    @location(1) color: vec4f,
    @location(2) world_normal: vec3f,
    @location(3) world_pos: vec3f,
    @location(4) is_lit: u32,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    if frag.color.a != 0.0 {
        return frag.color;
    }

    var color = textureSample(diffuse_atlas, texture_sampler, frag.texture_pos);

    if frag.is_lit == 0 {
        return color;
    }

    let ambient = vec3(0.5, 0.5, 0.5);
    let light_dir = normalize(vec3(0.2, 1.0, -0.7));

    let view_dir = normalize(camera.pos - frag.world_pos);
    let diffuse = max(dot(frag.world_normal, light_dir), 0.0);

    color = vec4((ambient + diffuse) * color.xyz, color.a);

    return color;
}