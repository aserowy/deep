use bevy::prelude::*;

pub fn setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("submarine/crosshair_01.png"),
                    ..default()
                },
                style: Style {
                    size: Size::all(Val::Px(300.0)),
                    ..default()
                },
                ..default()
            });
        });
}
