use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::materials::death_explosion_renderer::{DeathExplosionRenderer, components::DeathExplosion, shader_data::DeathExplosionShaderData};

pub fn update_render(
    query: Query<(&DeathExplosion, &GlobalTransform)>,
    material_query: Query<&MeshMaterial2d<DeathExplosionRenderer>>,
    materials: Res<Assets<DeathExplosionRenderer>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) {

    let mut data: Vec<DeathExplosionShaderData> = vec![];

    for (explosion, transform) in &query {
        println!("Diddy blud");
        data.push(
            DeathExplosionShaderData {
                transform: transform.compute_transform().to_matrix().to_cols_array_2d(),
                fragment_amount: [explosion.fragment_amount, 0, 0, 0]
            }
        );
    }

    if data.len() == 0 { return }

    if let Some(mat_handle) = material_query.iter().next() {
        if let Some(material) = materials.get(mat_handle) {

            if let Some(storage_buffer) = storage_buffers.get_mut(&material.dead_entities) {
                storage_buffer.set_data(data);
            }
        }
    }
}