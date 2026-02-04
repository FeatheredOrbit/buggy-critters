use bevy::{
    asset::uuid_handle, prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d, Material2dPlugin}
};

use crate::{constants::ENTITY_DEFAULT_SIZE, materials::renderer::{render::{available_for_rendering, update_renderer}, resources::{EntitiesToRender, RendererHandle, RendererMeshHandle}}};

pub mod shader_data;
mod render;
pub mod resources;
pub mod components;

pub const SHADER_HANDLE: Handle<Shader> = uuid_handle!("38c96d71-9b05-467e-b646-3380f0bdf860");


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct Renderer {
    #[storage(0, read_only)]
    pub entities: Handle<ShaderStorageBuffer>,

    #[texture(1)]
    #[sampler(2)]
    pub atlas_texture_bugs: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub noise_texture_bugs: Handle<Image>,

    #[texture(5)]
    #[sampler(6)]
    pub main_texture_fruit: Handle<Image>,

    #[texture(7)]
    #[sampler(8)]
    pub noise_texture_fruit: Handle<Image>,

    #[uniform(9)]
    pub time: f32


}
impl Material2d for Renderer {
    fn fragment_shader() -> ShaderRef {
        return ShaderRef::Handle(SHADER_HANDLE);
    }

    fn vertex_shader() -> ShaderRef {
        return ShaderRef::Handle(SHADER_HANDLE);
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}

pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();

        let misc_shader = include_str!("../../../assets/shaders/misc.wgsl");
        let entity_renderer_shader = include_str!("../../../assets/shaders/entity_renderer.wgsl");
        let fruit_renderer = include_str!("../../../assets/shaders/fruit_renderer.wgsl");
        let main_shader = include_str!("../../../assets/shaders/main_renderer.wgsl");
        let joined_shader = format!("{} {} {} {}", main_shader, entity_renderer_shader, fruit_renderer, misc_shader); 

        shaders.insert(SHADER_HANDLE.id(), Shader::from_wgsl(joined_shader, "Renderer Shader")).unwrap();

        app.insert_resource(EntitiesToRender::default());
        app.add_plugins(Material2dPlugin::<Renderer>::default());
        app.add_systems(PreStartup, compile_and_init);
        app.add_systems(Update, (available_for_rendering, update_renderer).chain());
    }
}

fn compile_and_init
(
    mut commands: Commands, 
    mut render: ResMut<Assets<Renderer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    asset_server: Res<AssetServer>
) 
{
    let atlas_bugs: Handle<Image> = asset_server.load("art/bugs/body_parts/atlas.png");
    let noise_bugs: Handle<Image> = asset_server.load("art/other/noise_texture.png");

    let texture_fruit: Handle<Image> = asset_server.load("art/food/fruit.png");
    let noise_fruit: Handle<Image> = asset_server.load("art/other/noise_texture_array.png");

    let data: Vec<f32> = vec![];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let mesh_handle = meshes.add(Mesh::from(Rectangle::new(ENTITY_DEFAULT_SIZE.0, ENTITY_DEFAULT_SIZE.1)));

    let renderer_handle = render.add(Renderer {
            entities: buffer.clone(),
            atlas_texture_bugs: atlas_bugs.clone(),
            noise_texture_bugs: noise_bugs.clone(),

            main_texture_fruit: texture_fruit.clone(),
            noise_texture_fruit: noise_fruit.clone(),
            time: 0.0
        }
    );

    let materials = commands.spawn((
        Mesh2d(mesh_handle.clone()),

        MeshMaterial2d(renderer_handle.clone())

    )).id();

    commands.insert_resource(RendererHandle(renderer_handle));
    commands.insert_resource(RendererMeshHandle(mesh_handle));

    commands.entity(materials).despawn();
}
