use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::{bug_entity::components::{render_components::{BodyPartsIndexes, BugEntityRoot}, utils_components::Velocity}, food::fruit_entity::components::{FruitEntityRoot, NutritionalValue}, materials::renderer::{Renderer, shader_data::ShaderData}};

pub fn update_renderer(
    bug_query: Query<(&GlobalTransform, &Velocity, &BodyPartsIndexes), With<BugEntityRoot>>,
    fruit_query: Query<(&GlobalTransform, &NutritionalValue), With<FruitEntityRoot>>, 
    material_query: Query<&MeshMaterial2d<Renderer>>,
    mut materials: ResMut<Assets<Renderer>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    time: Res<Time>
) 
{

    let mut buffer: Vec<ShaderData> = vec![];

    for (transform, velocity, indexes) in &bug_query {
        let data = ShaderData::create_for_entity( 
            transform.compute_transform().to_matrix().to_cols_array_2d(), 
            velocity.0.length(),
            indexes.head,
            indexes.body,
            indexes.legs
        );

        buffer.push(data);
    }

    for (transform, nutritional_value) in &fruit_query {
        buffer.push(
            ShaderData::create_for_fruit(
            transform.compute_transform().to_matrix().to_cols_array_2d(), 
            (1.0 / nutritional_value.0) * 250.0
            )
        );
    }

    if buffer.len() > 0 {

        if let Some(mat_handle) = material_query.iter().next() {
        
            if let Some(mat) = materials.get_mut(mat_handle) {

                mat.time = time.elapsed_secs();

                if let Some(buffer_asset) = storage_buffers.get_mut(&mat.entities) {
                    buffer_asset.set_data(buffer.clone());
                }

            }
        }

    }

}