const pi: f32 = 3.14159265359;

struct Env {
    view_proj: mat4x4<f32>,
    camera_pos: vec3f,
}

struct Lights {
    len: u32,
    buffer: array<Light>,
}

struct Light {
    pos: vec4f,
    color: vec4f,
    dir: vec4f,
    packed_vars: vec4f,
}

@group(0) @binding(0) var<uniform> env: Env;
@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var albedo_texture: texture_2d<f32>;
@group(1) @binding(2) var emissive_texture: texture_2d<f32>;
@group(1) @binding(3) var height_texture: texture_2d<f32>;
@group(1) @binding(4) var normal_texture: texture_2d<f32>;
@group(1) @binding(5) var orm_texture: texture_2d<f32>;
@group(2) @binding(0) var<storage, read> lights: Lights;


struct Vertex {
    @builtin(vertex_index) index: u32,
    @location(0) pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tangent: vec3f,
}

struct Instance {
    @location(4) model_0: vec4f,
    @location(5) model_1: vec4f,
    @location(6) model_2: vec4f,
    @location(7) model_3: vec4f,
    @location(8) uv: vec4f,
    @location(9) color: vec4f,
    @location(10) ao: vec4f,
}

struct Fragment {
    @builtin(position) clip_pos: vec4f,
    @location(0) world_pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) world_normal: vec3f,
    @location(3) world_tangent: vec3f,
    @location(4) color: vec4f,
    @location(5) vert_ao: f32,
}

@vertex
fn vs(vert: Vertex, inst: Instance) -> Fragment {
    let model = mat4x4(
        inst.model_0,
        inst.model_1,
        inst.model_2,
        inst.model_3,
    );

    let world_pos = model * vec4(vert.pos, 1.0);
    var clip_pos = env.view_proj * world_pos;
    clip_pos += env.jitter * frag.clip_pos.w;

    var frag: Fragment;
    frag.clip_pos = clip_pos;
    frag.world_pos = world_pos.xyz;
    frag.uv = inst.uv.xy + vert.uv * inst.uv.zw;
    frag.world_normal = (model * vec4(vert.normal, 0.0)).xyz;
    frag.world_tangent = normalize((model * vec4(vert.tangent, 0.0)).xyz);
    frag.color = inst.color;
    frag.vert_ao = inst.ao[vert.index % 4];

    return frag;
}

fn distributionGGX(n: vec3f, h: vec3f, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(n, h), 0.0);
    let NdotH2 = NdotH * NdotH;

    let num = a2;
    var den = (NdotH2 * (a2 - 1.0) + 1.0);
    den = pi * den * den;

    return num / den;
}

fn geometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    let num = NdotV;
    let den = NdotV * (1.0 - k) + k;
    return num / den;
}

fn geometrySmith(n: vec3f, v: vec3f, l: vec3f, roughness: f32) -> f32 {
    let NdotV = max(dot(n, v), 0.0);
    let NdotL = max(dot(n, l), 0.0);
    let ggx2 = geometrySchlickGGX(NdotV, roughness);
    let ggx1 = geometrySchlickGGX(NdotL, roughness);
    return ggx1 * ggx2;
}

fn fresnelSchlick(cosTheta: f32, F0: vec3f) -> vec3f {
    return F0 + (vec3(1.0) - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    let albedo = textureSample(albedo_texture, texture_sampler, frag.uv).rgb;
    let base = select(albedo, frag.color.rgb, frag.color.a != 0.0);

    let orm = textureSample(orm_texture, texture_sampler, frag.uv);
    let ao = orm.r;
    let roughness = orm.g;
    let metallic = orm.b;
    let emissive = textureSample(emissive_texture, texture_sampler, frag.uv).rgb;

    let v = normalize(env.camera_pos - frag.world_pos);

    let t = normalize(frag.world_tangent);
    let b = normalize(cross(frag.world_normal, t));
    let tbn = mat3x3(t, b, frag.world_normal);

    var n = textureSample(normal_texture, texture_sampler, frag.uv).rgb;
    n = n * 2.0 - 1.0;
    n = normalize(tbn * n);

    let F0 = mix(vec3(0.04), base, metallic);

    var Lo = vec3(0.0);
    for (var i = 0u; i < lights.len; i = i + 1) {
        let light = lights.buffer[i];

        var l: vec3f;
        var attenuation = 1.0;

        let is_point = bitcast<u32>(light.packed_vars.y);
        if (is_point != 0) {
            let light_vec = light.pos.xyz - frag.world_pos;
            let distance = length(light_vec);
            attenuation = 1.0 / (distance * distance + 0.0001);
            l = normalize(light_vec);
        } else {
            l = normalize(light.dir.xyz);
        }

        let h = normalize(v + l);
        let radiance = light.color.rgb * light.packed_vars.x * attenuation;

        let d = distributionGGX(n, h, roughness);
        let g = geometrySmith(n, v, l, roughness);
        let f = fresnelSchlick(max(dot(h, v), 0.0), F0);

        let NdotL = max(dot(n, l), 0.0);
        let kD = (vec3(1.0) - f) * (1.0 - metallic);

        let numerator = d * g * f;
        let denominator = 4.0 * max(dot(n, v), 0.0) * NdotL + 0.001;
        let specular = numerator / denominator;

        Lo += (kD * base / pi + specular) * radiance * NdotL;
    }

    let ambient = vec3(0.03) * base * ao;
    let color = ambient + Lo + emissive;
    let final_color = pow(color, vec3(1.0 / 2.2)) * frag.vert_ao;

    return vec4(final_color, 1.0);
}

