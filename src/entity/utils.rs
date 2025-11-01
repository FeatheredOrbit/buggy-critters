use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::constants::*;

use crate::entity::components::render_components::EntityRoot;
use crate::resources::CurrentlySelectedEntity;
use crate::scene::components::MainCamera;
use crate::{entity::components::{shared_components::*, utils_components::*}, resources::EntityGrid};

pub fn update_velocity(mut query: Query<(&Transform, &mut PreviousTransform, &mut Velocity), With<Moving>>, time: Res<Time>) {
    for (transform, mut previous_transform, mut velocity) in &mut query {

        let current_pos = transform.translation.xy();
        let previous_pos = previous_transform.0;

        velocity.0 = (current_pos - previous_pos) / time.delta_secs();

        previous_transform.0 = current_pos;
    }
}

pub fn update_entity_grid(query: Query<(Entity, &Transform), With<EntityRoot>>, mut grid: ResMut<EntityGrid>) {
    grid.0.clear();

    for (entity, transform) in &query {

        let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

        grid.0.entry((cell_x, cell_y)).or_default().push(entity);

    }
}

pub fn select_entity
(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    entities: Query<(Entity, &GlobalTransform, &PhysicalTraits), With<EntityRoot>>,
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