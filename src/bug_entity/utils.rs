use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::constants::*;

use crate::bug_entity::components::attribute_components::PhysicalTraits;
use crate::bug_entity::components::render_components::BugEntityRoot;
use crate::resources::{CurrentlySelectedEntity, LargestEntitySight};
use crate::scene::components::MainCamera;
use crate::{bug_entity::components::{shared_components::*, utils_components::*}, resources::EntityGrid};

pub fn update_velocity(mut query: Query<(&Transform, &mut PreviousTransform, &mut Velocity), With<Moving>>, time: Res<Time>) {
    for (transform, mut previous_transform, mut velocity) in &mut query {

        let current_pos = transform.translation.xy();
        let previous_pos = previous_transform.0;

        velocity.0 = (current_pos - previous_pos) / time.delta_secs();

        previous_transform.0 = current_pos;
    }
}

pub fn update_entity_grid(
    query: Query<(Entity, &Transform), With<BugEntityRoot>>, 
    mut grid: ResMut<EntityGrid>,
    largest_sight: Res<LargestEntitySight>
) {
    grid.0.clear();

    for (entity, transform) in &query {

        let cell_x: i32;
        let cell_y: i32;

        if let Some(sight_radius) = largest_sight.radius {
            let cell_size = sight_radius.ceil() * 2.0 / 3.0; // diameter * (1/3)
            cell_x = (transform.translation.x / cell_size).floor() as i32;
            cell_y = (transform.translation.y / cell_size).floor() as i32;
        }
        else {
            cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
            cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;
        }

        grid.0.entry((cell_x, cell_y)).or_default().push((entity, transform.clone()));

    }
}

pub fn calculate_grid_cell_position(transform: &Transform, largest_entity_sight: &Res<LargestEntitySight>) -> (i32, i32) {
    let cell_x: i32;
    let cell_y: i32;

    if let Some(sight_radius) = largest_entity_sight.radius {
        let cell_size = sight_radius.ceil() * 2.0 / 3.0; // diameter * (1/3)
        cell_x = (transform.translation.x / cell_size).floor() as i32;
        cell_y = (transform.translation.y / cell_size).floor() as i32;
    }
    else {
        cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;
    }

    return (cell_x, cell_y);
}

pub fn select_entity
(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    entities: Query<(Entity, &GlobalTransform, &PhysicalTraits), With<BugEntityRoot>>,
    camera: Single<(&Camera, &GlobalTransform, &Projection), With<MainCamera>>,
    mut current_entity: ResMut<CurrentlySelectedEntity>
) 
{

    let (camera, camera_transform, projection) = *camera;

    let Some(cursor_position) = window.cursor_position() else { return };

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Ok(camera_to_world) = camera.viewport_to_world_2d(camera_transform, cursor_position) {

            let mut projection_scale = 1.0;
            
            match projection {
                Projection::Orthographic(ortho) => {
                    projection_scale = ortho.scale;
                },

                _ => {}
            }

            for (entity, entity_transform, traits) in &entities {
                let distance = entity_transform.translation().xy().distance(camera_to_world);

                if distance <= 20.0 * traits.size * projection_scale {
                    current_entity.0 = Some(entity);
                    return;
                }
            }

            current_entity.0 = None;

        }
    }

}