use bevy::prelude::*;

use crate::entity::components::shared_components::{NextState, States, *};

pub mod actions;
pub mod searching;
pub mod moving;

pub fn change_state(mut query: Query<(Entity, &mut NextState)>, mut commands: Commands) {
    for (entity, mut next_state) in &mut query {

        if next_state.0 != States::None {

            commands.entity(entity)
            .remove::<Action>()
            .remove::<Idle>()
            .remove::<Searching>()
            .remove::<SearchingNew>()
            .remove::<SearchingFood>()
            .remove::<Moving>()
            .remove::<MovingNew>()
            .remove::<MovingFood>();

            match next_state.0 {
                States::Idle => commands.entity(entity).insert((Action, Idle)),

                States::SearchingNew => commands.entity(entity).insert((Searching, SearchingNew)),

                States::SearchingFood => commands.entity(entity).insert((Searching, SearchingFood)),

                States::MovingNew => commands.entity(entity).insert((Moving, MovingNew)),

                States::MovingFood => commands.entity(entity).insert((Moving, MovingFood)),

                States::None => commands.entity(entity).insert(())
            };

            next_state.0 = States::None;
        }
    }
}