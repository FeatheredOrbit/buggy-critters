use bevy::{color::palettes::css::RED, prelude::*};

use crate::{constants::DEBUG, bug_entity::components::{debug_components::*, render_components::*, attribute_components::*}};

pub fn draw_sight_radius(query: Query<(&Transform, &PhysicalTraits), (With<DrawSightRadius>, With<BugEntityRoot>)>, mut gizmo: Gizmos) {
    if DEBUG {
        for (transform, physical_traits) in &query {
        
            gizmo.circle_2d(Vec2 { x: (transform.translation.x), y: (transform.translation.y) }, physical_traits.sight , RED);
        }
    }   
}