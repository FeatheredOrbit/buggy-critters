use bevy::prelude::*;
use crate::{bug_entity::components::{attribute_components::*, debug_components::*, idle_components::*, moving_components::*, render_components::*, shared_components::*, utils_components::*}, constants::{AMOUNT_OF_ENTITIES, CHUNKY_BODY_ATLAS_INDEX, CHUNKY_HEAD_ATLAS_INDEX, CURVED_LEGS_ATLAS_INDEX}, materials::renderer::resources::RendererHandle};

pub fn spawn
(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    renderer_handle: Res<RendererHandle>
) 
{
    let mesh = Mesh::from(Rectangle::new(120.0, 120.0));

    let mesh_handle = meshes.add(mesh);
        
    for i in 0..AMOUNT_OF_ENTITIES {
        let entity = commands.spawn(()).id();
        
        // Its transform component
        commands.entity(entity).insert((
            Transform::from_xyz(500.0, 500.0, -(i as f32)),
            GlobalTransform::default()
        ));

        // Identifier for the parent
        commands.entity(entity).insert(BugEntityRoot);

        // Atrribute components
        let physical_traits = PhysicalTraits::new();
        let vitals = Vitals::new(&physical_traits);

        commands.entity(entity).insert((
            physical_traits,
            vitals
        ));
    
        // Component for handling what state to move to and the current state
        commands.entity(entity).insert((
            NextState(crate::bug_entity::components::shared_components::States::None),
            CurrentState(crate::bug_entity::components::shared_components::States::Idle)
        ));

        // Bundle for the idle state
        commands.entity(entity).insert(IdleStateBundle::default());

        // Initialize components for moving states
        commands.entity(entity).insert((
            CurrentlyRotating(true),
            CurrentlyMoving(false)
        ));

        // Initialize components for searching states and moving states
        commands.entity(entity).insert((
            MovementPattern(MovementPatterns::Smooth),
            FutureTransform{position: Vec3::default(), angle: Quat::default()}
        ));

        // Components that holds the index of the body parts on the sprite atlas
        commands.entity(entity).insert (BodyPartsIndexes {
            head: CHUNKY_HEAD_ATLAS_INDEX,
            body: CHUNKY_BODY_ATLAS_INDEX,
            legs: CURVED_LEGS_ATLAS_INDEX
        });

        // Debug components
        commands.entity(entity).insert(DrawSightRadius);

        // Utils components
        commands.entity(entity).insert((
            PreviousTransform(Vec2 { x: (0.0), y: (0.0) }),
            Velocity(Vec2 { x: (0.0), y: (0.0) })
        ));

        // Rendering components
        commands.entity(entity).insert((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(renderer_handle.0.clone()),
            Visibility::Visible
        ));

    }
}
