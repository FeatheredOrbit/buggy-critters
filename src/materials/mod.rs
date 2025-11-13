use bevy::{prelude::*, render::storage::ShaderStorageBuffer, sprite_render::Material2dPlugin};

pub mod entity_materials;
pub mod food_materials;
pub mod entity_utils;

use entity_materials::*;
use food_materials::*;

use crate::materials::entity_utils::EntityShaderData;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<EntityRenderer>::default());
        app.add_plugins(Material2dPlugin::<StaticMaterial>::default());

        app.add_systems(PreStartup, instance_materials);

    }
}

fn instance_materials
(
    mut commands: Commands, 
    mut entity_render: ResMut<Assets<EntityRenderer>>, 
    mut static_mats: ResMut<Assets<StaticMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let texture = Handle::<Image>::default();

    let data: Vec<EntityShaderData> = vec![];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let materials = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),

        MeshMaterial2d(entity_render.add(EntityRenderer {
            entities: buffer,
            atlas_texture: texture.clone(),
            noise_texture: texture.clone()
        })),

        MeshMaterial2d(static_mats.add(StaticMaterial {
            material_color: LinearRgba::WHITE,
            time: 0.0,
            speed: 0.0,
            main_tex: texture.clone(),
            noise_tex: texture.clone(),
        }))
        
    )).id();



    commands.entity(materials).despawn();
}


