const positions: array<vec2f, 4> = array(
    vec2(-0.5, -0.5),
    vec2( 0.5, -0.5),
    vec2(-0.5,  0.5),
    vec2( 0.5,  0.5),
);
const pos_indices: array<u32, 6> = array(0, 1, 2, 2, 1, 3);

struct Env {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> env: Env;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var albedo_texture: texture_2d<f32>;
@group(1) @binding(2) var mask_texture: texture_2d<f32>;

struct Instance {
    @location(0) transform: vec4f,
    @location(1) visual: vec4f,
    @location(2) mask: vec4f,
    @location(3) packed_vars: vec4f,
}

@vertex
fn vs(@builtin(vertex_index) vert_index: u32, inst: Instance) -> Fragment {
    // Get vertex position corresponding to index
    let vert_pos_index = pos_indices[vert_index];
    let vert_pos = positions[vert_pos_index];

    // Unpack instance arguments
    let inst_pos = inst.transform.xy;
    let inst_size = inst.transform.zw;

    let rotation = inst.packed_vars.x;
    let corner_radius = inst.packed_vars.y;

    let packed_flag = bitcast<u32>(inst.packed_vars.z);
    let use_albedo = (packed_flag & 1) != 0;

    // Calculate intermediaries
    let vert_uv = 0.5 + vert_pos;
    let scaled_pos = vert_pos * inst_size;
    let rotated_pos = rotation_matrix(rotation) * scaled_pos;
    let world_pos = inst_pos + rotated_pos;

    // Construct fragment arguments
    var frag: Fragment;
    frag.clip_pos = env.view_proj * vec4(world_pos, 0.0, 1.0);
    frag.color = inst.visual;
    frag.albedo_coords = inst.visual.xy + vert_uv * inst.visual.zw;
    frag.mask_coords = inst.mask.xy + vert_uv * inst.mask.zw;
    frag.scaled_pos = scaled_pos;
    frag.half_size = inst_size * 0.5;
    frag.corner_radius = corner_radius;
    frag.use_albedo = u32(use_albedo);
    frag.use_mask = u32(inst.mask.z != 0.0);

    return frag;
}

fn rotation_matrix(rotation: f32) -> mat2x2<f32> {
    let s = sin(rotation);
    let c = cos(rotation);

    return mat2x2(vec2(c, s), vec2(-s, c));
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
    @location(0) color: vec4f,
    @location(1) albedo_coords: vec2f,
    @location(2) mask_coords: vec2f,
    @location(3) scaled_pos: vec2f,
    @location(4) half_size: vec2f,
    @location(5) corner_radius: f32,
    @location(6) use_albedo: u32,
    @location(7) use_mask: u32,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    let albedo_color = textureSample(albedo_texture, texture_sampler, frag.albedo_coords);
    let base_color = select(frag.color, albedo_color, frag.use_albedo != 0);

    let mask_alpha = textureSample(mask_texture, texture_sampler, frag.mask_coords).r;
    let mask_alpha_factor = select(1.0, mask_alpha, frag.use_mask != 0);

    let corner_sd = rect_signed_distance(frag.scaled_pos, frag.half_size, frag.corner_radius);
    let corner_aa = fwidth(corner_sd);
    let corner_alpha_factor = 1.0 - smoothstep(-corner_aa, corner_aa, corner_sd);

    let final_alpha = base_color.a * mask_alpha_factor * corner_alpha_factor;
    let final_color = vec4(base_color.rgb, final_alpha);

    return final_color;
}

fn rect_signed_distance(pos: vec2f, size: vec2f, radius: f32) -> f32 {
    let r = min(radius, min(size.x, size.y));
    let d = abs(pos) - size + r;

    return length(max(d, vec2(0.0))) - r;
}