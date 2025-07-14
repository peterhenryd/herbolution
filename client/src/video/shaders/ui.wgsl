struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3f,
}

@group(0) @binding(0) var<uniform> camera: Camera;

@group(1) @binding(0) var albedo_sampler: sampler;
@group(1) @binding(1) var albedo_texture: texture_2d<f32>;

struct Vertex {
    @location(0) position: vec2f,
    @location(1) normal: vec2f,
    @location(2) uv: vec2f,
}

struct Instance {
    @location(3) model_0: vec2f,
    @location(4) model_1: vec2f,
    @location(5) model_2: vec2f,
    @location(6) color: vec4f,
    @location(7) uv_t: vec2f,
    @location(8) uv_s: vec2f,
    @location(9) border_radius: f32,
    @location(10) scale: vec2f,
}

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) world_position: vec3f,
    @location(1) uv: vec2f,
    @location(2) color: vec4f,
    @location(3) vert_uv: vec2f,
    @location(4) border_radius: f32,
    @location(5) scale: vec2f,
}

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let model = mat3x3(
        vec3(inst.model_0, 0.0),
        vec3(inst.model_1, 0.0),
        vec3(inst.model_2, 1.0)
    );

    let position = model * vec3(vert.position, 1.0);

    var frag: Fragment;
    frag.clip_position = camera.view_proj * vec4(position, 1.0);
    frag.world_position = position;
    frag.uv = inst.uv_t + vert.uv * inst.uv_s;
    frag.color = inst.color;
    frag.vert_uv = vert.uv;
    frag.border_radius = inst.border_radius;
    frag.scale = inst.scale;

    return frag;
}

// Fragment shader

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    let p = (frag.vert_uv - 0.5) * frag.scale;
    let b = frag.scale * 0.5;

    let r = min(frag.border_radius, min(b.x, b.y));

    let d = abs(p) - b + r;
    let sd = length(max(d, vec2(0.0))) - r;

    let aa = fwidth(sd);
    let alpha_mask = 1.0 - smoothstep(-aa, aa, sd);

    if (alpha_mask <= 0.0) {
        discard;
    }

    var base_color: vec4f;
    if frag.color.a != 0.0 {
        base_color = frag.color;
    } else {
        let texture_alpha = textureSample(albedo_texture, albedo_sampler, frag.uv).a;
        base_color = vec4(frag.color.rgb, texture_alpha);
    }

    let final_color = vec4(base_color.rgb, base_color.a * alpha_mask);

    return final_color;
}
