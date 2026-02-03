use bevy::prelude::*;

use crate::bug_entity::components::{render_components::BugEntityRoot, shared_components::Dead};

pub fn death_state(
    mut query: Query<(Entity, &mut Transform, &mut Dead), With<BugEntityRoot>>,
    time: Res<Time>,
    mut commands: Commands
) {
    
    for (entity, mut transform, mut death_handler) in &mut query {
        if handle_death_animation(&mut transform, &mut death_handler, &time) {
            println!("Animation finished");
            commands.entity(entity).despawn();
        }
    }

}

fn handle_death_animation(transform: &mut Transform, death_handler: &mut Dead, time: &Res<Time>) -> bool {
    death_handler.time_since_animation += time.delta_secs();

    transform.rotate_z(-(death_handler.time_since_animation.exp() * death_handler.animation_acceleration));

    if death_handler.time_since_animation >= 3.0 {
        let t = death_handler.time_since_animation - 3.0;
        let factor = (-t * death_handler.animation_acceleration).exp();

        transform.scale.x *= factor;
        transform.scale.y *= factor;
    }

    return transform.scale.x <= 0.01 && transform.scale.y <= 0.01;
}