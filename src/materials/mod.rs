use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod entity_materials;
pub mod food_materials;

use entity_materials::*;
use food_materials::*;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FuzzMaterial>::default());
        app.add_plugins(Material2dPlugin::<StaticMaterial>::default());

        app.add_systems(PreStartup, instance_materials);

    }
}

fn instance_materials(mut commands: Commands, mut fuzz_mats: ResMut<Assets<FuzzMaterial>>, mut static_mats: ResMut<Assets<StaticMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let texture = Handle::<Image>::default();

    let materials = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),

        MeshMaterial2d(fuzz_mats.add(FuzzMaterial {
            material_color: LinearRgba::WHITE,
            main_tex: texture.clone(),
            noise_tex: texture.clone(),
            velocity: 0.0
        })),

        MeshMaterial2d(static_mats.add(StaticMaterial {
            material_color: LinearRgba::WHITE,
            main_tex: texture.clone(),
            noise_tex: texture.clone(),
        }))
        
    )).id();



    commands.entity(materials).despawn();
}


