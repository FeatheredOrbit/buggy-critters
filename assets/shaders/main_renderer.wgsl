#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip;

const BUG_ENTITY: u32 = 0;
const FRUIT_ENTITY: u32 = 1;

struct ShaderData {
    // Transform matrix, format is the same for anything.
    transform: mat4x4<f32>,

    // What does this data represent?
    // Values [0, 1] represent [bug, fruit] respectively.
    data_type: u32,

    // First packet of information.
    // For entities, slots [0, 1, 2, 3] are [velocity, head_sprite_index, body_sprite_index, legs_sprite_index] respectively.
    // For fruits, slots [0, 1, 2, 3] are [speed, nothing, nothing, nothing] respectively.
    info1: vec4<f32>,

    // Second packet of information.
    // For entities, slots [0, 1, 2, 3] are [head_color_r, head_color_g, head_color_b, head_color_a] respectively.
    // For fruits, slots [0, 1, 2, 3] are [color_r, color_g, color_b, color_a] respectively.
    info2: vec4<f32>,

    // Third packet of information.
    // For entities, slots [0, 1, 2, 3] are [body_color_r, body_color_g, body_color_b, body_color_a] respectively.
    info3: vec4<f32>, 

    // Fourth packet of information.
    // For entities, slots [0, 1, 2, 3] are [legs_color_r, legs_color_g, legs_color_b, legs_color_a] respectively.
    info4: vec4<f32>
};

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,

    // What does this data represent?
    // Values [0, 1] represent [entity, fruit] respectively.
    @location(1) @interpolate(flat) data_type: u32,

    // First packet of information.
    // For entities, slots [0, 1, 2, 3] are [velocity, head_sprite_index, body_sprite_index, legs_sprite_index] respectively.
    // For fruits, slots [0, 1, 2, 3] are [speed, nothing, nothing, nothing] respectively.
    @location(2) @interpolate(flat) info1: vec4<f32>,

    // Second packet of information.
    // For entities, slots [0, 1, 2, 3] are [head_color_r, head_color_g, head_color_b, head_color_a] respectively.
    // For fruits, slots [0, 1, 2, 3] are [color_r, color_g, color_b, color_a] respectively.
    @location(3) @interpolate(flat) info2: vec4<f32>,

    // Third packet of information.
    // For entities, slots [0, 1, 2, 3] are [body_color_r, body_color_g, body_color_b, body_color_a] respectively.
    @location(4) @interpolate(flat) info3: vec4<f32>, 

    // Fourth packet of information.
    // For entities, slots [0, 1, 2, 3] are [legs_color_r, legs_color_g, legs_color_b, legs_color_a] respectively.
    @location(5) @interpolate(flat) info4: vec4<f32>
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> entities: array<ShaderData>;

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let entity = entities[in.instance_index];

    out.clip_position = mesh2d_position_local_to_clip(entity.transform, vec4<f32>(in.position.xy, 0.0, 1.0));

    out.uv = in.uv;

    out.data_type = entity.data_type;

    out.info1 = entity.info1;
    out.info2 = entity.info2;
    out.info3 = entity.info3;
    out.info4 = entity.info4;
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.data_type == BUG_ENTITY {
        return entity_fragment(in);
    }
    if in.data_type == FRUIT_ENTITY {
        return fruit_fragment(in);
    }
    else {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
}