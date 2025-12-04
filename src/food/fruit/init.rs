use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

use crate::materials::food_materials::*;

use crate::food::fruit::components::*;
use crate::materials::food_utils::FruitShaderData;

pub fn spawn
(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<FruitRenderer>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let texture: Handle<Image> = asset_server.load("art/food/fruit.png");
    let noise: Handle<Image> = asset_server.load("art/other/noise_texture_array.png");

    let data: Vec<FruitShaderData> = vec![];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let mesh_handle = meshes.add(Mesh::from(Rectangle::new(120.0, 120.0)));

    let material_handle = materials.add(FruitRenderer {
        fruits: buffer,
        main_texture: texture,
        noise_texture: noise,
        time: 0.0
    });


    for i in 0..1 {

        commands.spawn(())

        // Identifier
        .insert(Fruit)

        // Transform and GlobalTransform
        .insert((
            Transform::from_xyz(0.0, 0.0, -i as f32),
            GlobalTransform::default()
        ))

        // Values
        .insert((

            NutritionalValue(50.0),

        ))

        // Add the renderer and mesh
        .insert((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone())
        ));

    }
}