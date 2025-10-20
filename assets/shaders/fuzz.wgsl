#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var main_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var main_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let texture = textureSample(main_tex, main_sampler, mesh.uv);

    return vec4<f32>(
        texture.rgb * material_color.rgb, texture.a * material_color.a
    );
}
