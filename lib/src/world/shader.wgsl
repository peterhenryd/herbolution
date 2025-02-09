@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var diffuse_atlas: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var<storage> diffuse_texture_positions: array<vec2f>;
@group(2) @binding(0) var<storage> ambient_light_array: array<AmbientLight>;
@group(2) @binding(1) var<storage> directional_light_array: array<DirectionalLight>;
@group(2) @binding(2) var<storage> point_light_array: array<PointLight>;

struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3f,
}

struct AmbientLight {
    color: vec3f,
    intensity: f32,
}

struct DirectionalLight {
    color: vec3f,
    intensity: f32,
    direction: vec3f,
}

struct PointLight {
    color: vec3f,
    intensity: f32,
    position: vec3f,
    range: f32,
}

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
}

struct Instance {
    @location(2) model_0: vec4f,
    @location(3) model_1: vec4f,
    @location(4) model_2: vec4f,
    @location(5) model_3: vec4f,
    @location(6) texture_index: u32,
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

    let world_position = model * vec4f(vert.position, 1.0);
    frag.clip_position = camera.view_proj * world_position;
    frag.texture_position = diffuse_texture_positions[inst.texture_index * 4 + vert.index];
    frag.world_normal = (model * vec4f(vert.normal, 0.0)).xyz;
    frag.world_position = world_position.xyz;

    return frag;
}

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) texture_position: vec2f,
    @location(1) world_normal: vec3f,
    @location(2) world_position: vec3f,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    var color = textureSample(diffuse_atlas, texture_sampler, frag.texture_position);

    let len = arrayLength(&point_light_array);
    for (var i = 0u; i < len; i++) {
        let light = point_light_array[i];

        let ambient_color = light.color * light.intensity;
        let light_dir = normalize(light.position - frag.world_position);

        let view_dir = normalize(camera.position - frag.world_position);
        let half_dir = normalize(view_dir + light_dir);

        let specular_strength = pow(max(dot(frag.world_normal, half_dir), 0.0), 32.0);
        let specular_color = specular_strength * light.color;

        let diffuse_strength = max(dot(frag.world_normal, light_dir), 0.0);
        let diffuse_color = light.color * diffuse_strength;

        color = vec4f((ambient_color + diffuse_color + specular_color) * color.xyz, color.a);
    }

    return color;
}