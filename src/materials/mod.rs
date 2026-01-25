use bevy::{prelude::*, render::storage::ShaderStorageBuffer, sprite_render::Material2dPlugin};

pub mod entity_materials;
pub mod food_materials;
pub mod food_utils;
pub mod renderer;
pub mod shader_data;

use entity_materials::*;
use food_materials::*;

use crate::materials::renderer::{Renderer, RendererHandle, SHADER_HANDLE};

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();

        let misc_shader = include_str!("../../assets/shaders/misc.wgsl");
        let main_shader = include_str!("../../assets/shaders/renderer.wgsl");
        let joined_shader = format!("{} {}", main_shader, misc_shader); 

        shaders.insert(SHADER_HANDLE.id(), Shader::from_wgsl(joined_shader, "Renderer Shader")).unwrap();

        app.add_plugins(Material2dPlugin::<Renderer>::default());
        app.add_plugins(Material2dPlugin::<EntityRenderer>::default());
        app.add_plugins(Material2dPlugin::<FruitRenderer>::default());

        app.add_systems(PreStartup, instance_materials);

    }
}

fn instance_materials
(
    mut commands: Commands, 
    mut render: ResMut<Assets<Renderer>>,
    mut entity_render: ResMut<Assets<EntityRenderer>>,
    mut fruit_renderer: ResMut<Assets<FruitRenderer>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    asset_server: Res<AssetServer>
) 
{
    let texture = Handle::<Image>::default();

    let atlas_bugs: Handle<Image> = asset_server.load("art/bugs/body_parts/atlas.png");
    let noise_bugs: Handle<Image> = asset_server.load("art/other/noise_texture.png");

    let data: Vec<f32> = vec![];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let renderer_handle = render.add(Renderer {
            entities: buffer.clone(),
            atlas_texture: atlas_bugs.clone(),
            noise_texture: noise_bugs.clone()
        }
    );

    let materials = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),

        MeshMaterial2d(render.add(Renderer {
            entities: buffer.clone(),
            atlas_texture: texture.clone(),
            noise_texture: texture.clone()
        })),

        MeshMaterial2d(entity_render.add(EntityRenderer {
            entities: buffer.clone(),
            atlas_texture: texture.clone(),
            noise_texture: texture.clone()
        })),

        MeshMaterial2d(fruit_renderer.add(FruitRenderer {
            fruits: buffer.clone(),
            main_texture: texture.clone(),
            noise_texture: texture.clone(),

            time: 0.0
        })),

    )).id();

    commands.insert_resource(RendererHandle(renderer_handle));

    commands.entity(materials).despawn();
}


