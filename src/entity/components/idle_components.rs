use bevy::prelude::Component;

const ACTION_TIMER: f32 = 1.0;

#[derive(Component)]
pub struct ActionTimer(pub f32);

impl ActionTimer {
    pub fn new() -> Self {
        Self (ACTION_TIMER)
    }
}

#[derive(Component)]
pub struct TimeToAction(pub f32);

impl TimeToAction {
    pub fn new() -> Self {
        Self (ACTION_TIMER)
    }
}

pub enum IdleStates {
    Move,
    Stay
}

pub struct IdleBehaviour {
    pub name: IdleStates,
    pub weight: i32

}

#[derive(Component)]
pub struct IdleBehaviours(pub Vec<IdleBehaviour>);
