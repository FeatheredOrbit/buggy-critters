use bevy::prelude::*;
use rand::Rng;

use crate::bug_entity::components::{idle_components::*, render_components::BugEntityRoot, shared_components::{Action, Dead, Idling, RngComponent, StateChangeRequired, States}};

pub fn idle_state(
    mut query: Query<(Entity, &mut IdleStateBundle, &mut RngComponent), (With<BugEntityRoot>, With<Action>, With<Idling>, Without<Dead>)>, 
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity, mut idle_bundle, mut rng) in &mut query {

        idle_bundle.time_to_action -= time.delta_secs();

        if idle_bundle.time_to_action <= 0.0 {
            find_next_state(entity, &idle_bundle, &mut rng, &mut commands);
            idle_bundle.time_to_action = idle_bundle.action_timer;
        }
    }
}

fn find_next_state(entity: Entity, idle_bundle: &IdleStateBundle, rng: &mut RngComponent, mut commands: &mut Commands) {
    let probability: i32 = idle_bundle.idle_behaviours_cumulative_weight;

    let mut cumulative = 0;

    let chance = rng.0.random_range(0..probability);

    for behaviour in idle_bundle.idle_behaviours.iter() {
        cumulative += behaviour.weight;

        if cumulative > chance {
            call_next_state(entity, &behaviour.name, &mut commands);
            break;
        }
    }


}

fn call_next_state(entity: Entity, state: &IdleStates, commands: &mut Commands) {
    match state {
        IdleStates::Move => {
            commands.entity(entity).insert(StateChangeRequired(States::SearchingNew));
        },

        IdleStates::SearchFood => {
            commands.entity(entity).insert(StateChangeRequired(States::SearchingFood));
        }

        _ => {}
    }
}