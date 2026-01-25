fn mirror(uv: vec2<f32>) -> vec2<f32> {
    return abs(fract(uv) * 2 - 1);
}