use bevy::prelude::Component;

use crate::constants::{IDLE_ACTION_TIMER, IDLE_BEHAVIOURS};

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
    pub idle_behaviours: Vec<IdleBehaviour>
}

impl IdleStateBundle {
    pub fn default() -> Self {
        Self {
            action_timer: IDLE_ACTION_TIMER,
            time_to_action: IDLE_ACTION_TIMER,
            idle_behaviours: Vec::from(IDLE_BEHAVIOURS)
        }
    }
}


