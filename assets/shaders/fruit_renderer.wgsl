@group(#{MATERIAL_BIND_GROUP}) @binding(5)
var main_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(6)
var main_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(7)
var noise_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(8)
var noise_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(9)
var<uniform> time: f32;

fn fruit_fragment(in: VertexOutput) -> vec4<f32> {
    let base = textureSample(main_tex, main_sampler, in.uv);

    let index = floor(time * in.speed);

    let noise_uv = mirror(in.uv + vec2<f32>(0, 0.56 * index));

    let noise = textureSample(noise_tex, noise_sampler, noise_uv);

    let mix = noise * base * in.color;

    // Debug: return a solid color so we can verify the material/pipeline is active
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}