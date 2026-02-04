#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip;

struct DeathExplosionShaderData {
    transform: mat4x4<f32>,
    fragment_amount: vec4<u32>
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) fragment_id: f32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> explosions: array<DeathExplosionShaderData>;

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let explosion = explosions[in.instance_index];

    out.clip_position = mesh2d_position_local_to_clip(explosion.transform, vec4<f32>(in.position.xy, 0.0, 1.0));

    out.uv = in.uv;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}