use bevy::prelude::*;

use crate::bug_entity::components::{attribute_components::{Dehydrated, PhysicalTraits, Starving, Vitals}, render_components::BugEntityRoot, shared_components::{CurrentState, States}};

pub fn hunger_handler
(
    mut query: Query<(Entity, &mut Vitals, &PhysicalTraits, &CurrentState), With<BugEntityRoot>>,
    time: Res<Time>,
    mut commands: Commands
) 
{
    for (entity, mut vitals, physical_traits, current_state) in &mut query {

        let mut hunger_pay: f32 = 1.5;

        if matches!(current_state.0, States::MovingFood | States::MovingNew) {
            hunger_pay *= 2.0;
        }

        vitals.hunger -= hunger_pay * physical_traits.metabolism * time.delta_secs();

        if vitals.hunger <= vitals.starvation_threshold {
            commands.entity(entity).insert(Starving);
        } else {
            commands.entity(entity).remove::<Starving>();
        }

        if vitals.hunger <= 0.0 {
            vitals.hunger = 0.0;
        }

    } 
}

pub fn thirst_handler
(
    mut query: Query<(Entity, &mut Vitals, &PhysicalTraits, &CurrentState), With<BugEntityRoot>>,
    time: Res<Time>,
    mut commands: Commands
)
{
    for (entity, mut vitals, physical_traits, current_state) in &mut query {

        let mut thirst_pay: f32 = 1.5;

        if matches!(current_state.0, States::MovingFood | States::MovingNew) {
            thirst_pay *= 2.0;
        }

        vitals.thirst -= thirst_pay * physical_traits.metabolism * time.delta_secs();

        if vitals.thirst <= vitals.dehydration_threshold {
            commands.entity(entity).insert(Dehydrated);
        } else {
            commands.entity(entity).remove::<Dehydrated>();
        }

        if vitals.thirst <= 0.0 {
            vitals.thirst = 0.0;
        }

    } 
}

