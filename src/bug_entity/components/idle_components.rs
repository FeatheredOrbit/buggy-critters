use bevy::prelude::Component;
use rand::Rng;

use crate::{bug_entity::components::shared_components::RngComponent, constants::IDLE_ACTION_TIMER};

pub enum IdleStates {
    Move,
    Stay,
    SearchFood
}

pub struct IdleBehaviour {
    pub name: IdleStates,
    pub weight: i32

}

#[derive(Component)]
pub struct IdleStateBundle {
    pub action_timer: f32,
    pub time_to_action: f32,
    pub idle_behaviours: Vec<IdleBehaviour>,
    pub idle_behaviours_cumulative_weight: i32
}
impl IdleStateBundle {
    pub fn default(rng: &mut RngComponent) -> Self {
        let mut behaviours: Vec<IdleBehaviour> = Vec::with_capacity(3);
        let weight_move = rng.0.random_range(1..=3);
        let weight_stay = rng.0.random_range(4..=7);
        let weight_search_food = rng.0.random_range(1..=2);

        behaviours.push(IdleBehaviour { name: IdleStates::Move, weight: weight_move });
        behaviours.push(IdleBehaviour { name: IdleStates::Stay, weight: weight_stay });
        behaviours.push(IdleBehaviour { name: IdleStates::SearchFood, weight: weight_search_food });

        Self {
            action_timer: IDLE_ACTION_TIMER,
            time_to_action: IDLE_ACTION_TIMER,
            idle_behaviours: behaviours,
            idle_behaviours_cumulative_weight: weight_move + weight_search_food + weight_stay
        }
    }
}


