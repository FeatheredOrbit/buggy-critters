use bevy::prelude::Component;

#[derive(Component)]
pub struct Fruit;

#[derive(Component)]
pub struct NutritionalValue(pub f32);

#[derive(Component)]
pub struct BeingEaten;