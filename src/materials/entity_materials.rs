use bevy::{
    prelude::*, render::render_resource::AsBindGroup, sprite_render::{Material2d, AlphaMode2d}, shader::ShaderRef
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FuzzMaterial {
    #[uniform(0)]
    pub material_color: LinearRgba,

    #[texture(1)]
    #[sampler(2)]
    pub main_tex: Handle<Image>
}

impl Material2d for FuzzMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/fuzz.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}