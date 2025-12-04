use bevy::{prelude::*, render::storage::ShaderStorageBuffer, sprite_render::Material2dPlugin};

pub mod entity_materials;
pub mod food_materials;
pub mod entity_utils;
pub mod food_utils;

use entity_materials::*;
use food_materials::*;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<EntityRenderer>::default());
        app.add_plugins(Material2dPlugin::<FruitRenderer>::default());

        app.add_systems(PreStartup, instance_materials);

    }
}

fn instance_materials
(
    mut commands: Commands, 
    mut entity_render: ResMut<Assets<EntityRenderer>>,
    mut fruit_renderer: ResMut<Assets<FruitRenderer>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let texture = Handle::<Image>::default();

    let data: Vec<f32> = vec![];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let materials = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),

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

    commands.entity(materials).despawn();
}


