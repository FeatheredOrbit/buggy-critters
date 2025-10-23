use bevy::prelude::*;

use crate::materials::food_materials::*;

use crate::food::fruit::components::*;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<StaticMaterial>>, meshes: ResMut<Assets<Mesh>>) {

    let entity = commands.spawn(())

    // Identifier
    .insert(Fruit)

    // Transform and GlobalTransform
    .insert((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default()
    ))

    // Values
    .insert((

        NutritionalValue(50.0),

    ))

    .id();

    render(&mut commands, &entity, &asset_server, materials, meshes);

}

fn render(commands: &mut Commands, entity: &Entity, asset_server: &Res<AssetServer>, mut materials: ResMut<Assets<StaticMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {

    let fruit_texture: Handle<Image> = asset_server.load("art/food/fruit.png");
    let noise_texture: Handle<Image> = asset_server.load("art/other/noise_texture_array.png");

    commands.entity(*entity)
    

    // Creates a mesh and adds a material for the fruit

    .insert((

        Mesh2d(meshes.add(Rectangle::new(120.0, 120.0))),

        MeshMaterial2d(materials.add(StaticMaterial {
            material_color: LinearRgba::RED,
            time: 0.0,
            speed: 0.0,
            main_tex: fruit_texture.clone(),
            noise_tex: noise_texture.clone()
        }))

    ));

}