use bevy::prelude::*;

pub fn is_within_camera
(
    entity_transform: &GlobalTransform, 
    entity_size: (f32, f32),
    camera_transform: &GlobalTransform, 
    camera_projection: &Projection
) -> bool
{
    let entity_translation = entity_transform.translation().xy();

    let entity_left = entity_translation.x - entity_size.0 / 2.0;
    let entity_right = entity_translation.x + entity_size.0 / 2.0;

    let entity_top = entity_translation.y + entity_size.1 / 2.0;
    let entity_bottom = entity_translation.y - entity_size.1 / 2.0;

    let camera_translation = camera_transform.translation().xy();
    
    match camera_projection {
        Projection::Orthographic(projection) => {
            let camera_size = projection.area.size();

            let camera_left = camera_translation.x - camera_size.x / 2.0;
            let camera_right = camera_translation.x + camera_size.x / 2.0;

            let camera_top = camera_translation.y + camera_size.y / 2.0;
            let camera_bottom = camera_translation.y - camera_size.y / 2.0;

            return entity_left <= camera_right && entity_right >= camera_left && entity_top >= camera_bottom && entity_bottom <= camera_top;
 
        },

        _ => { return false; }
    }

}