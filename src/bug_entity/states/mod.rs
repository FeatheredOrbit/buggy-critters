use bevy::prelude::*;

use crate::bug_entity::components::{idle_components::IdleStateBundle, render_components::BugEntityRoot, shared_components::{NextState, States, *}};

pub mod actions;
pub mod moving;
pub mod searching;

pub fn change_state(mut query: Query<(Entity, &mut NextState, &mut CurrentState), With<BugEntityRoot>>, mut commands: Commands) {
    for (entity, mut next_state, mut current_state) in &mut query {

        if next_state.0 != States::None {

            commands.entity(entity)
            .remove::<IdleStateBundle>()
            .remove::<(SearchingNewBundle, Searching)>()
            .remove::<(SearchingFoodBundle, Searching)>()
            .remove::<(MovingNewBundle, Moving)>()
            .remove::<(MovingFoodBundle, Moving)>();

            match next_state.0 {
                States::Idle => { 
                    commands.entity(entity).insert(IdleStateBundle::default()); 
                    current_state.0 = States::Idle;
                },

                States::SearchingNew => { 
                    commands.entity(entity).insert((SearchingNewBundle, Searching)); 
                    current_state.0 = States::SearchingNew;
                },

                States::SearchingFood => { 
                    commands.entity(entity).insert((SearchingFoodBundle, Searching)); 
                    current_state.0 = States::SearchingFood;
                },

                States::MovingNew => { 
                    commands.entity(entity).insert((MovingNewBundle, Moving)); 
                    current_state.0 = States::MovingNew;
                },

                States::MovingFood => { 
                    commands.entity(entity).insert((MovingFoodBundle, Moving)); 
                    current_state.0 = States::MovingFood;
                },

                States::None => { commands.entity(entity).insert(()); }
            };

            next_state.0 = States::None;
        }
    }
}