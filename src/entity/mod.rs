pub mod entity_init;
pub mod components;

use bevy::prelude::*;

mod states;

use states::actions::idle_state::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity_init::spawn);

        app.add_systems(Update, idle_state);
    }
}