use bevy::{
    prelude::*, render::render_resource::AsBindGroup, sprite_render::{Material2d, AlphaMode2d}, shader::ShaderRef
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FuzzMaterial {
    #[uniform(0)]
    pub material_color: LinearRgba,

    #[uniform(1)]
    pub velocity: f32,

    #[texture(2)]
    #[sampler(3)]
    pub main_tex: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub noise_tex: Handle<Image>
}

impl Material2d for FuzzMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/fuzz.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}