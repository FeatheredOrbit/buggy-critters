use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entity::components::{idle_components::*, render_components::EntityRoot, shared_components::{NextState, States}};

pub fn idle_state(mut query: Query<(&mut IdleStateBundle, &mut NextState), With<EntityRoot>>, time: Res<Time>) {
    for (mut idle_bundle, mut next_state) in &mut query {

        idle_bundle.time_to_action -= time.delta_secs();

        if idle_bundle.time_to_action <= 0.0 {
            find_next_state(&idle_bundle.idle_behaviours, &mut next_state);
            idle_bundle.time_to_action = idle_bundle.action_timer;
        }
    }
}

fn find_next_state(behaviours: &Vec<IdleBehaviour>, next_state: &mut NextState) {
    let mut probability: i32 = 0;

    for behaviour in behaviours.iter() {
        probability += behaviour.weight;
    }

    let mut cumulative = 0;

    let mut rng = thread_rng();
    let chance = rng.gen_range(0..probability);

    for behaviour in behaviours.iter() {
        cumulative += behaviour.weight;

        if cumulative > chance {
            call_next_state(&behaviour.name, next_state);
            break;
        }
    }


}

fn call_next_state(state: &IdleStates, next_state: &mut NextState) {
    match state {
        IdleStates::Move => {
            next_state.0 = States::SearchingNew;
        },

        IdleStates::SearchFood => {
            next_state.0 = States::SearchingFood;
        }

        _ => {}
    }
}