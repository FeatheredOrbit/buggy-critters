@group(#{MATERIAL_BIND_GROUP}) @binding(1) 
var atlas: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var atlas_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) 
var noise_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(4) 
var noise_sampler: sampler;

fn entity_fragment(in: VertexOutput) -> vec4<f32> {
    let atlas_width = 360.0;
    let atlas_height = 140.0;

    let sprite_size = vec2<f32>(120.0, 140.0);

    let uv_size = sprite_size / vec2<f32>(atlas_width, atlas_height);

    let head_column = u32(in.info1.y) % 3;
    let head_row = u32(in.info1.y) / 3;

    let head_uv_offset = vec2<f32>(f32(head_column), f32(head_row)) * uv_size;

    let body_column = u32(in.info1.z) % 3;
    let body_row = u32(in.info1.z) / 3;

    let body_uv_offset = vec2<f32>(f32(body_column), f32(body_row)) * uv_size;

    let legs_column = u32(in.info1.w) % 3;
    let legs_row = u32(in.info1.w) / 3;

    let legs_uv_offset = vec2<f32>(f32(legs_column), f32(legs_row)) * uv_size;

    let final_head_uv = head_uv_offset + in.uv * uv_size;
    let final_body_uv = body_uv_offset + in.uv * uv_size;
    let final_legs_uv = legs_uv_offset + in.uv * uv_size;

    let head = textureSample(atlas, atlas_sampler, final_head_uv) * in.info2;
    let body = textureSample(atlas, atlas_sampler, final_body_uv) * in.info2;
    let legs = textureSample(atlas, atlas_sampler, final_legs_uv) * in.info4;

    let base = head + body + legs;

    let noise_uv = in.uv + vec2<f32>(-in.info1.x, 0) / 150.0;

    let mirrored_uv = mirror(noise_uv);

    let noise_texture = textureSample(noise_tex, noise_sampler, mirrored_uv);

    let result = (base * (noise_texture * 0.2) * base.a * 3);

    return result;
}