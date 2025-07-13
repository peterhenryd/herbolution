// Bind groups

struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3f,
}

@group(0) @binding(0) var<uniform> camera: Camera;

struct World {
    ambient_light: vec3f,
    light_dir: vec3f,
    fog_color: vec3f,
    fog: vec2f,
}

@group(1) @binding(0) var<uniform> world: World;

// Vertex shader

struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) uv: vec2f,
}

struct Instance {
    @location(3) model_0: vec4f,
    @location(4) model_1: vec4f,
    @location(5) model_2: vec4f,
    @location(6) model_3: vec4f,
    @location(7) color: vec4f,
    @location(8) light: u32,
    @location(9) ao: vec4f,
}

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let model = mat4x4(
        inst.model_0,
        inst.model_1,
        inst.model_2,
        inst.model_3,
    );
    let world_position = model * vec4(vert.position, 1.0);

    var frag: Fragment;
    frag.clip_position = camera.view_proj * world_position;
    frag.world_position = world_position.xyz;
    frag.normal = (model * vec4(vert.normal, 0.0)).xyz;
    frag.color = inst.color;
    frag.light = inst.light;
    frag.ao = inst.ao[vert.index];

    return frag;
}

// Fragment shader

struct Fragment {
    @builtin(position) clip_position: vec4f,
    @location(0) world_position: vec3f,
    @location(1) normal: vec3f,
    @location(2) color: vec4f,
    @location(3) light: u32,
    @location(4) ao: f32,
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    if (frag.light != 0) {
        return frag.color;
    }

    let albedo_color = frag.color;
    let diffuse = max(dot(frag.normal, world.light_dir), 0.0);
    let lit_color = (diffuse + world.ambient_light) * albedo_color.xyz * frag.ao;

    let fog_amount = smoothstep(world.fog.x - world.fog.y, world.fog.x, length(frag.world_position - camera.position));
    let color_with_fog = mix(lit_color.xyz, world.fog_color, fog_amount);

    return vec4(color_with_fog.xyz, albedo_color.a);
}
