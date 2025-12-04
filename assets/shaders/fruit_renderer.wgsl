#import bevy_sprite::mesh2d_functions::mesh2d_position_local_to_clip;

struct Fruit {
    transform: mat4x4<f32>,

    info1: vec4<f32>
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> fruits: array<Fruit>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var main_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var main_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3)
var noise_tex: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(4)
var noise_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(5)
var<uniform> time: f32;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,

    @location(2) speed: f32
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let fruit = fruits[in.instance_index];

    out.speed = fruit.info1.x;

    out.clip_position = mesh2d_position_local_to_clip(fruit.transform, vec4<f32>(in.position.xy, 0.0, 1.0));
    out.color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    out.uv = in.uv;

    return out;
}

fn mirror(uv: vec2<f32>) -> vec2<f32> {
    return abs(fract(uv) * 2 - 1);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(main_tex, main_sampler, in.uv);

    let index = floor(time * in.speed);

    let noise_uv = mirror(in.uv + vec2<f32>(0, 0.56 * index));

    let noise = textureSample(noise_tex, noise_sampler, noise_uv);

    let mix = noise * base * in.color;

    return vec4<f32>(
        mix.rgb, mix.a * 0.9
    );
}