#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1) var main_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var main_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var noise_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var noise_sampler: sampler;

fn mirror(uv: vec2<f32>) -> vec2<f32> {
    return abs(fract(uv) * 2 - 1);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(main_tex, main_sampler, mesh.uv);

    let noise_uv = mesh.uv + vec2<f32>(-velocity, 0) / 150.0;

    let mirrored_uv = mirror(noise_uv);

    let noise_texture = textureSample(noise_tex, noise_sampler, mirrored_uv);

    let result = (base * (noise_texture * 0.2) * base.a * 3) * color;

    return vec4<f32>(
        result.rgb, base.a * 0.9
    );
}