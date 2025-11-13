use bevy::{
    prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d}
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EntityRenderer {
    #[storage(0, read_only)]
    pub entities: Handle<ShaderStorageBuffer>,

    #[texture(1)]
    #[sampler(2)]
    pub atlas_texture: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub noise_texture: Handle<Image>
}

impl Material2d for EntityRenderer {
    fn fragment_shader() -> ShaderRef {
        return "shaders/trial.wgsl".into();
    }

    fn vertex_shader() -> ShaderRef {
        return "shaders/trial.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }

}