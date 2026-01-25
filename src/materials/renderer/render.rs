use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::{bug_entity::components::{render_components::{BodyPartsIndexes, BugEntityRoot}, utils_components::Velocity}, food::fruit_entity::components::{FruitEntityRoot, NutritionalValue}, materials::renderer::{Renderer, components::{NoRender, RenderChanged}, resources::EntitiesToRender, shader_data::ShaderData}};

pub fn available_for_rendering
(
    bug_query: Query<(Entity, &ViewVisibility, Option<&NoRender>), (With<BugEntityRoot>, With<RenderChanged>)>,
    fruit_query: Query<(Entity, &ViewVisibility, Option<&NoRender>), (With<FruitEntityRoot>, With<RenderChanged>)>,
    mut commands: Commands
) 
{
    for (entity, visibility, no_render) in &bug_query {
        match (visibility.get(), no_render.is_some()) {
            (true, true) => { commands.entity(entity).remove::<NoRender>(); },
            (false, false) => { commands.entity(entity).insert(NoRender); },
            _ => {}
        }
    }

    for (entity, visibility, no_render) in &fruit_query {
        match (visibility.get(), no_render.is_some()) {
            (true, true) => { commands.entity(entity).remove::<NoRender>(); },
            (false, false) => { commands.entity(entity).insert(NoRender); },
            _ => {}
        }

    }
}

pub fn update_renderer
(
    bug_query: Query<(Entity, &GlobalTransform, &Velocity, &BodyPartsIndexes), (With<BugEntityRoot>, Without<NoRender>)>,
    fruit_query: Query<(Entity, &GlobalTransform, &NutritionalValue), (With<FruitEntityRoot>, Without<NoRender>)>, 
    material_query: Query<&MeshMaterial2d<Renderer>>,
    mut materials: ResMut<Assets<Renderer>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut entities_to_render: ResMut<EntitiesToRender>,
    time: Res<Time>
) 
{
    for (entity, transform, velocity, indexes) in &bug_query {
        if let Some(&idx) = entities_to_render.indexes.get(&entity) {
            entities_to_render.data[idx] = ShaderData::create_for_entity( 
                transform.compute_transform().to_matrix().to_cols_array_2d(), 
                velocity.0.length(),
                indexes.head,
                indexes.body,
                indexes.legs
            );
        }
        else {
            entities_to_render.data.push(
                ShaderData::create_for_entity( 
                    transform.compute_transform().to_matrix().to_cols_array_2d(), 
                    velocity.0.length(),
                    indexes.head,
                    indexes.body,
                    indexes.legs
                )
            );

            let idx = entities_to_render.data.len() - 1;

            entities_to_render.indexes.insert(entity, idx);
        }
    }

    for (entity, transform, nutritional_value) in &fruit_query {
        if let Some(&idx) = entities_to_render.indexes.get(&entity) {
            entities_to_render.data[idx] = ShaderData::create_for_fruit(
                transform.compute_transform().to_matrix().to_cols_array_2d(), 
                (1.0 / nutritional_value.0) * 250.0
            )
        }
        else {
            entities_to_render.data.push(
                ShaderData::create_for_fruit(
                    transform.compute_transform().to_matrix().to_cols_array_2d(), 
                    (1.0 / nutritional_value.0) * 250.0
            )
            );

            let idx = entities_to_render.data.len() - 1;

            entities_to_render.indexes.insert(entity, idx);
        }
    }

    if entities_to_render.data.len() > 0 {

        if let Some(mat_handle) = material_query.iter().next() {
        
            if let Some(mat) = materials.get_mut(mat_handle) {

                mat.time = time.elapsed_secs();

                if let Some(buffer_asset) = storage_buffers.get_mut(&mat.entities) {
                    buffer_asset.set_data(entities_to_render.data.clone());
                }

            }
        }

    }

}