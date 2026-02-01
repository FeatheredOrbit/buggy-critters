use bevy::prelude::*;
use rand::Rng;

use crate::bug_entity::components::{idle_components::*, render_components::BugEntityRoot, shared_components::{Action, Idling, RngComponent, StateChangeRequired, States}};

pub fn idle_state(
    mut query: Query<(Entity, &mut IdleStateBundle, &mut RngComponent), (With<BugEntityRoot>, With<Action>, With<Idling>)>, 
    time: Res<Time>,
    mut commands: Commands
) {
    for (entity, mut idle_bundle, mut rng) in &mut query {

        idle_bundle.time_to_action -= time.delta_secs();

        if idle_bundle.time_to_action <= 0.0 {
            find_next_state(entity, &idle_bundle.idle_behaviours, &mut rng, &mut commands);
            idle_bundle.time_to_action = idle_bundle.action_timer;
        }
    }
}

fn find_next_state(entity: Entity, behaviours: &Vec<IdleBehaviour>, rng: &mut RngComponent, mut commands: &mut Commands) {
    let mut probability: i32 = 0;

    for behaviour in behaviours.iter() {
        probability += behaviour.weight;
    }

    let mut cumulative = 0;

    let chance = rng.0.random_range(0..probability);

    for behaviour in behaviours.iter() {
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