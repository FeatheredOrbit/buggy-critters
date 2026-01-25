#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip;

struct Entity {
    transform: mat4x4<f32>,

    info1: vec4<f32>
};

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,

    @location(2) velocity: f32,

    @location(3) @interpolate(flat) head_atlas_index: u32,
    @location(4) @interpolate(flat) body_atlas_index: u32,
    @location(5) @interpolate(flat) legs_atlas_index: u32
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> entities: array<Entity>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1) 
var atlas: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var atlas_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) 
var noise_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(4) 
var noise_sampler: sampler;

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let entity = entities[in.instance_index];

    out.clip_position = mesh2d_position_local_to_clip(entity.transform, vec4<f32>(in.position.xy, 0.0, 1.0));

    out.color = vec4<f32>(0.0, 1.0, 0.0, 1.0);

    out.uv = in.uv;

    out.velocity = entity.info1.x;

    out.head_atlas_index = u32(entity.info1.y);
    out.body_atlas_index = u32(entity.info1.z);
    out.legs_atlas_index = u32(entity.info1.w);
    
    return out;
}

fn mirror(uv: vec2<f32>) -> vec2<f32> {
    return abs(fract(uv) * 2 - 1);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let atlas_width = 360.0;
    let atlas_height = 140.0;

    let sprite_size = vec2<f32>(120.0, 140.0);

    let uv_size = sprite_size / vec2<f32>(atlas_width, atlas_height);

    let head_column = in.head_atlas_index % 3;
    let head_row = in.head_atlas_index / 3;

    let head_uv_offset = vec2<f32>(f32(head_column), f32(head_row)) * uv_size;

    let body_column = in.body_atlas_index % 3;
    let body_row = in.body_atlas_index / 3;

    let body_uv_offset = vec2<f32>(f32(body_column), f32(body_row)) * uv_size;

    let legs_column = in.legs_atlas_index % 3;
    let legs_row = in.legs_atlas_index / 3;

    let legs_uv_offset = vec2<f32>(f32(legs_column), f32(legs_row)) * uv_size;

    let final_head_uv = head_uv_offset + in.uv * uv_size;
    let final_body_uv = body_uv_offset + in.uv * uv_size;
    let final_legs_uv = legs_uv_offset + in.uv * uv_size;

    let head = textureSample(atlas, atlas_sampler, final_head_uv);
    let body = textureSample(atlas, atlas_sampler, final_body_uv);
    let legs = textureSample(atlas, atlas_sampler, final_legs_uv);

    let base = head + body + legs;

    let noise_uv = in.uv + vec2<f32>(-in.velocity, 0) / 150.0;

    let mirrored_uv = mirror(noise_uv);

    let noise_texture = textureSample(noise_tex, noise_sampler, mirrored_uv);

    let result = (base * (noise_texture * 0.2) * base.a * 3) * in.color;

    return result;
}