use bevy::prelude::*;

pub fn ui_init(mut commands: Commands) {

    let container = (
        Node {
            position_type: PositionType::Absolute,

            width: Val::Percent(40.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,

            ..Default::default()
        },

        BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.4))
    );

    let container_id = commands.spawn(container).id();

    commands.entity(container_id).with_children(| parent | {



    });

}