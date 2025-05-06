struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3f,
}

struct World {
    ambient_light: vec3f,
    light_dir: vec3f,
    fog_color: vec3f,
    fog_density: f32,
}

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3f,
    @location(1) texture_multiplier: vec2f,
    @location(2) normal: vec3f,
}

struct Instance {
    @location(3) model_0: vec4f,
    @location(4) model_1: vec4f,
    @location(5) model_2: vec4f,
    @location(6) model_3: vec4f,
    @location(7) texture_offset: vec2f,
    @location(8) texture_size: f32,
    @location(9) color: vec4f,
    @location(10) light: u32,
}

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) texture: vec2f,
    @location(3) color: vec4f,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> world: World;
@group(2) @binding(0) var texture_sampler: sampler;
@group(2) @binding(1) var albedo_atlas: texture_2d<f32>;
@group(2) @binding(2) var specular_atlas: texture_2d<f32>;
@group(2) @binding(3) var normal_atlas: texture_2d<f32>;

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let model = mat4x4(
        inst.model_0,
        inst.model_1,
        inst.model_2,
        inst.model_3,
    );
    let position = model * vec4(vert.position, 1.0);

    var frag: Fragment;
    frag.clip_position = camera.view_proj * position;
    frag.position = position.xyz;
    frag.normal = (model * vec4(vert.normal, 0.0)).xyz;
    frag.texture = inst.texture_offset + vert.texture_multiplier * inst.texture_size;
    frag.color = inst.color;

    return frag;
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    if frag.color.a != 0.0 {
        return frag.color;
    }

    let albedo_color = textureSample(albedo_atlas, texture_sampler, frag.texture);
    //let specular_color = textureSample(specular_atlas, texture_sampler, frag.texture).xyz;
    //let normal_color = textureSample(normal_atlas, texture_sampler, frag.texture).xyz;

    let diffuse = max(dot(frag.normal, world.light_dir), 0.0);
    let lit_color = (diffuse + world.ambient_light) * albedo_color.xyz;

    let fog_amount = smoothstep(0.0, world.fog_density, length(frag.position - camera.position));
    let color_with_fog = mix(lit_color.xyz, world.fog_color, fog_amount);

    return vec4(color_with_fog.xyz, albedo_color.a);
}