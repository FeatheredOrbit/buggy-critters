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
    entities: Query<(Entity, &GlobalTransform), With<EntityRoot>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut current_entity: ResMut<CurrentlySelectedEntity>
) 
{

    let (camera, camera_transform) = *camera;

    let Some(cursor_position) = window.cursor_position() else { return };

    for (entity, entity_transform) in &entities {

        if mouse_buttons.just_pressed(MouseButton::Left) {

            if let Ok(screen_position) = camera.world_to_viewport(camera_transform, entity_transform.translation()) {

                let distance = cursor_position.distance(screen_position);

                if distance <= 10.0 {

                    current_entity.0 = Some(entity);

                } else { current_entity.0 = None };

            }

        }

    }

}