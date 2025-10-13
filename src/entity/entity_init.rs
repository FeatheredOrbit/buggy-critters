use bevy::prelude::*;
use crate::entity::components::{shared_components::*, idle_components::*, render_components::*};


pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    

    let entity = commands.spawn(
        (

            // Its transform component
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            
            // Identifier
            EntityRoot,

            // The behaviours to choose from when idling
            IdleBehaviours(vec![
                IdleBehaviour{name: IdleStates::Move, weight: 3},
                IdleBehaviour{name: IdleStates::Stay, weight: 7}
            ]),
            
            // Starting state
            Idle,

            // Initialize variables for idle state
            TimeToAction::new(),
            ActionTimer::new()
        )
    ).id();

    spawn_render(&mut commands, &entity, &asset_server);

}

fn spawn_render(commands: &mut Commands, entity: &Entity, asset_server: &Res<AssetServer>) {

    let default_head: Handle<Image> = asset_server.load("bugs/body_parts/heads/chunky.png");
    let default_body: Handle<Image> = asset_server.load("bugs/body_parts/bodies/chunky.png");
    let default_legs: Handle<Image> = asset_server.load("bugs/body_parts/legs/curved.png");

    commands.entity(*entity).with_children(|parent| {

        parent.spawn((

            // Identifiers
            Head,
            EntityPart,

            SpriteBundle {
                texture: default_head,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            }

        ));

        parent.spawn((

            // Identifiers
            Body,
            EntityPart,

            SpriteBundle {
                texture: default_body,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            }

        ));

        parent.spawn((

            // Identifiers
            Legs,
            EntityPart,

            SpriteBundle {
                texture: default_legs,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            }

        ));

    });
}