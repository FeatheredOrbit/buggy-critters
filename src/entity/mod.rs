pub mod entity_init;
pub mod components;

use bevy::prelude::*;

mod states;
mod debug;

use states::actions::idle_state::*;
use states::searching::searching_new_state::*;
use states::moving::moving_new_state::*;

use debug::debug::*;


pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity_init::spawn);

        app.add_systems(FixedUpdate, (idle_state, searching_new_state));
        app.add_systems(Update, moving_new_state);

        app.add_systems(Last, draw_sight_radius);
    }
}