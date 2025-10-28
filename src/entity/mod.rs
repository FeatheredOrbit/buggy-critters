use bevy::prelude::*;

mod entity_init;
mod components;
mod render;
mod states;
mod debug;
mod utils;

use states::*;
use entity_init::*;
use states::actions::idle_state::*;
use states::searching::{searching_new_state::*, searching_food_state::*};
use states::moving::moving_new_state::*;

use render::*;

use debug::debug::*;

use utils::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);

        app.add_systems(FixedUpdate, (change_state, idle_state, searching_new_state, searching_food_state));
        app.add_systems(Update, moving_new_state);

        app.add_systems(Update, update_entity_material);

        app.add_systems(PostUpdate, update_velocity);

        app.add_systems(Last, draw_sight_radius);
    }
}