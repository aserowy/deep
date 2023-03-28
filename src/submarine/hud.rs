use bevy::{
    pbr::NotShadowCaster,
    prelude::{shape::Circle, *},
};
use bevy_rapier3d::prelude::Velocity;

use crate::render::line::{LineMaterial, LineStrip};

use super::{controller::ForwardThrustChangedEvent, PlayerSubmarineResource};

#[derive(Default, Component)]
pub struct VelocityUiComponent {}

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
            let line_material = line_materials.add(LineMaterial {
                color: Color::WHITE.into(),
            });

            parent.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(LineStrip::from(Circle::new(0.002)).into()),
                    material: line_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                NotShadowCaster,
            ));

            parent.spawn((
                MaterialMeshBundle {
                    // TODO: calculate circle size from options.movement_spot (0.075 fits an 125 spot)
                    mesh: meshes.add(LineStrip::from(Circle::new(0.075)).into()),
                    material: line_material,
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                NotShadowCaster,
            ));
        });
    }

    let font = asset_server.load("fonts/monofur.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        gap: Size::width(Val::Px(175.0)),
                        margin: UiRect::new(
                            Val::Px(0.0),
                            Val::Px(40.0),
                            Val::Px(250.0),
                            Val::Px(0.0),
                        ),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                gap: Size::height(Val::Px(5.0)),
                                size: Size::all(Val::Px(100.0)),
                                align_content: AlignContent::FlexEnd,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "24,5",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    TextSection::new(
                                        " m/s",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                ])
                                .with_style(Style {
                                    align_self: AlignSelf::FlexEnd,
                                    margin: UiRect::right(Val::Px(12.0)),
                                    ..default()
                                }),
                                VelocityUiComponent::default(),
                            ));

                            builder.spawn((
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    TextSection::new(
                                        "/",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    TextSection::new(
                                        "",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    TextSection::new(
                                        " kN",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 15.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                ])
                                .with_style(Style {
                                    align_self: AlignSelf::FlexEnd,
                                    ..default()
                                }),
                                ThrustUiComponent::default(),
                            ));
                        });

                    builder.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            size: Size::all(Val::Px(100.0)),
                            align_content: AlignContent::FlexEnd,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn update_velocity(
    mut ui_query: Query<&mut Text, With<VelocityUiComponent>>,
    mut velocity_query: Query<&Velocity, With<Camera>>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        if let Ok(velocity) = velocity_query.get_single_mut() {
            text.sections[0].value = format!("{:.2}", velocity.linvel.length());
        }
    }
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
