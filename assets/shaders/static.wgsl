#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> time: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> speed: f32;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var main_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var main_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(5) var noise_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var noise_sampler: sampler;

fn mirror(uv: vec2<f32>) -> vec2<f32> {
    return abs(fract(uv) * 2 - 1);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(main_tex, main_sampler, mesh.uv);

    let index = floor(time * speed);

    let noise_uv = mirror(mesh.uv + vec2<f32>(0, 0.56 * index));

    let noise = textureSample(noise_tex, noise_sampler, noise_uv);

    let mix = base * color * noise;

    return vec4<f32>(
        mix.rgb, base.a * 0.9
    );
}