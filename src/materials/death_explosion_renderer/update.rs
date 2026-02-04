use bevy::prelude::*;

use crate::materials::death_explosion_renderer::components::DeathExplosion;

pub fn update_explosions(
    mut query: Query<&mut DeathExplosion>,
    time: Res<Time>
) {
    for mut explosion in &mut query {

        explosion.duration -= time.elapsed_secs();

    }
}