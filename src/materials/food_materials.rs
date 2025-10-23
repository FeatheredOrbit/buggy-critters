use bevy::{
    prelude::*, render::render_resource::AsBindGroup, sprite_render::{Material2d, AlphaMode2d}, shader::ShaderRef
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StaticMaterial {
    #[uniform(0)]
    pub material_color: LinearRgba,

    #[uniform(1)]
    pub time: f32,

    #[uniform(2)]
    pub speed: f32,

    #[texture(3)]
    #[sampler(4)]
    pub main_tex: Handle<Image>,

    #[texture(5)]
    #[sampler(6)]
    pub noise_tex: Handle<Image>
}

impl Material2d for StaticMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/static.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}