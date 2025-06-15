// Bind groups

struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3f,
    position_int: vec3i,
    position_frac: vec3f,
}

@group(0) @binding(0) var<uniform> camera: Camera;

struct World {
    ambient_light: vec3f,
    light_dir: vec3f,
    fog_color: vec3f,
    fog_distance: f32,
}

@group(1) @binding(0) var<uniform> world: World;

@group(2) @binding(0) var texture_sampler: sampler;
@group(2) @binding(1) var albedo_texture: texture_2d<f32>;
@group(2) @binding(2) var normal_texture: texture_2d<f32>;
@group(2) @binding(3) var specular_texture: texture_2d<f32>;

// Vertex shader

struct Vertex {
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) uv: vec2f,
}

struct Instance {
    @location(3) model_0: vec4f,
    @location(4) model_1: vec4f,
    @location(5) model_2: vec4f,
    @location(6) position_int: vec3i,
    @location(7) position_frac: vec3f,
    @location(8) color: vec4f,
    @location(9) uv_t: vec2f,
    @location(10) uv_s: vec2f,
}

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let world_position = vec3f(inst.position_int) + inst.position_frac;

    let relative_position_int = inst.position_int - camera.position_int;
    let relative_positive_frac = inst.position_frac - camera.position_frac;
    let relative_position = vec3f(relative_position_int) + relative_positive_frac;
    let model = mat4x4<f32>(
        inst.model_0,
        inst.model_1,
        inst.model_2,
        vec4(relative_position, 1.0)
    );
    let position = model * vec4(vert.position, 1.0);

    var frag: Fragment;
    frag.clip_position = camera.view_proj * position;
    frag.world_position = world_position;
    frag.normal = (model * vec4(vert.normal, 0.0)).xyz;
    frag.uv = inst.uv_t + vert.uv * inst.uv_s;
    frag.color = inst.color;

    return frag;
}

// Fragment shader

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) world_position: vec3f,
    @location(1) normal: vec3f,
    @location(2) uv: vec2f,
    @location(3) color: vec4f,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    if frag.color.a != 0.0 {
        return frag.color;
    }

    let albedo_color = textureSample(albedo_texture, texture_sampler, frag.uv);

    let diffuse = max(dot(frag.normal, world.light_dir), 0.0);
    let lit_color = (diffuse + world.ambient_light) * albedo_color.xyz;

    let fog_amount = smoothstep(world.fog_distance - 10.0, world.fog_distance, length(frag.world_position - camera.position));
    let color_with_fog = mix(lit_color.xyz, world.fog_color, fog_amount);

    return vec4(color_with_fog.xyz, albedo_color.a);
}
