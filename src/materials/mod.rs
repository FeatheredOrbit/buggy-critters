use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod entity_materials;

use entity_materials::*;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FuzzMaterial>::default());

        app.add_systems(Startup, instance_materials);

    }
}

fn instance_materials(mut commands: Commands, mut materials: ResMut<Assets<FuzzMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let texture = Handle::<Image>::default();

    let materials = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),

        MeshMaterial2d(materials.add(FuzzMaterial {
            material_color: LinearRgba::WHITE,
            main_tex: texture.clone(),
            noise_tex: texture.clone(),
            velocity: 0.0
        }))
    )).id();

    commands.entity(materials).despawn();
}


