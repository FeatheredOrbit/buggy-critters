#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip;

struct Entity {
    transform: mat4x4<f32>, 

    head_atlas_index: u32,
    body_atlas_index: u32,
    legs_atlas_index: u32
};

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,

    @location(2) @interpolate(flat) head_atlas_index: u32,
    @location(3) @interpolate(flat) body_atlas_index: u32,
    @location(4) @interpolate(flat) legs_atlas_index: u32
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> entities: array<Entity>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1) 
var atlas: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var atlas_sampler: sampler;

@vertex
fn vertex
(
    in: VertexInput
) -> VertexOutput 
{
    var out: VertexOutput;

    let entity = entities[in.instance_index];

    out.clip_position = mesh2d_position_local_to_clip(entity.transform, vec4<f32>(in.position.xy, 0.0, 1.0));

    out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    out.uv = in.uv;

    out.head_atlas_index = entity.head_atlas_index;
    out.body_atlas_index = entity.body_atlas_index;
    out.legs_atlas_index = entity.legs_atlas_index;
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}