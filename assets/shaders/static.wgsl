#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1) var main_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var main_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3) var noise_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var noise_sampler: sampler;

const rolls = 5;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(main_tex, main_sampler, mesh.uv);

    let mix = base * color;

    return vec4<f32>(
        mix.rgb, base.a / 0.9
    );
}