use bevy::prelude::*;

use crate::entity::components::{render_components::EntityRoot, shared_components::{NextState, States, *}};

pub mod actions;
pub mod searching;
pub mod moving;

pub fn change_state(mut query: Query<(Entity, &mut NextState, &mut CurrentState), With<EntityRoot>>, mut commands: Commands) {
    for (entity, mut next_state, mut current_state) in &mut query {

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
                States::Idle => { 
                    commands.entity(entity).insert((Action, Idle)); 
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
                    commands.entity(entity).insert((Moving, MovingNew)); 
                    current_state.0 = States::MovingNew;
                },

                States::MovingFood => { 
                    commands.entity(entity).insert((Moving, MovingFood)); 
                    current_state.0 = States::MovingFood;
                },

                States::None => { commands.entity(entity).insert(()); }
            };

            next_state.0 = States::None;
        }
    }
}