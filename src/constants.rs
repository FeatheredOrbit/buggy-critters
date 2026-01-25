use crate::bug_entity::components::idle_components::{IdleBehaviour, IdleStates};

pub const IDLE_BEHAVIOURS: [IdleBehaviour; 3] = [
    IdleBehaviour{name: IdleStates::Move, weight: 3},
    IdleBehaviour{name: IdleStates::Stay, weight: 7},
    IdleBehaviour{name: IdleStates::SearchFood, weight: 2}
];

pub const IDLE_ACTION_TIMER: f32 = 1.0;

pub const GRID_CELL_WIDTH: f32 = 1000.0;
pub const GRID_CELL_HEIGHT: f32 = 1000.0;

pub const DEBUG: bool = false;

pub const CHUNKY_HEAD_ATLAS_INDEX: u32 = 0;
pub const CHUNKY_BODY_ATLAS_INDEX: u32 = 1;
pub const CURVED_LEGS_ATLAS_INDEX: u32 = 2;

pub const AMOUNT_OF_ENTITIES: usize = 30;