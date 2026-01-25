@group(#{MATERIAL_BIND_GROUP}) @binding(5)
var main_tex_fruits: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(6)
var main_sampler_fruits: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(7)
var noise_tex_fruits: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(8)
var noise_sampler_fruits: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(9)
var<uniform> time: f32;

fn fruit_fragment(in: VertexOutput) -> vec4<f32> {
    let base = textureSample(main_tex_fruits, main_sampler_fruits, in.uv);

    let index = floor(time * in.info1.x);

    let noise_uv = mirror(in.uv + vec2<f32>(0, 0.56 * index));

    let noise = textureSample(noise_tex_fruits, noise_sampler_fruits, noise_uv);

    let mix = noise * base * in.info2;

    return vec4<f32> (
        mix.rgb, mix.a * 0.9
    );
}