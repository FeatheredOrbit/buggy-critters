use bevy::prelude::*;

use crate::{entity::components::ui_components::{CurrentStateText, EntityPanelRoot}, resources::CurrentlySelectedEntity};

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

        Visibility::Visible,

        BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.4))
    ))

    .with_children(| parent | {

        parent.spawn((
            Node {
                height: Val::Px(30.0),
                width: Val::Auto,

                ..Default::default()
            },

            BorderRadius::all(Val::Percent(20.0)),
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

    });

}

pub fn ui_dislay
(
    mut panel_visibility: Single<&mut Visibility, With<EntityPanelRoot>>,
    selected_entity: Res<CurrentlySelectedEntity>
) 
{

    if let Some(entity) = selected_entity.0 {

        

    }

}