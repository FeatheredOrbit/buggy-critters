use bevy::prelude::*;

use crate::{bug_entity::components::{render_components::BugEntityRoot, shared_components::{States, *}}, materials::renderer::components::RenderChanged};

pub mod actions;
pub mod moving;
pub mod searching;

pub fn change_state(
    mut query: Query<(Entity, &StateChangeRequired, &mut CurrentState), (With<BugEntityRoot>, Without<Dead>)>, 
    mut commands: Commands
) {
    for (entity, next_state, mut current_state) in &mut query {

        if next_state.0 == States::None { unreachable!() }

        match current_state.0 {
            States::Idle => {
                commands.entity(entity).remove::<(Action, Idling)>();
            },
            States::SearchingNew => {
                commands.entity(entity).remove::<(Searching, SearchingNew)>();
            },
            States::SearchingFood => {
                commands.entity(entity).remove::<(Searching, SearchingFood)>();
            },
            States::MovingNew => {
                commands.entity(entity).remove::<(Moving, MovingNew)>();
            },
            States::MovingFood => {
                commands.entity(entity).remove::<(Moving, MovingFood)>();
            },
            States::None => {}
        }

        match next_state.0 {
            States::Idle => { 
                commands.entity(entity).insert((Action, Idling)); 
                current_state.0 = States::Idle;
            },

            States::SearchingNew => { 
                commands.entity(entity).insert((Searching, SearchingNew)); 
                current_state.0 = States::SearchingNew;
            },

            States::SearchingFood => { 
                commands.entity(entity).insert((Searching, SearchingFood)); 
                current_state.0 = States::SearchingFood;
            },

            States::MovingNew => { 
                commands.entity(entity).insert((Moving, MovingNew, RenderChanged)); 
                current_state.0 = States::MovingNew;
            },

            States::MovingFood => { 
                commands.entity(entity).insert((Moving, MovingFood, RenderChanged)); 
                current_state.0 = States::MovingFood;
            },

            States::None => { commands.entity(entity).insert(()); }
        };

        commands.entity(entity).remove::<StateChangeRequired>();
    }
}