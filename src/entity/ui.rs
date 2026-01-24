use bevy::prelude::*;

use crate::{entity::components::{attribute_components::{PhysicalTraits, Vitals}, shared_components::{CurrentState, States}, ui_components::{CurrentStateText, EntityPanelRoot, HungerText, ThirstText}}, resources::CurrentlySelectedEntity};

pub fn ui_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/VT323.otf");

    commands.spawn((
        EntityPanelRoot,

        Node {
            position_type: PositionType::Absolute,

            width: Val::Percent(30.0),
            height: Val::Percent(50.0),
            justify_content: JustifyContent::Center,

            display: Display::Flex,

            ..Default::default()
        },

        Visibility::Hidden,

        BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.4))
    ))

    .with_children(| parent | {
        /////////////////////////////////////
        // Current state UI text
        ////////////////////////////////////
        parent.spawn((
            Node {
                height: Val::Px(30.0),
                width: Val::Auto,

                position_type: PositionType::Absolute,

                top: Val::Percent(0.0),
                left: Val::Percent(0.0),

                border_radius: BorderRadius::all(Val::Percent(20.0)),
                ..Default::default()
            },
            
            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),

            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
            
        ))

        .with_children(| row | {

            row.spawn((
                Text::new("Current state: "),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

            row.spawn((
                CurrentStateText,

                Text::new(""),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

        });

        /////////////////////////////////////
        // Hunger UI text
        ////////////////////////////////////
        parent.spawn((
            Node {
                height: Val::Px(30.0),
                width: Val::Auto,

                position_type: PositionType::Absolute,

                top: Val::Percent(10.0),
                left: Val::Percent(0.0),
                border_radius: BorderRadius::all(Val::Percent(20.0)),

                ..Default::default()
            },

            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),

            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
        ))

        .with_children(|row| {
            row.spawn((
                Text::new("Hunger: "),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

            row.spawn((
                HungerText,

                Text::new(""),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

            
        });

        /////////////////////////////////////
        // Thirst UI text
        ////////////////////////////////////
        parent.spawn((
            Node {
                height: Val::Px(30.0),
                width: Val::Auto,

                position_type: PositionType::Absolute,

                top: Val::Percent(20.0),
                left: Val::Percent(0.0),

                border_radius: BorderRadius::all(Val::Percent(20.0)),

                ..Default::default()
            },

            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),

            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
        ))

        .with_children(|row| {
            row.spawn((
                Text::new("Thirst: "),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

            row.spawn((
                ThirstText,

                Text::new(""),
                TextColor::WHITE, 
                TextLayout::new_with_justify(Justify::Left),
                TextFont {
                    font: font.clone(), 

                    ..Default::default()
                },
            ));

            
        });

    });

}

pub fn ui_display
(
    mut panel_visibility: Single<&mut Visibility, With<EntityPanelRoot>>,
    mut text_widgets: ParamSet<(
        Single<&mut Text, With<CurrentStateText>>,
        Single<&mut Text, With<HungerText>>,
        Single<&mut Text, With<ThirstText>>
    )>,
    selected_entity: Res<CurrentlySelectedEntity>,
    component_query: Query<(&PhysicalTraits, &CurrentState, &Vitals)>
) 
{

    if let Some(entity) = selected_entity.0 {

        if let Ok((_physical_traits, current_state, vitals)) = component_query.get(entity) {

            // Figuring out current state
            let current_state_info: &str;

            match current_state.0 {
                States::Idle => { current_state_info = "Idling" },
                States::SearchingNew => { current_state_info = "Searching for new location" },
                States::SearchingFood => { current_state_info = "Searching for food" },
                States::MovingNew => { current_state_info = "Moving towards a new location" },
                States::MovingFood => { current_state_info = "Moving towards food" },

                States::None => { current_state_info = "Doing fuckass nothing" }
            }

            // Assigning current value to its text widget
            text_widgets.p0().as_mut().0 = current_state_info.to_string();

            // Assigning current hunger value to its text widget, rounded for simplicity
            text_widgets.p1().as_mut().0 = vitals.hunger.round().to_string();

            // Assigning current thirst value to its text widget, rounded for simplicity
            text_widgets.p2().as_mut().0 = vitals.thirst.round().to_string();

        }

        *panel_visibility.as_mut() = Visibility::Visible;

    }

    else { *panel_visibility.as_mut() = Visibility::Hidden }

}