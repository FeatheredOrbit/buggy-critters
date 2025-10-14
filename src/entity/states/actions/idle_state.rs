use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entity::components::{shared_components::*, idle_components::*};

pub fn idle_state(mut query: Query<(Entity, &IdleBehaviours, &mut TimeToAction, &ActionTimer), With<Idle>>, mut commands: Commands, time: Res<Time>) {
    for (entity, behaviours, mut time_to_action, action_timer) in &mut query {

        time_to_action.0 -= time.delta_seconds();

        if (time_to_action.0 <= 0.0) {
            find_next_state(entity, behaviours, &mut commands);
            time_to_action.0 = action_timer.0;
        }
    }
}

fn find_next_state(entity: Entity, behaviours: &IdleBehaviours, commands: &mut Commands) {
    let mut probability: i32 = 0;

    for behaviour in behaviours.0.iter() {
        probability += behaviour.weight;
    }

    let mut cumulative = 0;

    let mut rng = thread_rng();
    let chance = rng.gen_range(0..probability);

    for behaviour in behaviours.0.iter() {
        cumulative += behaviour.weight;

        if (cumulative > chance) {
            call_next_state(entity, &behaviour.name, commands);
            break;
        }
    }


}

fn call_next_state(entity: Entity, state: &IdleStates, commands: &mut Commands) {
    match (state) {
        IdleStates::Move => {
            commands.entity(entity).remove::<Idle>().insert(SearchingNew);
        },

        IdleStates::Stay => {}
    }
}