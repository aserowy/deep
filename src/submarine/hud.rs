use bevy::prelude::{shape::Circle, *};

use crate::render::line::{LineMaterial, LineStrip};

use super::{
    controller::ForwardThrustChangedEvent,
    PlayerSubmarineResource,
};

#[derive(Default, Component)]
pub struct ThrustUiComponent {}

pub fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    player: Res<PlayerSubmarineResource>,
) {
    if !player.enabled {
        return;
    }

    if let Some(entity) = player.entity {
        commands.entity(entity).with_children(|parent| {
            // hud
            parent.spawn(MaterialMeshBundle {
                mesh: meshes.add(LineStrip::from(Circle::new(0.002)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });

            parent.spawn(MaterialMeshBundle {
                // TODO: calculate circle size from options.movement_spot (0.075 fits an 125 spot)
                mesh: meshes.add(LineStrip::from(Circle::new(0.075)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
    }

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/monofur.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "/",
                TextStyle {
                    font: asset_server.load("fonts/monofur.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/monofur.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position: UiRect {
                left: Val::Px(100.0),
                top: Val::Px(100.0),
                ..default()
            },
            ..default()
        }),
        ThrustUiComponent::default(),
    ));
}

pub fn update_on_forward_thrust_changed_event(
    mut forward_thrust_event_reader: EventReader<ForwardThrustChangedEvent>,
    mut query: Query<&mut Text, With<ThrustUiComponent>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        for event in forward_thrust_event_reader.iter() {
            text.sections[0].value = format!("{:.0}", event.0.forward_thrust);
            text.sections[2].value = format!("{:.0}", event.0.forward_thrust_max);
        }
    }
}
