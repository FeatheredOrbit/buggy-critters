use bevy::{
    prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d}
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FruitRenderer {
    #[storage(0, read_only)]
    pub fruits: Handle<ShaderStorageBuffer>,

    #[texture(1)]
    #[sampler(2)]
    pub main_texture: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub noise_texture: Handle<Image>,

    #[uniform(5)]
    pub time: f32
}

impl Material2d for FruitRenderer {
    fn fragment_shader() -> ShaderRef {
        return "shaders/fruit_renderer.wgsl".into();
    }
    
    fn vertex_shader() -> ShaderRef {
        return "shaders/fruit_renderer.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}