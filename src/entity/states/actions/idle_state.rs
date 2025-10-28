use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entity::components::{shared_components::{*, NextState, States}, idle_components::*};

pub fn idle_state(mut query: Query<(&IdleBehaviours, &mut TimeToAction, &ActionTimer, &mut NextState), (With<Action>, With<Idle>)>, time: Res<Time>) {
    query.par_iter_mut().for_each(| (behaviours, mut time_to_action, action_timer, mut next_state) | {

        time_to_action.0 -= time.delta_secs();

        if time_to_action.0 <= 0.0 {
            find_next_state(behaviours, &mut next_state);
            time_to_action.0 = action_timer.0;
        }
    })
}

fn find_next_state(behaviours: &IdleBehaviours, next_state: &mut NextState) {
    let mut probability: i32 = 0;

    for behaviour in behaviours.0.iter() {
        probability += behaviour.weight;
    }

    let mut cumulative = 0;

    let mut rng = thread_rng();
    let chance = rng.gen_range(0..probability);

    for behaviour in behaviours.0.iter() {
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