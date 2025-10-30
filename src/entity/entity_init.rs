use bevy::{prelude::*};
use crate::entity::{
    components::{shared_components::*, idle_components::*, render_components::*, moving_components::*, debug_components::*, utils_components::*},
    
};

use crate::materials::entity_materials::*;


pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<FuzzMaterial>>, meshes: ResMut<Assets<Mesh>>) {
    let entity = commands.spawn(())
        
    // Its transform component
    .insert((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InheritedVisibility::default()
    ))

    // Identifier for the parent
    .insert(EntityRoot)

    // Physical traits
    .insert(PhysicalTraits::new())
    
    // Component for handling what state to move to and the current state
    .insert((
        NextState(crate::entity::components::shared_components::States::None),
        CurrentState(crate::entity::components::shared_components::States::Idle)
    ))

    // Starting state
    .insert((
        Action,
        Idle
    ))

    // Initialize components for idle state
    .insert((
        TimeToAction::new(),
        ActionTimer::new(),

        IdleBehaviours(vec![
            IdleBehaviour{name: IdleStates::Move, weight: 3},
            IdleBehaviour{name: IdleStates::Stay, weight: 7},
            IdleBehaviour{name: IdleStates::SearchFood, weight: 2}
        ])
    ))

    // Initialize components for moving states
    .insert((
        CurrentlyRotating(true),
        CurrentlyMoving(false)
    ))

    // Initialize components for searching states and moving states
    .insert((
        MovementPattern(MovementPatterns::Smooth),
        FutureTransform{position: Vec3::default(), angle: Quat::default()}
    ))

    // Debug components
    .insert(DrawSightRadius)

    // Utils components
    .insert((
        PreviousTransform(Vec2 { x: (0.0), y: (0.0) }),
        Velocity(Vec2 { x: (0.0), y: (0.0) })
    ))

    .id();

    spawn_render(&mut commands, &entity, &asset_server, materials, meshes);
}

fn spawn_render(commands: &mut Commands, entity: &Entity, asset_server: &Res<AssetServer>, mut materials: ResMut<Assets<FuzzMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let default_head: Handle<Image> = asset_server.load("art/bugs/body_parts/heads/chunky.png");
    let default_body: Handle<Image> = asset_server.load("art/bugs/body_parts/bodies/chunky.png");
    let default_legs: Handle<Image> = asset_server.load("art/bugs/body_parts/legs/curved.png");

    let noise: Handle<Image> = asset_server.load("art/other/noise_texture.png");

    commands.entity(*entity).with_children(|parent| {

        parent.spawn((

            // Identifiers
            Legs,
            EntityPart,
            Mesh2d(meshes.add(Rectangle::new(120.0, 120.0))),
            MeshMaterial2d(materials.add(FuzzMaterial {
                material_color: LinearRgba::BLUE,
                main_tex: default_legs,
                noise_tex: noise.clone(),
                velocity: 0.0
            }))

        ));

        parent.spawn((

            // Identifiers
            Head,
            EntityPart,

            Mesh2d(meshes.add(Rectangle::new(120.0, 120.0))),
            MeshMaterial2d(materials.add(FuzzMaterial {
                material_color: LinearRgba::BLUE,
                main_tex: default_head,
                noise_tex: noise.clone(),
                velocity: 0.0
            }))

        ));

        parent.spawn((

            // Identifiers
            Body,
            EntityPart,

            Mesh2d(meshes.add(Rectangle::new(120.0, 120.0))),
            MeshMaterial2d(materials.add(FuzzMaterial {
                material_color: LinearRgba::BLUE,
                main_tex: default_body,
                noise_tex: noise.clone(),
                velocity: 0.0
            }))

        ));

    });
}